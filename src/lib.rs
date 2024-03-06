#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

use std::fmt::{Debug, Formatter};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub mod impls;

pub use impls::columns::ColumnsRegion;
pub use impls::mirror::MirrorRegion;
pub use impls::option::OptionRegion;
pub use impls::result::ResultRegion;
pub use impls::slice::SliceRegion;
pub use impls::slice_copy::CopyRegion;
pub use impls::string::StringRegion;

/// An index into a region. Automatically implemented for relevant types.
///
/// We require an index to be [`Copy`] and to support serde.
#[cfg(feature = "serde")]
pub trait Index: Copy + Serialize + for<'a> Deserialize<'a> {}
#[cfg(feature = "serde")]
impl<T: Copy + Serialize + for<'a> Deserialize<'a>> Index for T {}

/// An index into a region. Automatically implemented for relevant types.
///
/// We require an index to be [`Copy`].
#[cfg(not(feature = "serde"))]
pub trait Index: Copy {}
#[cfg(not(feature = "serde"))]
impl<T: Copy> Index for T {}

/// A region to absorb presented data and present it as a type with a lifetime.
///
/// This type absorbs data and provides an index to look up an equivalent representation
/// of this data at a later time. It is up to an implementation to select the appropriate
/// presentation of the data, and what data it can absorb.
///
/// Implement the [`CopyOnto`] trait for all types that can be copied into a region.
pub trait Region: Default {
    /// The type of the data that one gets out of the container.
    type ReadItem<'a>
    where
        Self: 'a;

    /// The type to index into the container. Should be treated
    /// as an opaque type, even if known.
    type Index: Index;

    /// Construct a region that can absorb the contents of `regions` in the future.
    fn merge_regions<'a>(regions: impl Iterator<Item = &'a Self> + Clone) -> Self
    where
        Self: 'a;

    /// Index into the container. The index must be obtained by
    /// pushing data into the container.
    fn index(&self, index: Self::Index) -> Self::ReadItem<'_>;

    /// Ensure that the region can absorb the items of `regions` without reallocation
    fn reserve_regions<'a, I>(&mut self, regions: I)
    where
        Self: 'a,
        I: Iterator<Item = &'a Self> + Clone;

    /// Remove all elements from this region, but retain allocations if possible.
    fn clear(&mut self);

    /// Heap size, size - capacity
    fn heap_size<F: FnMut(usize, usize)>(&self, callback: F);
}

/// A trait to let types express a default container type.
pub trait Containerized {
    /// The recommended container type.
    type Region: Region;
}

/// A type that can write its contents into a region.
pub trait CopyOnto<C: Region> {
    /// Copy self into the target container, returning an index that allows to
    /// look up the corresponding read item.
    fn copy_onto(self, target: &mut C) -> C::Index;
}

/// Reserve space in the receiving region.
///
/// Closely related to [`CopyOnto`], but separate because target type is likely different.
pub trait ReserveItems<R: Region> {
    /// Ensure that the region can absorb `items` without reallocation.
    fn reserve_items<I>(target: &mut R, items: I)
    where
        I: Iterator<Item = Self> + Clone;
}

/// A container for indices into a region.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(
        bound = "R: Serialize + for<'a> Deserialize<'a>, R::Index: Serialize + for<'a> Deserialize<'a>"
    )
)]
pub struct FlatStack<R: Region> {
    /// The indices, which we use to lookup items in the region.
    indices: Vec<R::Index>,
    /// A region to index into.
    region: R,
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

impl<R: Region> Debug for FlatStack<R>
where
    for<'a> R::ReadItem<'a>: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<R: Region> FlatStack<R> {
    /// Default implementation based on the preference of type `T`.
    #[inline]
    #[must_use]
    pub fn default_impl<T: Containerized<Region = R>>() -> Self {
        Self::default()
    }

    /// Default implementation based on the preference of type `T`.
    #[inline]
    #[must_use]
    pub fn default_val<T: Containerized<Region = R>>(_: &T) -> Self {
        Self::default()
    }

    /// Returns a flat stack that can absorb `capacity` indices without reallocation.
    ///
    /// Prefer [`Self::merge_capacity`] over this function to also pre-size the regions.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            indices: Vec::with_capacity(capacity),
            region: R::default(),
        }
    }

    /// Returns a flat stack that can absorb the contents of `iter` without reallocation.
    #[must_use]
    pub fn merge_capacity<'a, I: Iterator<Item = &'a Self> + Clone + 'a>(stacks: I) -> Self
    where
        R: 'a,
    {
        Self {
            indices: Vec::with_capacity(stacks.clone().map(|s| s.indices.len()).sum()),
            region: R::merge_regions(stacks.map(|r| &r.region)),
        }
    }

    /// Appends the element to the back of the stack.
    #[inline]
    pub fn copy(&mut self, item: impl CopyOnto<R>) {
        let index = item.copy_onto(&mut self.region);
        self.indices.push(index);
    }

    /// Returns the element at the `offset` position.
    #[inline]
    #[must_use]
    pub fn get(&self, offset: usize) -> R::ReadItem<'_> {
        self.region.index(self.indices[offset])
    }

    /// Returns the number of indices in the stack.
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.indices.len()
    }

    /// Returns `true` if the stack contains no elements.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.indices.is_empty()
    }

    /// Returns the total number of indices the stack can hold without reallocation.
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.indices.capacity()
    }

    /// Reserves space to hold `additional` indices.
    pub fn reserve(&mut self, additional: usize) {
        self.indices.reserve(additional);
    }

    /// Remove all elements while possibly retaining allocations.
    pub fn clear(&mut self) {
        self.indices.clear();
        self.region.clear();
    }

    /// Reserve space for the items returned by the iterator.
    pub fn reserve_items<T>(&mut self, items: impl Iterator<Item = T> + Clone)
    where
        T: ReserveItems<R>,
    {
        ReserveItems::reserve_items(&mut self.region, items);
    }

    /// Reserve space for the regions returned by the iterator.
    pub fn reserve_regions<'a>(&mut self, regions: impl Iterator<Item = &'a R> + Clone)
    where
        R: 'a,
    {
        self.region.reserve_regions(regions);
    }

    /// Iterate the items in this stack.
    pub fn iter(&self) -> Iter<'_, R> {
        self.into_iter()
    }

    /// Heap size, size - capacity
    pub fn heap_size<F: FnMut(usize, usize)>(&self, mut callback: F) {
        use crate::impls::offsets::OffsetContainer;
        self.region.heap_size(&mut callback);
        self.indices.heap_size(callback);
    }

    /// Convert the flat stack into its parts.
    pub fn into_parts(self) -> (R, Vec<R::Index>) {
        (self.region, self.indices)
    }

    /// Construct a flat stack from its parts.
    ///
    /// The method performs no validation, which means the parameters must
    /// be correct: The region must contain an element for each index.
    pub fn from_parts(region: R, indices: Vec<R::Index>) -> Self {
        Self { region, indices }
    }
}

impl<T: CopyOnto<R>, R: Region> Extend<T> for FlatStack<R> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        let iter = iter.into_iter();
        self.reserve(iter.size_hint().0);
        for item in iter {
            self.indices.push(item.copy_onto(&mut self.region));
        }
    }
}

impl<'a, R: Region> IntoIterator for &'a FlatStack<R> {
    type Item = R::ReadItem<'a>;
    type IntoIter = Iter<'a, R>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            inner: self.indices.iter(),
            region: &self.region,
        }
    }
}

/// An iterator over [`FlatStack`]. The iterator yields [`Region::ReadItem`] elements, which
/// it obtains by looking up indices.
pub struct Iter<'a, R: Region> {
    /// Iterator over indices.
    inner: std::slice::Iter<'a, R::Index>,
    /// Region to map indices to read items.
    region: &'a R,
}

impl<'a, R: Region> Iterator for Iter<'a, R> {
    type Item = R::ReadItem<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|idx| self.region.index(*idx))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<R: Region> ExactSizeIterator for Iter<'_, R> {}

impl<R: Region, T: CopyOnto<R>> FromIterator<T> for FlatStack<R> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let iter = iter.into_iter();
        let mut c = Self::with_capacity(iter.size_hint().0);
        c.extend(iter);
        c
    }
}

impl<R: Region> Clone for FlatStack<R>
where
    for<'a> R::ReadItem<'a>: CopyOnto<R>,
{
    fn clone(&self) -> Self {
        let mut clone = Self::merge_capacity(std::iter::once(self));
        clone.extend(self.iter());
        clone
    }
}

/// A type to wrap and copy iterators onto regions.
///
/// This only exists to avoid blanket implementations that might conflict with more specific
/// implementations offered by some regions.
#[repr(transparent)]
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct CopyIter<I>(pub I);

#[cfg(test)]
mod tests {
    use crate::impls::deduplicate::{CollapseSequence, ConsecutiveOffsetPairs};
    use crate::impls::tuple::TupleARegion;

    use super::*;

    fn copy<R: Region>(r: &mut R, item: impl CopyOnto<R>) -> R::Index {
        item.copy_onto(r)
    }

    #[test]
    fn test_readme() {
        let r: Result<_, u16> = Ok("abc");
        let mut c = FlatStack::default_impl::<Result<&str, u16>>();
        c.copy(r);
        assert_eq!(r, c.get(0));
    }

    #[test]
    fn test_slice_string_onto() {
        let mut c = <StringRegion>::default();
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
        let mut c = <SliceRegion<MirrorRegion<_>>>::default();
        let slice = &[1u8, 2, 3];
        let idx = slice.copy_onto(&mut c);
        assert!(slice.iter().copied().eq(c.index(idx)));
    }

    #[test]
    fn test_vec_onto() {
        let mut c = <SliceRegion<MirrorRegion<u8>>>::default();
        let slice = &[1u8, 2, 3][..];
        let idx = slice.copy_onto(&mut c);
        assert!(slice.iter().copied().eq(c.index(idx)));
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

        fn merge_regions<'a>(regions: impl Iterator<Item = &'a Self> + Clone) -> Self
        where
            Self: 'a,
        {
            Self {
                name_container: <String as Containerized>::Region::merge_regions(
                    regions.clone().map(|r| &r.name_container),
                ),
                age_container: <u16 as Containerized>::Region::merge_regions(
                    regions.clone().map(|r| &r.age_container),
                ),
                hobbies: <Vec<String> as Containerized>::Region::merge_regions(
                    regions.map(|r| &r.hobbies),
                ),
            }
        }

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

        fn heap_size<F: FnMut(usize, usize)>(&self, mut callback: F) {
            self.name_container.heap_size(&mut callback);
            self.age_container.heap_size(&mut callback);
            self.hobbies.heap_size(callback);
        }
    }

    impl CopyOnto<PersonRegion> for &Person {
        fn copy_onto(self, target: &mut PersonRegion) -> <PersonRegion as Region>::Index {
            let name = (&self.name).copy_onto(&mut target.name_container);
            let age = self.age.copy_onto(&mut target.age_container);
            let hobbies = (&self.hobbies).copy_onto(&mut target.hobbies);
            (name, age, hobbies)
        }
    }

    impl ReserveItems<PersonRegion> for &Person {
        fn reserve_items<I>(target: &mut PersonRegion, items: I)
        where
            I: Iterator<Item = Self> + Clone,
        {
            ReserveItems::reserve_items(&mut target.name_container, items.clone().map(|i| &i.name));
            ReserveItems::reserve_items(&mut target.age_container, items.clone().map(|i| &i.age));
            ReserveItems::reserve_items(&mut target.hobbies, items.map(|i| &i.hobbies));
        }
    }

    impl CopyOnto<PersonRegion> for PersonRef<'_> {
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
        assert_eq!(2, person_ref.hobbies.len());
        for (copied_hobby, hobby) in person_ref.hobbies.iter().zip(hobbies) {
            assert_eq!(copied_hobby, hobby);
        }

        let mut cc = FlatStack::default_impl::<Person>();

        cc.copy(c.get(0));

        let person_ref = cc.get(0);
        assert_eq!("Moritz", person_ref.name);
        assert_eq!(123, person_ref.age);
        assert_eq!(2, person_ref.hobbies.len());
        for (copied_hobby, hobby) in person_ref.hobbies.iter().zip(hobbies) {
            assert_eq!(copied_hobby, hobby);
        }
    }

    #[test]
    fn test_result() {
        let r: Result<_, u16> = Ok("abc");
        let mut c = <ResultRegion<StringRegion, MirrorRegion<_>>>::default();
        let idx = copy(&mut c, r);
        assert_eq!(r, c.index(idx));
    }

    #[test]
    fn all_types() {
        fn test_copy<T, R: Region + Clone>(t: T)
        where
            T: CopyOnto<R>,
            // Make sure that types are debug, even if we don't use this in the test.
            for<'a> R::ReadItem<'a>: Debug + CopyOnto<R>,
        {
            let mut c = FlatStack::default();
            c.copy(t);

            let mut cc = c.clone();
            cc.copy(c.get(0));

            c.clear();

            let mut r = R::default();
            cc.get(0).copy_onto(&mut r);

            c.reserve_regions(std::iter::once(&r));

            let mut c = FlatStack::merge_capacity(std::iter::once(&c));
            c.copy(cc.get(0));
        }

        test_copy::<_, StringRegion>(&"a".to_string());
        test_copy::<_, StringRegion>("a".to_string());
        test_copy::<_, StringRegion>("a");

        test_copy::<_, MirrorRegion<()>>(());
        test_copy::<_, MirrorRegion<()>>(&());
        test_copy::<_, MirrorRegion<bool>>(true);
        test_copy::<_, MirrorRegion<bool>>(&true);
        test_copy::<_, MirrorRegion<char>>(' ');
        test_copy::<_, MirrorRegion<char>>(&' ');
        test_copy::<_, MirrorRegion<u8>>(0u8);
        test_copy::<_, MirrorRegion<u8>>(&0u8);
        test_copy::<_, MirrorRegion<u16>>(0u16);
        test_copy::<_, MirrorRegion<u16>>(&0u16);
        test_copy::<_, MirrorRegion<u32>>(0u32);
        test_copy::<_, MirrorRegion<u32>>(&0u32);
        test_copy::<_, MirrorRegion<u64>>(0u64);
        test_copy::<_, MirrorRegion<u64>>(&0u64);
        test_copy::<_, MirrorRegion<u128>>(0u128);
        test_copy::<_, MirrorRegion<u128>>(&0u128);
        test_copy::<_, MirrorRegion<usize>>(0usize);
        test_copy::<_, MirrorRegion<usize>>(&0usize);
        test_copy::<_, MirrorRegion<i8>>(0i8);
        test_copy::<_, MirrorRegion<i8>>(&0i8);
        test_copy::<_, MirrorRegion<i16>>(0i16);
        test_copy::<_, MirrorRegion<i16>>(&0i16);
        test_copy::<_, MirrorRegion<i32>>(0i32);
        test_copy::<_, MirrorRegion<i32>>(&0i32);
        test_copy::<_, MirrorRegion<i64>>(0i64);
        test_copy::<_, MirrorRegion<i64>>(&0i64);
        test_copy::<_, MirrorRegion<i128>>(0i128);
        test_copy::<_, MirrorRegion<i128>>(&0i128);
        test_copy::<_, MirrorRegion<isize>>(0isize);
        test_copy::<_, MirrorRegion<isize>>(&0isize);
        test_copy::<_, MirrorRegion<f32>>(0f32);
        test_copy::<_, MirrorRegion<f32>>(&0f32);
        test_copy::<_, MirrorRegion<f64>>(0f64);
        test_copy::<_, MirrorRegion<f64>>(&0f64);
        test_copy::<_, MirrorRegion<std::num::Wrapping<i8>>>(std::num::Wrapping(0i8));
        test_copy::<_, MirrorRegion<std::num::Wrapping<i8>>>(&std::num::Wrapping(0i8));
        test_copy::<_, MirrorRegion<std::num::Wrapping<i16>>>(std::num::Wrapping(0i16));
        test_copy::<_, MirrorRegion<std::num::Wrapping<i16>>>(&std::num::Wrapping(0i16));
        test_copy::<_, MirrorRegion<std::num::Wrapping<i32>>>(std::num::Wrapping(0i32));
        test_copy::<_, MirrorRegion<std::num::Wrapping<i32>>>(&std::num::Wrapping(0i32));
        test_copy::<_, MirrorRegion<std::num::Wrapping<i64>>>(std::num::Wrapping(0i64));
        test_copy::<_, MirrorRegion<std::num::Wrapping<i64>>>(&std::num::Wrapping(0i64));
        test_copy::<_, MirrorRegion<std::num::Wrapping<i128>>>(std::num::Wrapping(0i128));
        test_copy::<_, MirrorRegion<std::num::Wrapping<i128>>>(&std::num::Wrapping(0i128));
        test_copy::<_, MirrorRegion<std::num::Wrapping<isize>>>(std::num::Wrapping(0isize));
        test_copy::<_, MirrorRegion<std::num::Wrapping<isize>>>(&std::num::Wrapping(0isize));

        test_copy::<_, ResultRegion<MirrorRegion<u8>, MirrorRegion<u8>>>(Result::<u8, u8>::Ok(0));
        test_copy::<_, ResultRegion<MirrorRegion<u8>, MirrorRegion<u8>>>(&Result::<u8, u8>::Ok(0));
        test_copy::<_, ResultRegion<MirrorRegion<u8>, MirrorRegion<u8>>>(Result::<u8, u8>::Err(0));
        test_copy::<_, ResultRegion<MirrorRegion<u8>, MirrorRegion<u8>>>(&Result::<u8, u8>::Err(0));

        test_copy::<_, SliceRegion<MirrorRegion<u8>>>([0u8].as_slice());
        test_copy::<_, SliceRegion<MirrorRegion<u8>>>(vec![0u8]);
        test_copy::<_, SliceRegion<MirrorRegion<u8>>>(&vec![0u8]);

        test_copy::<_, SliceRegion<StringRegion>>(["a"].as_slice());
        test_copy::<_, SliceRegion<StringRegion>>(vec!["a"]);
        test_copy::<_, SliceRegion<StringRegion>>(&vec!["a"]);

        test_copy::<_, SliceRegion<TupleARegion<StringRegion>>>([("a",)].as_slice());
        test_copy::<_, SliceRegion<TupleARegion<StringRegion>>>(vec![("a",)]);
        test_copy::<_, SliceRegion<TupleARegion<StringRegion>>>(&vec![("a",)]);

        test_copy::<_, CopyRegion<_>>([0u8].as_slice());
        test_copy::<_, CopyRegion<_>>(&[0u8].as_slice());

        test_copy::<_, <(u8, u8) as Containerized>::Region>((1, 2));
        test_copy::<_, <(u8, u8) as Containerized>::Region>(&(1, 2));

        test_copy::<_, ConsecutiveOffsetPairs<CopyRegion<_>>>([1, 2, 3].as_slice());

        test_copy::<_, CollapseSequence<CopyRegion<_>>>([1, 2, 3].as_slice());
        test_copy::<_, CollapseSequence<CopyRegion<_>>>(&[1, 2, 3]);

        test_copy::<_, OptionRegion<StringRegion>>(Some("abc"));
        test_copy::<_, OptionRegion<StringRegion>>(&Some("abc"));
        test_copy::<_, OptionRegion<StringRegion>>(Option::<&'static str>::None);
        test_copy::<_, OptionRegion<StringRegion>>(&Option::<&'static str>::None);

        test_copy::<_, ResultRegion<StringRegion, MirrorRegion<u8>>>(
            Result::<&'static str, u8>::Ok("abc"),
        );
        test_copy::<_, ResultRegion<StringRegion, MirrorRegion<u8>>>(
            &Result::<&'static str, u8>::Ok("abc"),
        );
        test_copy::<_, ResultRegion<StringRegion, MirrorRegion<u8>>>(
            Result::<&'static str, u8>::Err(1),
        );
        test_copy::<_, ResultRegion<StringRegion, MirrorRegion<u8>>>(
            Result::<&'static str, u8>::Err(2),
        );
    }

    #[test]
    fn slice_region_read_item() {
        fn is_clone<T: Clone>(_: &T) {}

        let mut c = FlatStack::<SliceRegion<MirrorRegion<u8>>>::default();
        c.copy(vec![1, 2, 3]);

        let mut r = SliceRegion::<MirrorRegion<u8>>::default();
        let idx = [1, 2, 3].copy_onto(&mut r);
        let read_item = r.index(idx);
        is_clone(&read_item);
        let _read_item3 = read_item;
        assert_eq!(vec![1, 2, 3], read_item.into_iter().collect::<Vec<_>>());
    }

    #[test]
    fn flat_stack_default_val() {
        let _ = FlatStack::default_val(&([1u8], [1u8], 0_i64));
    }
}
