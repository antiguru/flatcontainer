use std::marker::PhantomData;

/// A container for data.
pub trait Container: Default {
    /// The type of the data that one gets out of the container.
    type ReadItem<'a>
    where
        Self: 'a;

    /// The type to index into the container. Should be treated
    /// as an opaque type, even if known.
    type Index: Copy;

    /// Index into the container. The index must be obtained by
    /// pushing data into the container.
    fn index(&self, index: Self::Index) -> Self::ReadItem<'_>;

    /// The length of the container. 0 if unspecified.
    fn len(&self) -> usize;

    /// Test whether the container is empty. True if length is not specified.
    ///
    /// Must return `true` if and only if [`Self::len`] returns 0.
    fn is_empty(&self) -> bool;
}

/// A trait to let types express a default container type.
pub trait Containerized {
    /// The recommended container type.
    type Container: Container;
}

/// Push a type into a container.
pub trait CopyOnto<C: Container> {
    /// Copy self into the target container, returning an index that allows to
    /// look up the corresponding read item.
    fn copy_onto(self, target: &mut C) -> C::Index;
}

/// A container for [`Copy`] types.
#[derive(Debug)]
pub struct CopyContainer<T: Copy> {
    offsets: Vec<usize>,
    slices: Vec<T>,
}

impl<T: Copy> Container for CopyContainer<T> {
    type ReadItem<'a> = &'a [T] where Self: 'a;
    type Index = usize;

    fn index(&self, index: Self::Index) -> Self::ReadItem<'_> {
        &self.slices[self.offsets[index]..self.offsets[index + 1]]
    }

    fn len(&self) -> usize {
        self.offsets.len()
    }

    fn is_empty(&self) -> bool {
        self.offsets.is_empty()
    }
}

impl<T: Copy> Default for CopyContainer<T> {
    fn default() -> Self {
        Self {
            offsets: vec![0],
            slices: Vec::default(),
        }
    }
}

impl<'a, T> CopyOnto<CopyContainer<T>> for &'a [T]
where
    T: Copy,
{
    fn copy_onto(self, target: &mut CopyContainer<T>) -> usize {
        target.slices.extend(self);
        target.offsets.push(target.slices.len());
        target.offsets.len() - 2
    }
}

/// A container representing slices of data.
#[derive(Debug)]
pub struct SliceContainer<C: Container> {
    offsets: Vec<usize>,
    slices: Vec<C::Index>,
    inner: C,
}

impl<C: Container> Container for SliceContainer<C> {
    type ReadItem<'a> = (&'a C, &'a [C::Index]) where Self: 'a;
    type Index = usize;

    fn index(&self, index: Self::Index) -> Self::ReadItem<'_> {
        let slice = &self.slices[self.offsets[index]..self.offsets[index + 1]];
        (&self.inner, slice)
    }

    fn len(&self) -> usize {
        self.offsets.len()
    }

    fn is_empty(&self) -> bool {
        self.offsets.is_empty()
    }
}

impl<C: Container> Default for SliceContainer<C> {
    fn default() -> Self {
        Self {
            offsets: vec![0],
            slices: Vec::default(),
            inner: C::default(),
        }
    }
}

impl<'a, C, T> CopyOnto<SliceContainer<C>> for &'a [T]
where
    C: Container,
    &'a T: CopyOnto<C>,
{
    fn copy_onto(self, target: &mut SliceContainer<C>) -> usize {
        target
            .slices
            .extend(self.iter().map(|t| t.copy_onto(&mut target.inner)));
        target.offsets.push(target.slices.len());
        target.offsets.len() - 2
    }
}

impl<'a, C, T> CopyOnto<SliceContainer<C>> for &'a Vec<T>
where
    C: Container,
    &'a [T]: CopyOnto<SliceContainer<C>>,
{
    fn copy_onto(self, target: &mut SliceContainer<C>) -> usize {
        self.as_slice().copy_onto(target)
    }
}

impl<'a, C: Container> CopyOnto<SliceContainer<C>> for &'a (&'a C, &'a [C::Index])
where
    C::ReadItem<'a>: CopyOnto<C>,
{
    fn copy_onto(self, target: &mut SliceContainer<C>) -> <SliceContainer<C> as Container>::Index {
        let (container, indexes) = self;
        target.slices.extend(
            indexes
                .iter()
                .map(|&index| container.index(index).copy_onto(&mut target.inner)),
        );
        target.offsets.push(target.slices.len());
        target.offsets.len() - 2
    }
}

impl<T: Containerized> Containerized for Vec<T> {
    type Container = SliceContainer<T::Container>;
}

impl<T: Containerized> Containerized for [T] {
    type Container = SliceContainer<T::Container>;
}

/// A container to store strings and read `&str`.
#[derive(Default, Debug)]
pub struct StringContainer {
    inner: SliceContainer<MirrorContainer<u8>>,
}

impl Container for StringContainer {
    type ReadItem<'a>= &'a str where Self: 'a ;
    type Index = usize;

    fn index(&self, index: Self::Index) -> Self::ReadItem<'_> {
        unsafe { std::str::from_utf8_unchecked(self.inner.index(index).1) }
    }

    fn len(&self) -> usize {
        self.inner.len()
    }

    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl Containerized for String {
    type Container = StringContainer;
}

impl<'a> CopyOnto<StringContainer> for &'a String {
    fn copy_onto(self, target: &mut StringContainer) -> usize {
        self.as_str().copy_onto(target)
    }
}

impl<'a> CopyOnto<StringContainer> for &'a str {
    fn copy_onto(self, target: &mut StringContainer) -> usize {
        self.as_bytes().copy_onto(&mut target.inner)
    }
}

/// A container for types where the read item type is equal to the index type.
#[derive(Debug)]
pub struct MirrorContainer<T>(PhantomData<*const T>);

impl<T: Copy> Container for MirrorContainer<T> {
    type ReadItem<'a> = T where T: 'a;
    type Index = T;

    fn index(&self, index: Self::Index) -> Self::ReadItem<'_> {
        index
    }

    fn len(&self) -> usize {
        0
    }

    fn is_empty(&self) -> bool {
        true
    }
}

impl<T> Default for MirrorContainer<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

mod implementations {
    use super::*;

    macro_rules! implement_for {
        ($index_type:ty) => {
            impl Containerized for $index_type {
                type Container = MirrorContainer<Self>;
            }

            impl CopyOnto<MirrorContainer<Self>> for $index_type {
                fn copy_onto(self, _target: &mut MirrorContainer<Self>) -> $index_type {
                    self
                }
            }

            impl<'a> CopyOnto<MirrorContainer<$index_type>> for &'a $index_type {
                fn copy_onto(self, _target: &mut MirrorContainer<$index_type>) -> $index_type {
                    *self
                }
            }
        };
    }

    implement_for!(());
    implement_for!(bool);
    implement_for!(char);

    implement_for!(u8);
    implement_for!(u16);
    implement_for!(u32);
    implement_for!(u64);
    implement_for!(u128);
    implement_for!(usize);

    implement_for!(i8);
    implement_for!(i16);
    implement_for!(i32);
    implement_for!(i64);
    implement_for!(i128);
    implement_for!(isize);

    implement_for!(f32);
    implement_for!(f64);

    implement_for!(std::num::Wrapping<i8>);
    implement_for!(std::num::Wrapping<i16>);
    implement_for!(std::num::Wrapping<i32>);
    implement_for!(std::num::Wrapping<i64>);
    implement_for!(std::num::Wrapping<i128>);
    implement_for!(std::num::Wrapping<isize>);

    implement_for!(std::time::Duration);
}

pub use result::ResultContainer;

mod result {
    use super::*;

    #[derive(Default)]
    pub struct ResultContainer<T, E> {
        oks: T,
        errs: E,
    }

    impl<T, E> Container for ResultContainer<T, E>
    where
        T: Container,
        E: Container,
    {
        type ReadItem<'a> = Result<T::ReadItem<'a>, E::ReadItem<'a>> where Self: 'a;
        type Index = Result<T::Index, E::Index>;

        fn index(&self, index: Self::Index) -> Self::ReadItem<'_> {
            match index {
                Ok(index) => Ok(self.oks.index(index)),
                Err(index) => Err(self.errs.index(index)),
            }
        }

        fn len(&self) -> usize {
            self.oks.len() + self.errs.len()
        }

        fn is_empty(&self) -> bool {
            self.oks.is_empty() && self.errs.is_empty()
        }
    }

    impl<T, TC, E, EC> CopyOnto<ResultContainer<TC, EC>> for Result<T, E>
    where
        TC: Container,
        EC: Container,
        T: CopyOnto<TC>,
        E: CopyOnto<EC>,
    {
        fn copy_onto(
            self,
            target: &mut ResultContainer<TC, EC>,
        ) -> <ResultContainer<TC, EC> as Container>::Index {
            match self {
                Ok(t) => Ok(t.copy_onto(&mut target.oks)),
                Err(e) => Err(e.copy_onto(&mut target.errs)),
            }
        }
    }

    impl<'a, T, TC, E, EC> CopyOnto<ResultContainer<TC, EC>> for &'a Result<T, E>
    where
        TC: Container,
        EC: Container,
        &'a T: CopyOnto<TC>,
        &'a E: CopyOnto<EC>,
    {
        fn copy_onto(
            self,
            target: &mut ResultContainer<TC, EC>,
        ) -> <ResultContainer<TC, EC> as Container>::Index {
            match self {
                Ok(t) => Ok(t.copy_onto(&mut target.oks)),
                Err(e) => Err(e.copy_onto(&mut target.errs)),
            }
        }
    }

    impl<T: Containerized, E: Containerized> Containerized for Result<T, E> {
        type Container = ResultContainer<T::Container, E::Container>;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slice_string_onto() {
        let mut c = StringContainer::default();
        let index = "abc".to_string().copy_onto(&mut c);
        assert_eq!("abc", c.index(index));
        let index = "def".copy_onto(&mut c);
        assert_eq!("def", c.index(index));
    }

    #[test]
    fn test_vec() {
        let mut c = SliceContainer::default();
        let slice = &[1u8, 2, 3];
        let idx = slice.copy_onto(&mut c);
        assert_eq!(slice, c.index(idx).1)
    }

    #[test]
    fn test_vec_onto() {
        let mut c: SliceContainer<MirrorContainer<u8>> = SliceContainer::default();
        let slice = &[1u8, 2, 3][..];
        let idx = slice.copy_onto(&mut c);
        assert_eq!(slice, c.index(idx).1)
    }

    struct Person {
        name: String,
        age: u16,
        hobbies: Vec<String>,
    }

    impl Containerized for Person {
        type Container = PersonContainer;
    }

    #[derive(Default)]
    struct PersonContainer {
        index: Vec<(
            <<String as Containerized>::Container as Container>::Index,
            <<u16 as Containerized>::Container as Container>::Index,
            <<Vec<String> as Containerized>::Container as Container>::Index,
        )>,
        name_container: <String as Containerized>::Container,
        age_container: <u16 as Containerized>::Container,
        hobbies: <Vec<String> as Containerized>::Container,
    }

    #[derive(Debug)]
    struct PersonRef<'a> {
        name: <<String as Containerized>::Container as Container>::ReadItem<'a>,
        age: <<u16 as Containerized>::Container as Container>::ReadItem<'a>,
        hobbies: <<Vec<String> as Containerized>::Container as Container>::ReadItem<'a>,
    }

    impl Container for PersonContainer {
        type ReadItem<'a> = PersonRef<'a> where Self: 'a;
        type Index = usize;

        fn index(&self, index: Self::Index) -> Self::ReadItem<'_> {
            let (name, age, hobbies) = self.index[index];
            PersonRef {
                name: self.name_container.index(name),
                age: self.age_container.index(age),
                hobbies: self.hobbies.index(hobbies),
            }
        }

        fn len(&self) -> usize {
            self.index.len()
        }

        fn is_empty(&self) -> bool {
            self.index.is_empty()
        }
    }

    impl CopyOnto<PersonContainer> for Person {
        fn copy_onto(self, target: &mut PersonContainer) -> usize {
            let name = self.name.copy_onto(&mut target.name_container);
            let age = self.age.copy_onto(&mut target.age_container);
            let hobbies = self.hobbies.copy_onto(&mut target.hobbies);
            target.index.push((name, age, hobbies));
            target.index.len() - 1
        }
    }

    impl<'a> CopyOnto<PersonContainer> for PersonRef<'a> {
        fn copy_onto(self, target: &mut PersonContainer) -> usize {
            let name = self.name.copy_onto(&mut target.name_container);
            let age = self.age.copy_onto(&mut target.age_container);
            let hobbies = self.hobbies.copy_onto(&mut target.hobbies);
            target.index.push((name, age, hobbies));
            target.index.len() - 1
        }
    }

    #[test]
    fn test_person() {
        let hobbies = ["Computers", "Guitar"];
        let p = Person {
            name: "Moritz".to_string(),
            age: 123,
            hobbies: hobbies.iter().map(ToString::to_string).collect(),
        };

        let mut c = <Person as Containerized>::Container::default();
        let idx = p.copy_onto(&mut c);
        let person_ref = c.index(idx);
        assert_eq!("Moritz", person_ref.name);
        assert_eq!(123, person_ref.age);
        assert_eq!(2, person_ref.hobbies.1.len());
        for (idx, hobby) in person_ref.hobbies.1.iter().zip(hobbies) {
            assert_eq!(hobby, person_ref.hobbies.0.index(*idx));
        }

        let mut cc = PersonContainer::default();

        let idx = c.index(idx).copy_onto(&mut cc);

        let person_ref = cc.index(idx);
        assert_eq!("Moritz", person_ref.name);
        assert_eq!(123, person_ref.age);
        assert_eq!(2, person_ref.hobbies.1.len());
        for (idx, hobby) in person_ref.hobbies.1.iter().zip(hobbies) {
            assert_eq!(hobby, person_ref.hobbies.0.index(*idx));
        }
    }

    #[test]
    fn test_result() {
        let r: Result<_, u16> = Ok("abc");
        let mut c = ResultContainer::default();
        let idx = r.copy_onto(&mut c);
        assert_eq!(r, c.index(idx));
    }
}
