#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

use std::borrow::Borrow;
use std::fmt::{Debug, Formatter};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub mod impls;

use crate::impls::offsets::OffsetContainer;
pub use impls::columns::ColumnsRegion;
pub use impls::deduplicate::CombineSequential;
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
    /// An owned type that can be constructed from a read item.
    type Owned;

    /// The type of the data that one gets out of the container.
    type ReadItem<'a>: IntoOwned<'a, Owned = Self::Owned>
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
    #[must_use]
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
    #[must_use]
    fn reborrow<'b, 'a: 'b>(item: Self::ReadItem<'a>) -> Self::ReadItem<'b>
    where
        Self: 'a;
}

/// A trait to let types express a default container type and an owned type, which can
/// be used to define regions in simpler terms.
///
/// # Example
///
/// ```
/// # use flatcontainer::{FlatStack, RegionPreference};
/// let _ = FlatStack::<<((Vec<String>, &[usize]), Option<String>, Result<u8, u16>) as RegionPreference>::Region>::default();
/// ```
pub trait RegionPreference {
    /// The owned type of the region.
    type Owned;
    /// The recommended container type.
    type Region: Region<Owned = Self::Owned>;
}

impl<T: RegionPreference + ?Sized> RegionPreference for &T {
    type Owned = T::Owned;
    type Region = T::Region;
}

/// Push an item `T` into a region.
pub trait Push<T>: Region {
    /// Push `item` into self, returning an index that allows to look up the
    /// corresponding read item.
    #[must_use]
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

/// A reference type corresponding to an owned type, supporting conversion in each direction.
///
/// This trait can be implemented by a GAT, and enables owned types to be borrowed as a GAT.
/// This trait is analogous to `ToOwned`, but not as prescriptive. Specifically, it avoids the
/// requirement that the other trait implement `Borrow`, for which a borrow must result in a
/// `&'self Borrowed`, which cannot move the lifetime into a GAT borrowed type.
pub trait IntoOwned<'a> {
    /// Owned type into which this type can be converted.
    type Owned;
    /// Conversion from an instance of this type to the owned type.
    #[must_use]
    fn into_owned(self) -> Self::Owned;
    /// Clones `self` onto an existing instance of the owned type.
    fn clone_onto(self, other: &mut Self::Owned);
    /// Borrows an owned instance as oneself.
    #[must_use]
    fn borrow_as(owned: &'a Self::Owned) -> Self;
}

impl<'a, T: ToOwned + ?Sized> IntoOwned<'a> for &'a T {
    type Owned = T::Owned;
    #[inline]
    fn into_owned(self) -> Self::Owned {
        self.to_owned()
    }
    #[inline]
    fn clone_onto(self, other: &mut Self::Owned) {
        <T as ToOwned>::clone_into(self, other)
    }
    #[inline]
    fn borrow_as(owned: &'a Self::Owned) -> Self {
        owned.borrow()
    }
}

/// A container for indices into a region.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound = "
            R: Serialize + for<'a> Deserialize<'a>,
            R::Index: Serialize + for<'a> Deserialize<'a>,
            S: Serialize + for<'a> Deserialize<'a>,
            ")
)]
pub struct FlatStack<R: Region, S: OffsetContainer<R::Index> = Vec<<R as Region>::Index>> {
    /// The indices, which we use to lookup items in the region.
    indices: S,
    /// A region to index into.
    region: R,
}

impl<R: Region, S: OffsetContainer<<R as Region>::Index>> Default for FlatStack<R, S> {
    #[inline]
    fn default() -> Self {
        Self {
            indices: S::default(),
            region: R::default(),
        }
    }
}

impl<R: Region, S: OffsetContainer<<R as Region>::Index>> Debug for FlatStack<R, S>
where
    for<'a> R::ReadItem<'a>: Debug,
    for<'a> &'a S: IntoIterator<Item = &'a R::Index>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<R: Region, S: OffsetContainer<<R as Region>::Index>> FlatStack<R, S> {
    /// Returns a flat stack that can absorb `capacity` indices without reallocation.
    ///
    /// Prefer [`Self::merge_capacity`] over this function to also pre-size the regions.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            indices: S::with_capacity(capacity),
            region: R::default(),
        }
    }

    /// Returns a flat stack that can absorb the contents of `iter` without reallocation.
    #[must_use]
    pub fn merge_capacity<'a, I: Iterator<Item = &'a Self> + Clone + 'a>(stacks: I) -> Self
    where
        Self: 'a,
    {
        Self {
            indices: S::merge_regions(stacks.clone().map(|s| &s.indices)),
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
        self.region.index(self.indices.index(offset))
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

    /// Reserves space to hold `additional` indices.
    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        self.indices.reserve(additional);
    }

    /// Remove all elements while possibly retaining allocations.
    #[inline]
    pub fn clear(&mut self) {
        self.indices.clear();
        self.region.clear();
    }

    /// Reserve space for the items returned by the iterator.
    #[inline]
    pub fn reserve_items<T>(&mut self, items: impl Iterator<Item = T> + Clone)
    where
        R: ReserveItems<T>,
    {
        ReserveItems::reserve_items(&mut self.region, items);
    }

    /// Reserve space for the regions returned by the iterator.
    #[inline]
    pub fn reserve_regions<'a>(&mut self, regions: impl Iterator<Item = &'a R> + Clone)
    where
        R: 'a,
    {
        self.region.reserve_regions(regions);
    }

    /// Heap size, size - capacity
    #[inline]
    pub fn heap_size<F: FnMut(usize, usize)>(&self, mut callback: F) {
        self.region.heap_size(&mut callback);
        self.indices.heap_size(callback);
    }
}

impl<R, S> FlatStack<R, S>
where
    R: Region,
    S: OffsetContainer<<R as Region>::Index>,
{
    /// Iterate the items in this stack.
    #[inline]
    pub fn iter<'a>(&'a self) -> Iter<'a, R, <&'a S as IntoIterator>::IntoIter>
    where
        &'a S: IntoIterator<Item = &'a R::Index>,
    {
        self.into_iter()
    }
}

impl<R: Region> FlatStack<R> {
    /// Default implementation based on the preference of type `T`.
    #[inline]
    #[must_use]
    pub fn default_impl<T: RegionPreference<Region = R>>() -> Self {
        Self::default()
    }

    /// Returns the total number of indices the stack can hold without reallocation.
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.indices.capacity()
    }
}

impl<T, R: Region + Push<T>, S: OffsetContainer<<R as Region>::Index>> Extend<T>
    for FlatStack<R, S>
{
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        let iter = iter.into_iter();
        self.reserve(iter.size_hint().0);
        for item in iter {
            self.indices.push(self.region.push(item));
        }
    }
}

impl<'a, R: Region, S: OffsetContainer<<R as Region>::Index>> IntoIterator for &'a FlatStack<R, S>
where
    &'a S: IntoIterator<Item = &'a <R as Region>::Index>,
{
    type Item = R::ReadItem<'a>;
    type IntoIter = Iter<'a, R, <&'a S as IntoIterator>::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            inner: self.indices.into_iter(),
            region: &self.region,
        }
    }
}

/// An iterator over [`FlatStack`]. The iterator yields [`Region::ReadItem`] elements, which
/// it obtains by looking up indices.
pub struct Iter<'a, R, S>
where
    R: Region,
    S: Iterator<Item = &'a <R as Region>::Index>,
{
    /// Iterator over indices.
    inner: S,
    /// Region to map indices to read items.
    region: &'a R,
}

impl<'a, R, S> Iterator for Iter<'a, R, S>
where
    R: Region,
    S: Iterator<Item = &'a <R as Region>::Index>,
{
    type Item = R::ReadItem<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|idx| self.region.index(*idx))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a, R, S> ExactSizeIterator for Iter<'a, R, S>
where
    R: Region,
    S: ExactSizeIterator<Item = &'a <R as Region>::Index>,
{
}

impl<'a, R, S> Clone for Iter<'a, R, S>
where
    R: Region,
    S: Iterator<Item = &'a <R as Region>::Index> + Clone,
{
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

    fn clone_from(&mut self, source: &Self) {
        self.region.clone_from(&source.region);
        self.indices.clone_from(&source.indices);
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
    use crate::impls::deduplicate::{CollapseSequence, ConsecutiveOffsetPairs, Sequential};
    use crate::impls::offsets::OffsetStride;
    use crate::impls::tuple::{TupleABRegion, TupleARegion};

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
            let _ = r.push(cc.get(0));

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

        test_copy::<_, <(u8, u8) as RegionPreference>::Region>((1, 2));
        test_copy::<_, <(u8, u8) as RegionPreference>::Region>(&(1, 2));

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

    #[test]
    fn test_owned() {
        fn owned_roundtrip<R, O>(region: &mut R, index: R::Index)
        where
            for<'a> R: Region + Push<<<R as Region>::ReadItem<'a> as IntoOwned<'a>>::Owned>,
            for<'a> R::ReadItem<'a>: IntoOwned<'a, Owned = O> + Eq + Debug,
        {
            let item = region.index(index);
            let owned = item.into_owned();
            let index2 = region.push(owned);
            let item = region.index(index);
            assert_eq!(item, region.index(index2));
        }

        let mut c = <StringRegion>::default();
        let index = c.push("abc".to_string());
        owned_roundtrip::<StringRegion, String>(&mut c, index);
    }

    #[test]
    fn test_my_understanding() {
        let item = (vec![1, 2, 3], vec![1, 2, 3]);
        let mut r = <TupleABRegion<SliceRegion<MirrorRegion<u8>>, OwnedRegion<u8>>>::default();
        let _index: ((usize, usize), (usize, usize)) = r.push(&item);

        let mut r = <TupleABRegion<
            ConsecutiveOffsetPairs<SliceRegion<MirrorRegion<u8>>>,
            ConsecutiveOffsetPairs<OwnedRegion<u8>>,
        >>::default();
        let _index: (Sequential, Sequential) = r.push(&item);

        let mut r = <CombineSequential<
            TupleABRegion<
                ConsecutiveOffsetPairs<SliceRegion<MirrorRegion<u8>>>,
                ConsecutiveOffsetPairs<OwnedRegion<u8>>,
            >,
        >>::default();
        let _index: Sequential = r.push(&item);

        let mut fs = FlatStack::<
            CombineSequential<
                TupleABRegion<
                    ConsecutiveOffsetPairs<SliceRegion<MirrorRegion<u8>>>,
                    ConsecutiveOffsetPairs<OwnedRegion<u8>>,
                >,
            >,
            OffsetStride,
        >::default();

        for _ in 0..1000 {
            fs.copy(&item);
            let mut size = 0;
            let mut capacity = 0;
            let mut count = 0;
            fs.heap_size(|siz, cap| {
                size += siz;
                capacity += cap;
                count += 1;
            });

            println!("size {size}, capacity {capacity}, allocations {count}");
        }
        assert_eq!(&item.1, fs.get(0).1);
    }

    /// Test that items and owned variants can be reborrowed to shorten their lifetimes.
    fn _test_reborrow<R>(item: R::ReadItem<'_>, owned: &R::Owned)
    where
        R: Region,
        for<'a> R::ReadItem<'a>: Eq,
    {
        // The following line requires `reborrow` because otherwise owned must outlive '_.
        // fn _test_reborrow<R>(item: R::ReadItem<'_>, owned: &R::Owned) where R: Region, for<'a> R::ReadItem<'a>: Eq {
        //                      ----                          - let's call the lifetime of this reference `'1`
        //                      |
        //                      has type `<R as Region>::ReadItem<'2>`
        //     // The following line requires `reborrow` because otherwise owned must outlive '_.
        //     let _ = item == IntoOwned::borrow_as(owned);
        //                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ argument requires that `'1` must outlive `'2`
        // let _ = item == IntoOwned::borrow_as(owned);
        let _ = R::reborrow(item) == R::reborrow(IntoOwned::borrow_as(owned));
    }
}
