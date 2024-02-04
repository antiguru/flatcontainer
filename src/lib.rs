#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

mod impls;

pub use impls::mirror::MirrorRegion;
pub use impls::result::ResultRegion;
pub use impls::slice::SliceRegion;
pub use impls::slice_copy::CopyRegion;
pub use impls::string::StringRegion;

#[cfg(feature = "serde")]
pub trait Index: Copy + Serialize + for<'a> Deserialize<'a> {}
#[cfg(feature = "serde")]
impl<T: Copy + Serialize + for<'a> Deserialize<'a>> Index for T {}

#[cfg(not(feature = "serde"))]
pub trait Index: Copy {}
#[cfg(not(feature = "serde"))]
impl<T: Copy> Index for T {}

/// A region to store data.
pub trait Region: Default {
    /// The type of the data that one gets out of the container.
    type ReadItem<'a>: CopyOnto<Self>
    where
        Self: 'a;

    /// The type to index into the container. Should be treated
    /// as an opaque type, even if known.
    type Index: Index;

    /// Index into the container. The index must be obtained by
    /// pushing data into the container.
    fn index(&self, index: Self::Index) -> Self::ReadItem<'_>;

    // Ensure that the region can absorb the items of `regions` without reallocation
    fn reserve_regions<'a, I>(&mut self, regions: I)
    where
        Self: 'a,
        I: Iterator<Item = &'a Self> + Clone;

    fn clear(&mut self);
}

/// A trait to let types express a default container type.
pub trait Containerized {
    /// The recommended container type.
    type Region: Region;
}

/// Push a type into a container.
pub trait CopyOnto<C: Region> {
    /// Copy self into the target container, returning an index that allows to
    /// look up the corresponding read item.
    fn copy_onto(self, target: &mut C) -> C::Index;
}

pub trait ReserveItems<R: Region> {
    /// Ensure that the region can absorb `items` without reallocation.
    fn reserve_items<I>(target: &mut R, items: I)
    where
        I: Iterator<Item = Self> + Clone;
}

impl<T: Containerized> Containerized for Vec<T> {
    type Region = SliceRegion<T::Region>;
}

impl<T: Containerized> Containerized for [T] {
    type Region = SliceRegion<T::Region>;
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(
        bound = "R: Serialize + for<'a> Deserialize<'a>, R::Index: Serialize + for<'a> Deserialize<'a>"
    )
)]
pub struct FlatStack<R: Region> {
    indices: Vec<R::Index>,
    region: R,
}

impl<R: Region> FlatStack<R> {
    #[inline]
    pub fn default_impl<T: Containerized<Region = R>>() -> Self {
        Self::default()
    }
}

impl<R: Region> Default for FlatStack<R> {
    #[inline]
    fn default() -> Self {
        Self {
            indices: Vec::default(),
            region: R::default(),
        }
    }
}

impl<R: Region> FlatStack<R> {
    #[inline]
    pub fn copy(&mut self, item: impl CopyOnto<R>) {
        let index = item.copy_onto(&mut self.region);
        self.indices.push(index);
    }

    #[inline]
    pub fn get(&self, offset: usize) -> R::ReadItem<'_> {
        self.region.index(self.indices[offset])
    }

    /// The length of the container. 0 if unspecified.
    #[inline]
    pub fn len(&self) -> usize {
        self.indices.len()
    }

    /// Test whether the container is empty. True if length is not specified.
    ///
    /// Must return `true` if and only if [`Self::len`] returns 0.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.indices.is_empty()
    }

    /// Remove all elements while possibly retaining allocations.
    pub fn clear(&mut self) {
        self.indices.clear();
        self.region.clear();
    }

    pub fn reserve_items<T>(&mut self, items: impl Iterator<Item = T> + Clone)
    where
        T: ReserveItems<R>,
    {
        ReserveItems::reserve_items(&mut self.region, items);
    }

    pub fn reserve_regions<'a>(&mut self, regions: impl Iterator<Item = &'a R> + Clone)
    where
        R: 'a,
    {
        self.region.reserve_regions(regions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slice_string_onto() {
        let mut c = StringRegion::default();
        let index = "abc".to_string().copy_onto(&mut c);
        assert_eq!("abc", c.index(index));
        let index = "def".copy_onto(&mut c);
        assert_eq!("def", c.index(index));
    }

    #[test]
    fn test_container_string() {
        let mut c = FlatStack::default_impl::<String>();
        c.copy(&"abc".to_string());
        assert_eq!("abc", c.get(0));
        c.copy("def");
        assert_eq!("def", c.get(1));
    }

    #[test]
    fn test_vec() {
        let mut c = SliceRegion::default();
        let slice = &[1u8, 2, 3];
        let idx = slice.copy_onto(&mut c);
        assert_eq!(slice, c.index(idx).1)
    }

    #[test]
    fn test_vec_onto() {
        let mut c: SliceRegion<MirrorRegion<u8>> = SliceRegion::default();
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
        type Region = PersonRegion;
    }

    #[derive(Default)]
    struct PersonRegion {
        name_container: <String as Containerized>::Region,
        age_container: <u16 as Containerized>::Region,
        hobbies: <Vec<String> as Containerized>::Region,
    }

    #[derive(Debug)]
    struct PersonRef<'a> {
        name: <<String as Containerized>::Region as Region>::ReadItem<'a>,
        age: <<u16 as Containerized>::Region as Region>::ReadItem<'a>,
        hobbies: <<Vec<String> as Containerized>::Region as Region>::ReadItem<'a>,
    }

    impl Region for PersonRegion {
        type ReadItem<'a> = PersonRef<'a> where Self: 'a;
        type Index = (
            <<String as Containerized>::Region as Region>::Index,
            <<u16 as Containerized>::Region as Region>::Index,
            <<Vec<String> as Containerized>::Region as Region>::Index,
        );

        fn index(&self, (name, age, hobbies): Self::Index) -> Self::ReadItem<'_> {
            PersonRef {
                name: self.name_container.index(name),
                age: self.age_container.index(age),
                hobbies: self.hobbies.index(hobbies),
            }
        }

        fn reserve_regions<'a, I>(&mut self, regions: I)
        where
            Self: 'a,
            I: Iterator<Item = &'a Self> + Clone,
        {
            self.name_container
                .reserve_regions(regions.clone().map(|r| &r.name_container));
            self.age_container
                .reserve_regions(regions.clone().map(|r| &r.age_container));
            self.hobbies
                .reserve_regions(regions.clone().map(|r| &r.hobbies));
        }

        fn clear(&mut self) {
            self.name_container.clear();
            self.age_container.clear();
            self.hobbies.clear();
        }
    }

    impl<'a> CopyOnto<PersonRegion> for &'a Person {
        fn copy_onto(self, target: &mut PersonRegion) -> <PersonRegion as Region>::Index {
            let name = self.name.copy_onto(&mut target.name_container);
            let age = self.age.copy_onto(&mut target.age_container);
            let hobbies = (&self.hobbies).copy_onto(&mut target.hobbies);
            (name, age, hobbies)
        }
    }

    impl<'a> ReserveItems<PersonRegion> for &'a Person {
        fn reserve_items<I>(target: &mut PersonRegion, items: I)
        where
            I: Iterator<Item = Self> + Clone,
        {
            ReserveItems::reserve_items(&mut target.name_container, items.clone().map(|i| &i.name));
            ReserveItems::reserve_items(&mut target.age_container, items.clone().map(|i| &i.age));
            ReserveItems::reserve_items(&mut target.hobbies, items.map(|i| &i.hobbies));
        }
    }

    impl<'a> CopyOnto<PersonRegion> for PersonRef<'a> {
        fn copy_onto(self, target: &mut PersonRegion) -> <PersonRegion as Region>::Index {
            let name = self.name.copy_onto(&mut target.name_container);
            let age = self.age.copy_onto(&mut target.age_container);
            let hobbies = self.hobbies.copy_onto(&mut target.hobbies);
            (name, age, hobbies)
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

        let mut c = FlatStack::default_impl::<Person>();
        c.copy(&p);
        let person_ref = c.get(0);
        assert_eq!("Moritz", person_ref.name);
        assert_eq!(123, person_ref.age);
        assert_eq!(2, person_ref.hobbies.1.len());
        for (idx, hobby) in person_ref.hobbies.1.iter().zip(hobbies) {
            assert_eq!(hobby, person_ref.hobbies.0.index(*idx));
        }

        let mut cc = FlatStack::default_impl::<Person>();

        cc.copy(c.get(0));

        let person_ref = cc.get(0);
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
        let mut c = ResultRegion::default();
        let idx = r.copy_onto(&mut c);
        assert_eq!(r, c.index(idx));
    }

    #[test]
    fn all_types() {
        fn test_copy<T, C: Region + Clone>(t: T)
        where
            T: CopyOnto<C>,
        {
            let mut c = FlatStack::default();
            c.copy(t);

            let mut cc = c.clone();
            cc.copy(c.get(0));
        }

        test_copy::<_, StringRegion>(&"a".to_string());
        test_copy::<_, StringRegion>("a");

        test_copy::<_, MirrorRegion<_>>(());
        test_copy::<_, MirrorRegion<_>>(&());
        test_copy::<_, MirrorRegion<_>>(true);
        test_copy::<_, MirrorRegion<_>>(&true);
        test_copy::<_, MirrorRegion<_>>(' ');
        test_copy::<_, MirrorRegion<_>>(&' ');
        test_copy::<_, MirrorRegion<_>>(0u8);
        test_copy::<_, MirrorRegion<_>>(&0u8);
        test_copy::<_, MirrorRegion<_>>(0u16);
        test_copy::<_, MirrorRegion<_>>(&0u16);
        test_copy::<_, MirrorRegion<_>>(0u32);
        test_copy::<_, MirrorRegion<_>>(&0u32);
        test_copy::<_, MirrorRegion<_>>(0u64);
        test_copy::<_, MirrorRegion<_>>(&0u64);
        test_copy::<_, MirrorRegion<_>>(0u128);
        test_copy::<_, MirrorRegion<_>>(&0u128);
        test_copy::<_, MirrorRegion<_>>(0usize);
        test_copy::<_, MirrorRegion<_>>(&0usize);
        test_copy::<_, MirrorRegion<_>>(0i8);
        test_copy::<_, MirrorRegion<_>>(&0i8);
        test_copy::<_, MirrorRegion<_>>(0i16);
        test_copy::<_, MirrorRegion<_>>(&0i16);
        test_copy::<_, MirrorRegion<_>>(0i32);
        test_copy::<_, MirrorRegion<_>>(&0i32);
        test_copy::<_, MirrorRegion<_>>(0i64);
        test_copy::<_, MirrorRegion<_>>(&0i64);
        test_copy::<_, MirrorRegion<_>>(0i128);
        test_copy::<_, MirrorRegion<_>>(&0i128);
        test_copy::<_, MirrorRegion<_>>(0isize);
        test_copy::<_, MirrorRegion<_>>(&0isize);
        test_copy::<_, MirrorRegion<_>>(0f32);
        test_copy::<_, MirrorRegion<_>>(&0f32);
        test_copy::<_, MirrorRegion<_>>(0f64);
        test_copy::<_, MirrorRegion<_>>(&0f64);
        test_copy::<_, MirrorRegion<_>>(std::num::Wrapping(0i8));
        test_copy::<_, MirrorRegion<_>>(&std::num::Wrapping(0i8));
        test_copy::<_, MirrorRegion<_>>(std::num::Wrapping(0i16));
        test_copy::<_, MirrorRegion<_>>(&std::num::Wrapping(0i16));
        test_copy::<_, MirrorRegion<_>>(std::num::Wrapping(0i32));
        test_copy::<_, MirrorRegion<_>>(&std::num::Wrapping(0i32));
        test_copy::<_, MirrorRegion<_>>(std::num::Wrapping(0i64));
        test_copy::<_, MirrorRegion<_>>(&std::num::Wrapping(0i64));
        test_copy::<_, MirrorRegion<_>>(std::num::Wrapping(0i128));
        test_copy::<_, MirrorRegion<_>>(&std::num::Wrapping(0i128));
        test_copy::<_, MirrorRegion<_>>(std::num::Wrapping(0isize));
        test_copy::<_, MirrorRegion<_>>(&std::num::Wrapping(0isize));

        test_copy::<_, ResultRegion<_, _>>(Result::<u8, u8>::Ok(0));
        test_copy::<_, ResultRegion<_, _>>(&Result::<u8, u8>::Ok(0));

        test_copy::<_, SliceRegion<_>>([0u8].as_slice());
        test_copy::<_, SliceRegion<_>>(vec![0u8]);
        test_copy::<_, SliceRegion<_>>(&vec![0u8]);

        test_copy::<_, SliceRegion<_>>(["a"].as_slice());
        test_copy::<_, SliceRegion<_>>(vec!["a"]);
        test_copy::<_, SliceRegion<_>>(&vec!["a"]);

        test_copy::<_, SliceRegion<_>>([("a",)].as_slice());
        test_copy::<_, SliceRegion<_>>(vec![("a",)]);
        test_copy::<_, SliceRegion<_>>(&vec![("a",)]);

        test_copy::<_, CopyRegion<_>>([0u8].as_slice());

        test_copy::<_, <(u8, u8) as Containerized>::Region>((1, 2));
    }

    #[test]
    fn slice_region_read_item() {
        let mut c = FlatStack::<SliceRegion<MirrorRegion<u8>>>::default();
        c.copy(vec![1, 2, 3]);

        let mut r = SliceRegion::<MirrorRegion<u8>>::default();
        let idx = [1, 2, 3].copy_onto(&mut r);
        let read_item = r.index(idx);
        let _read_item2 = read_item.clone();
        let _read_item3 = read_item;
        assert_eq!(vec![1, 2, 3], read_item.into_iter().collect::<Vec<_>>());
    }
}
