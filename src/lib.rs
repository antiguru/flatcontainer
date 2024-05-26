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
pub use impls::slice_copy::OwnedRegion;
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
/// Implement the [`Push`] trait for all types that can be copied into a region.
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

    /// Converts a read item into one with a narrower lifetime.
    fn reborrow<'b, 'a: 'b>(item: Self::ReadItem<'a>) -> Self::ReadItem<'b>
    where
        Self: 'a;
}

/// A trait to let types express a default container type.
pub trait Containerized {
    /// The recommended container type.
    type Region: Region;
}

/// Push an item `T` into a region.
pub trait Push<T>: Region {
    /// Push `item` into self, returning an index that allows to look up the
    /// corresponding read item.
    fn push(&mut self, item: T) -> Self::Index;
}

/// Reserve space in the receiving region.
///
/// Closely related to [`Push`], but separate because target type is likely different.
pub trait ReserveItems<T>: Region {
    /// Ensure that the region can absorb `items` without reallocation.
    fn reserve_items<I>(&mut self, items: I)
    where
        I: Iterator<Item = T> + Clone;
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
    pub fn copy<T>(&mut self, item: T)
    where
        R: Push<T>,
    {
        let index = self.region.push(item);
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
        R: ReserveItems<T>,
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
}

impl<T, R: Region + Push<T>> Extend<T> for FlatStack<R> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        let iter = iter.into_iter();
        self.reserve(iter.size_hint().0);
        for item in iter {
            self.indices.push(self.region.push(item));
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

impl<R: Region> Clone for Iter<'_, R> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            region: self.region,
        }
    }
}

impl<R: Region + Push<T>, T> FromIterator<T> for FlatStack<R> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let iter = iter.into_iter();
        let mut c = Self::with_capacity(iter.size_hint().0);
        c.extend(iter);
        c
    }
}

impl<R: Region + Clone> Clone for FlatStack<R> {
    fn clone(&self) -> Self {
        Self {
            region: self.region.clone(),
            indices: self.indices.clone(),
        }
    }
}

/// A type to wrap and copy iterators onto regions.
///
/// This only exists to avoid blanket implementations that might conflict with more specific
/// implementations offered by some regions.
#[repr(transparent)]
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct CopyIter<I>(pub I);

/// Conversion of references to owned data. Similar to [`ToOwned`], but without the requirement
/// that target can be borrowed to self.
///
/// The clumsy names originate from `to_owned` already being in scope.
pub trait ReadToOwned {
    /// The owned type. Static lifetime to indicate that the lifetime of the owned object must not
    /// depend on self.
    type Owned;
    /// Convert self into an owned representation.
    fn read_to_owned(self) -> Self::Owned;
    /// Convert self into an owned representation, re-using an existing allocation.
    fn read_to_owned_into(&self, target: &mut Self::Owned);
}

/// TODO
pub trait OpinionatedRegion: Region { //where for<'a> <Self as Region>::ReadItem<'a>: ReadToOwned<Owned=Self::Owned> {
    /// TODO
    type Owned;

    /// TODO
    fn item_to_owned(item: Self::ReadItem<'_>) -> Self::Owned;
    /// TODO
    fn item_to_owned_into(item: Self::ReadItem<'_>, target: &mut Self::Owned);
}

impl<T: ToOwned + ?Sized> ReadToOwned for &T
where
    T::Owned: 'static,
{
    type Owned = T::Owned;

    fn read_to_owned(self) -> Self::Owned {
        self.to_owned()
    }

    fn read_to_owned_into(&self, target: &mut Self::Owned) {
        <T as ToOwned>::clone_into(self, target);
    }
}

#[cfg(test)]
mod tests {
    use crate::impls::deduplicate::{CollapseSequence, ConsecutiveOffsetPairs};
    use crate::impls::tuple::TupleARegion;

    use super::*;

    fn copy<R: Region + Push<T>, T>(r: &mut R, item: T) -> R::Index {
        r.push(item)
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
        let index = c.push("abc".to_string());
        assert_eq!("abc", c.index(index));
        let index = c.push("def");
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
        let idx = c.push(slice);
        assert!(slice.iter().copied().eq(c.index(idx)));
    }

    #[test]
    fn test_vec_onto() {
        let mut c = <SliceRegion<MirrorRegion<u8>>>::default();
        let slice = &[1u8, 2, 3][..];
        let idx = c.push(slice);
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

    impl ReadToOwned for PersonRef<'_> {
        type Owned = Person;
        fn read_to_owned(self) -> Person {
            Person {
                name: self.name.to_string(),
                age: self.age,
                hobbies: self.hobbies.iter().map(|s| s.to_string()).collect(),
            }
        }
        fn read_to_owned_into(&self, target: &mut Person) {
            target.name.clear();
            target.name.push_str(self.name);
            target.age = self.age;
            target.hobbies.clear();
            target
                .hobbies
                .extend(self.hobbies.iter().map(str::to_string));
        }
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

        fn reborrow<'b, 'a: 'b>(item: Self::ReadItem<'a>) -> Self::ReadItem<'b>
        where
            Self: 'a,
        {
            PersonRef {
                name: <String as Containerized>::Region::reborrow(item.name),
                age: <u16 as Containerized>::Region::reborrow(item.age),
                hobbies: <Vec<String> as Containerized>::Region::reborrow(item.hobbies),
            }
        }
    }

    impl Push<&Person> for PersonRegion {
        fn push(&mut self, item: &Person) -> <PersonRegion as Region>::Index {
            let name = self.name_container.push(&item.name);
            let age = self.age_container.push(item.age);
            let hobbies = self.hobbies.push(&item.hobbies);
            (name, age, hobbies)
        }
    }

    impl<'a> ReserveItems<&'a Person> for PersonRegion {
        fn reserve_items<I>(&mut self, items: I)
        where
            I: Iterator<Item = &'a Person> + Clone,
        {
            self.name_container
                .reserve_items(items.clone().map(|i| &i.name));
            self.age_container
                .reserve_items(items.clone().map(|i| &i.age));
            self.hobbies.reserve_items(items.map(|i| &i.hobbies));
        }
    }

    impl Push<PersonRef<'_>> for PersonRegion {
        fn push(&mut self, item: PersonRef<'_>) -> <PersonRegion as Region>::Index {
            let name = self.name_container.push(item.name);
            let age = self.age_container.push(item.age);
            let hobbies = self.hobbies.push(item.hobbies);
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
            for<'a> R: Push<T> + Push<<R as Region>::ReadItem<'a>>,
            // Make sure that types are debug, even if we don't use this in the test.
            for<'a> R::ReadItem<'a>: Debug,
        {
            let mut c = FlatStack::default();
            c.copy(t);

            let mut cc = c.clone();
            cc.copy(c.get(0));

            c.clear();

            let mut r = R::default();
            r.push(cc.get(0));

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

        test_copy::<_, OwnedRegion<_>>([0u8].as_slice());
        test_copy::<_, OwnedRegion<_>>(&[0u8].as_slice());

        test_copy::<_, <(u8, u8) as Containerized>::Region>((1, 2));
        test_copy::<_, <(u8, u8) as Containerized>::Region>(&(1, 2));

        test_copy::<_, ConsecutiveOffsetPairs<OwnedRegion<_>>>([1, 2, 3].as_slice());

        test_copy::<_, CollapseSequence<OwnedRegion<_>>>([1, 2, 3].as_slice());
        test_copy::<_, CollapseSequence<OwnedRegion<_>>>(&[1, 2, 3]);

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
        let idx = r.push([1, 2, 3]);
        let read_item = r.index(idx);
        is_clone(&read_item);
        let _read_item3 = read_item;
        assert_eq!(vec![1, 2, 3], read_item.into_iter().collect::<Vec<_>>());
    }

    #[test]
    fn nested_slice_copy() {
        let mut c = FlatStack::default_impl::<[[[[[u8; 1]; 1]; 1]; 1]; 1]>();

        c.copy([[[[[1]]]]]);
        c.copy(&[[[[[1]]]]]);
        c.copy(&[[[[[&1]]]]]);
        c.copy([[[[[&1]]]]]);
        c.copy([[&[[[&1]]]]]);
        c.copy([[[[[1]]; 1]; 1]; 1]);
        c.copy(&[[[[[1; 1]; 1]; 1]; 1]; 1]);
        c.copy(&[[[[[&1; 1]; 1]; 1]; 1]; 1]);
        c.copy([[[[[&1; 1]; 1]; 1]; 1]; 1]);
        c.copy([[&[[[&1; 1]; 1]; 1]; 1]; 1]);
        c.copy([[vec![[[1; 1]; 1]; 1]; 1]; 1]);
        c.copy(&[[vec![[[1; 1]; 1]; 1]; 1]; 1]);
        c.copy(&[[vec![[[&1; 1]; 1]; 1]; 1]; 1]);
        c.copy([[[vec![[&1; 1]; 1]; 1]; 1]; 1]);
        c.copy([[&vec![[[&1; 1]; 1]; 1]; 1]; 1]);
    }

    fn owned_roundtrip<R, O>(region: &mut R, index: R::Index)
    where
        for<'a> R: Region + Push<<<R as Region>::ReadItem<'a> as ReadToOwned>::Owned>,
        for<'a> R::ReadItem<'a>: ReadToOwned<Owned=O>,
    {
        let item = region.index(index).read_to_owned();
        region.push(item);
    }

    #[test]
    fn test_owned() {
        let mut c = <StringRegion>::default();
        let index = c.push("abc".to_string());
        owned_roundtrip(&mut c, index);
    }
}
