//! A region that stores slices.

use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, Range};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{
    Clear, HeapSize, Index, IndexAs, IntoOwned, Len, Push, Region, RegionPreference, Reserve,
    ReserveItems,
};

impl<T: RegionPreference> RegionPreference for Vec<T> {
    type Owned = Vec<T::Owned>;
    type Region = SliceRegion<T::Region>;
}

impl<T: RegionPreference> RegionPreference for [T] {
    type Owned = Vec<T::Owned>;
    type Region = SliceRegion<T::Region>;
}

impl<T: RegionPreference, const N: usize> RegionPreference for [T; N] {
    type Owned = Vec<T::Owned>;
    type Region = SliceRegion<T::Region>;
}

type Idx = u64;

/// A container representing slices of data.
///
/// Reading from this region is more involved than for others, because the data only exists in
/// an indexable representation. The read item is a [`ReadSlice`], which can be iterated or indexed.
/// However, it is not possible to represent the data as a slice, simply because the slice doesn't
/// exist.
///
/// # Examples
///
/// We fill some data into a slice region and use the [`ReadSlice`] to extract it later.
/// ```
/// use flatcontainer::{RegionPreference, Push, Region, SliceRegion, Index};
/// let mut r = <SliceRegion<<String as RegionPreference>::Region>>::default();
///
/// let panagram_en = "The quick fox jumps over the lazy dog"
///     .split(" ")
///     .collect::<Vec<_>>();
/// let panagram_de = "Zwölf Boxkämpfer jagen Viktor quer über den großen Sylter Deich"
///     .split(" ")
///     .collect::<Vec<_>>();
///
/// r.push(&panagram_en);
/// r.push(&panagram_de);
///
/// assert!(panagram_de.into_iter().eq(r.index(0)));
/// assert!(panagram_en.into_iter().eq(r.index(1)));
///
/// assert_eq!(r.index(1).get(2), "jagen");
/// ```
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SliceRegion<R, B = Vec<Idx>> {
    /// Container of bounds.
    bounds: B,
    /// Inner region.
    inner: R,
}

impl<R, O> Clone for SliceRegion<R, O>
where
    R: Clone,
    O: Clone,
{
    fn clone(&self) -> Self {
        Self {
            bounds: self.bounds.clone(),
            inner: self.inner.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.bounds.clone_from(&source.bounds);
        self.inner.clone_from(&source.inner);
    }
}

impl<R: Region, B: Region> Region for SliceRegion<R, B> {
    #[inline]
    fn merge_regions<'a>(regions: impl Iterator<Item = &'a Self> + Clone) -> Self
    where
        Self: 'a,
    {
        Self {
            bounds: B::merge_regions(regions.clone().map(|r| &r.bounds)),
            inner: R::merge_regions(regions.map(|r| &r.inner)),
        }
    }

    #[inline]
    fn reserve_regions<'a, I>(&mut self, regions: I)
    where
        Self: 'a,
        I: Iterator<Item = &'a Self> + Clone,
    {
        self.bounds
            .reserve_regions(regions.clone().map(|r| &r.bounds));
        self.inner.reserve_regions(regions.map(|r| &r.inner));
    }
}

impl<R, B> Len for SliceRegion<R, B>
where
    B: Len,
{
    #[inline]
    fn len(&self) -> usize {
        self.bounds.len()
    }
}

impl<R, B> Clear for SliceRegion<R, B>
where
    R: Clear,
    B: Clear,
{
    #[inline]
    fn clear(&mut self) {
        self.bounds.clear();
        self.inner.clear();
    }
}
impl<R, B> HeapSize for SliceRegion<R, B>
where
    R: HeapSize,
    B: HeapSize,
{
    #[inline]
    fn heap_size<F: FnMut(usize, usize)>(&self, mut callback: F) {
        self.bounds.heap_size(&mut callback);
        self.inner.heap_size(callback);
    }
}

impl<R, B> Index for SliceRegion<R, B>
where
    R: Index,
    B: IndexAs<Idx>,
{
    type Owned = Vec<R::Owned>;

    type ReadItem<'a> = ReadSlice<'a, R, B> where Self: 'a;
    #[inline]
    fn index(&self, index: usize) -> Self::ReadItem<'_> {
        let start = if index == 0 {
            0
        } else {
            self.bounds
                .index_as(index - 1)
                .try_into()
                .expect("must fit")
        };
        let end = self.bounds.index_as(index).try_into().expect("must fit");
        ReadSlice(Ok(ReadSliceInner {
            region: self,
            start,
            end,
        }))
    }

    #[inline]
    fn reborrow<'b, 'a: 'b>(item: Self::ReadItem<'a>) -> Self::ReadItem<'b>
    where
        Self: 'a,
    {
        item
    }
}

impl<R: Default, B: Default> Default for SliceRegion<R, B> {
    #[inline]
    fn default() -> Self {
        Self {
            bounds: B::default(),
            inner: R::default(),
        }
    }
}

/// A helper to read data out of a slice region.
pub struct ReadSlice<'a, R: Index, B>(Result<ReadSliceInner<'a, R, B>, &'a [R::Owned]>);

impl<R: Index, B: IndexAs<Idx>> ReadSlice<'_, R, B> {
    /// Read the n-th item from the underlying region.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds, i.e., it is larger than the
    /// length of this slice representation.
    #[inline]
    #[must_use]
    pub fn get(&self, index: usize) -> R::ReadItem<'_> {
        match &self.0 {
            Ok(inner) => inner.get(index),
            Err(slice) => IntoOwned::borrow_as(&slice[index]),
        }
    }

    /// The number of elements in this slice.
    #[must_use]
    pub fn len(&self) -> usize {
        match self.0 {
            Ok(inner) => inner.len(),
            Err(slice) => slice.len(),
        }
    }

    /// Returns `true` if the slice is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        match self.0 {
            Ok(inner) => inner.is_empty(),
            Err(slice) => slice.is_empty(),
        }
    }

    /// Returns an iterator over all contained items.
    #[must_use]
    pub fn iter(&self) -> <Self as IntoIterator>::IntoIter {
        self.into_iter()
    }
}

impl<R: Index, B: IndexAs<Idx>> PartialEq for ReadSlice<'_, R, B>
where
    for<'a> R::ReadItem<'a>: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.iter().eq(*other)
    }
}

impl<R: Index, B: IndexAs<Idx>> Eq for ReadSlice<'_, R, B> where for<'a> R::ReadItem<'a>: Eq {}

impl<R: Index, O: IndexAs<Idx>> PartialOrd for ReadSlice<'_, R, O>
where
    for<'a> R::ReadItem<'a>: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.iter().partial_cmp(*other)
    }
}

impl<R: Index, O: IndexAs<Idx>> Ord for ReadSlice<'_, R, O>
where
    for<'a> R::ReadItem<'a>: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.iter().cmp(*other)
    }
}

struct ReadSliceInner<'a, R, O> {
    region: &'a SliceRegion<R, O>,
    start: usize,
    end: usize,
}

impl<R: Index, O: IndexAs<Idx>> ReadSliceInner<'_, R, O> {
    /// Read the n-th item from the underlying region.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds, i.e., it is larger than the
    /// length of this slice representation.
    #[inline]
    #[must_use]
    pub fn get(&self, index: usize) -> R::ReadItem<'_> {
        assert!(
            index <= self.end - self.start,
            "Index {index} out of bounds {} ({}..{})",
            self.end - self.start,
            self.start,
            self.end
        );
        self.region.inner.index(self.start + index)
    }

    /// The number of elements in this slice.
    #[must_use]
    pub fn len(&self) -> usize {
        self.end - self.start
    }

    /// Returns `true` if the slice is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }
}

impl<R: Index, O: IndexAs<Idx>> Debug for ReadSlice<'_, R, O>
where
    for<'a> R::ReadItem<'a>: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<R: Index, O: IndexAs<Idx>> Clone for ReadSlice<'_, R, O> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<R: Index, O: IndexAs<Idx>> Clone for ReadSliceInner<'_, R, O> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<R: Index, O: IndexAs<Idx>> Copy for ReadSlice<'_, R, O> {}
impl<R: Index, O: IndexAs<Idx>> Copy for ReadSliceInner<'_, R, O> {}

impl<'a, R, O> IntoOwned<'a> for ReadSlice<'a, R, O>
where
    R: Index,
    O: IndexAs<Idx>,
{
    type Owned = Vec<R::Owned>;

    #[inline]
    fn into_owned(self) -> Self::Owned {
        self.iter().map(IntoOwned::into_owned).collect()
    }

    #[inline]
    fn clone_onto(self, other: &mut Self::Owned) {
        let r = std::cmp::min(self.len(), other.len());
        for (item, target) in self.iter().zip(other.iter_mut()) {
            item.clone_onto(target);
        }
        other.extend(self.iter().skip(r).map(IntoOwned::into_owned));
        other.truncate(self.len());
    }

    #[inline]
    fn borrow_as(owned: &'a Self::Owned) -> Self {
        Self(Err(owned.as_slice()))
    }
}

impl<'a, R: Index, O: IndexAs<Idx>> IntoIterator for ReadSlice<'a, R, O> {
    type Item = R::ReadItem<'a>;
    type IntoIter = ReadSliceIter<'a, R, O>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        match self.0 {
            Ok(inner) => {
                ReadSliceIter(Ok(ReadSliceIterInner(inner.region, inner.start..inner.end)))
            }
            Err(slice) => ReadSliceIter(Err(slice.iter())),
        }
    }
}

/// An iterator over the items read from a slice region.
#[derive(Debug)]
pub struct ReadSliceIter<'a, C: Index, O: IndexAs<Idx>>(
    Result<ReadSliceIterInner<'a, C, O>, std::slice::Iter<'a, C::Owned>>,
);

impl<'a, C: Index, O: IndexAs<Idx>> Clone for ReadSliceIter<'a, C, O> {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

/// An iterator over the items read from a slice region.
#[derive(Debug)]
pub struct ReadSliceIterInner<'a, C: Index, O: IndexAs<Idx>>(&'a SliceRegion<C, O>, Range<usize>);

impl<'a, C: Index, O: IndexAs<Idx>> Clone for ReadSliceIterInner<'a, C, O> {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0, self.1.clone())
    }
}

impl<'a, C: Index, O: IndexAs<Idx>> Iterator for ReadSliceIter<'a, C, O> {
    type Item = C::ReadItem<'a>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.0 {
            Ok(inner) => inner.next(),
            Err(iter) => iter.next().map(IntoOwned::borrow_as),
        }
    }
}

impl<'a, R, O> ExactSizeIterator for ReadSliceIter<'a, R, O>
where
    R: Index,
    O: IndexAs<Idx>,
    std::slice::Iter<'a, R::Owned>: ExactSizeIterator,
    ReadSliceIterInner<'a, R, O>: ExactSizeIterator,
{
}

impl<'a, C: Index, O: IndexAs<Idx>> Iterator for ReadSliceIterInner<'a, C, O> {
    type Item = C::ReadItem<'a>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|idx| self.0.inner.index(idx))
    }
}

impl<'a, R, O> ExactSizeIterator for ReadSliceIterInner<'a, R, O>
where
    R: Index,
    O: IndexAs<Idx>,
    Range<usize>: ExactSizeIterator,
{
}

impl<'a, C, T, B> Push<&'a [T]> for SliceRegion<C, B>
where
    C: Push<&'a T> + Len,
    B: IndexAs<Idx> + Push<Idx>,
{
    #[inline]
    fn push(&mut self, items: &'a [T]) {
        for item in items.iter() {
            self.inner.push(item);
        }
        self.bounds
            .push(self.inner.len().try_into().expect("must fit"));
    }
}

impl<'a, T, R, O> ReserveItems<&'a [T]> for SliceRegion<R, O>
where
    R: ReserveItems<&'a T>,
    O: Reserve,
{
    #[inline]
    fn reserve_items<I>(&mut self, items: I)
    where
        I: Iterator<Item = &'a [T]> + Clone,
    {
        self.bounds.reserve(1);
        self.inner.reserve_items(items.flatten());
    }
}

impl<C, T, O> Push<Vec<T>> for SliceRegion<C, O>
where
    C: Push<T> + Len,
    O: IndexAs<Idx> + Push<Idx>,
{
    #[inline]
    fn push(&mut self, items: Vec<T>) {
        for item in items {
            self.inner.push(item);
        }
        self.bounds
            .push(self.inner.len().try_into().expect("must fit"));
    }
}

impl<C, T, O> Push<&Vec<T>> for SliceRegion<C, O>
where
    C: for<'a> Push<&'a T> + Len,
    O: Push<Idx> + IndexAs<Idx>,
{
    #[inline]
    fn push(&mut self, item: &Vec<T>) {
        self.push(item.as_slice())
    }
}

impl<'a, C, T, O> Push<&&'a Vec<T>> for SliceRegion<C, O>
where
    C: Push<&'a T> + Len,
    O: Push<Idx> + IndexAs<Idx>,
{
    #[inline]
    fn push(&mut self, item: &&'a Vec<T>) {
        self.push(item.as_slice())
    }
}

impl<'a, T, R, O> ReserveItems<&'a Vec<T>> for SliceRegion<R, O>
where
    R: ReserveItems<&'a T>,
    O: Reserve,
{
    #[inline]
    fn reserve_items<I>(&mut self, items: I)
    where
        I: Iterator<Item = &'a Vec<T>> + Clone,
    {
        self.reserve_items(items.map(Deref::deref));
    }
}

impl<'a, C, B> Push<ReadSlice<'a, C, B>> for SliceRegion<C, B>
where
    C: Index + Push<<C as Index>::ReadItem<'a>> + Len,
    B: IndexAs<Idx> + Push<Idx>,
{
    #[inline]
    fn push(&mut self, item: ReadSlice<'a, C, B>) {
        match item.0 {
            Ok(inner) => self.push(inner),
            Err(slice) => {
                for item in slice.iter().map(IntoOwned::borrow_as) {
                    self.inner.push(item);
                }
                self.bounds
                    .push(self.inner.len().try_into().expect("must fit"));
            }
        }
    }
}

impl<'a, C, O> Push<ReadSliceInner<'a, C, O>> for SliceRegion<C, O>
where
    C: Index + Push<<C as Index>::ReadItem<'a>> + Len,
    O: IndexAs<Idx> + Push<Idx>,
{
    #[inline]
    fn push(&mut self, item: ReadSliceInner<'a, C, O>) {
        let ReadSliceInner { region, start, end } = item;
        for index in start..end {
            self.inner.push(region.inner.index(index));
        }
        self.bounds
            .push(self.inner.len().try_into().expect("must fit"));
    }
}

impl<T, R, O, const N: usize> Push<[T; N]> for SliceRegion<R, O>
where
    for<'a> R: Push<T> + Len,
    O: IndexAs<Idx> + Push<Idx>,
{
    #[inline]
    fn push(&mut self, items: [T; N]) {
        for item in items {
            self.inner.push(item);
        }
        self.bounds
            .push(self.inner.len().try_into().expect("must fit"));
    }
}

impl<'a, T, R, O, const N: usize> Push<&'a [T; N]> for SliceRegion<R, O>
where
    R: Push<&'a T> + Len,
    O: Push<Idx> + IndexAs<Idx>,
{
    #[inline]
    fn push(&mut self, item: &'a [T; N]) {
        self.push(item.as_slice())
    }
}

impl<'a, T, R, O, const N: usize> Push<&&'a [T; N]> for SliceRegion<R, O>
where
    R: Push<&'a T> + Len,
    O: Push<Idx> + IndexAs<Idx>,
{
    #[inline]
    fn push(&mut self, item: &&'a [T; N]) {
        self.push(item.as_slice())
    }
}

impl<'a, T, R, O, const N: usize> ReserveItems<&'a [T; N]> for SliceRegion<R, O>
where
    R: ReserveItems<&'a T>,
    O: Reserve,
{
    fn reserve_items<I>(&mut self, items: I)
    where
        I: Iterator<Item = &'a [T; N]> + Clone,
    {
        self.reserve_items(items.map(<[T; N]>::as_slice));
    }
}

impl<'a, R, O> ReserveItems<ReadSlice<'a, R, O>> for SliceRegion<R, O>
where
    R: Index + ReserveItems<<R as Index>::ReadItem<'a>> + 'a,
    O: Reserve + IndexAs<Idx>,
{
    fn reserve_items<I>(&mut self, items: I)
    where
        I: Iterator<Item = ReadSlice<'a, R, O>> + Clone,
    {
        self.bounds.reserve(1);
        self.inner.reserve_items(items.flatten());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Index, Push};

    #[test]
    fn read_slice() {
        let s = [1, 2, 3, 4];
        let mut r = <SliceRegion<Vec<u8>>>::default();

        r.push(s);

        assert!(s.iter().eq(r.index(0).iter()));

        r.push(s);
        let slice = r.index(1);
        assert_eq!(s.len(), slice.len());
        assert!(!slice.is_empty());
        assert_eq!(s.get(0), Some(&1));
        assert_eq!(s.get(1), Some(&2));
        assert_eq!(s.get(2), Some(&3));
        assert_eq!(s.get(3), Some(&4));

        <_ as Push<[u8; 0]>>::push(&mut r, []);
        let slice = r.index(2);
        assert_eq!(0, slice.len());
        assert!(slice.is_empty());
    }

    #[test]
    #[should_panic]
    fn test_get_out_of_bounds() {
        let mut r = <SliceRegion<Vec<u8>>>::default();
        r.push([1; 4]);

        // Index 4 is out of bounds and expected to panic.
        let _ = r.index(0).get(4);
    }

    #[test]
    fn test_read_slice_debug() {
        let mut r = <SliceRegion<Vec<u8>>>::default();
        r.push([1; 4]);

        assert_eq!("[1, 1, 1, 1]", format!("{:?}", r.index(0)));
    }

    #[test]
    fn test_read_slice_clone() {
        let mut r = <SliceRegion<Vec<u8>>>::default();
        r.push([1; 4]);

        assert_eq!("[1, 1, 1, 1]", format!("{:?}", r.index(0).clone()));
    }

    #[test]
    fn test_read_slice_eq() {
        let mut r = <SliceRegion<Vec<u8>>>::default();
        r.push([1; 4]);

        assert_eq!(
            <ReadSlice<_, _> as IntoOwned>::borrow_as(&vec![1; 4]),
            r.index(0)
        );
        assert_ne!(
            <ReadSlice<_, _> as IntoOwned>::borrow_as(&vec![0; 4]),
            r.index(0)
        );
        assert_ne!(
            <ReadSlice<_, _> as IntoOwned>::borrow_as(&vec![1; 5]),
            r.index(0)
        );
    }

    #[test]
    fn test_read_slice_cmp() {
        let mut r = <SliceRegion<Vec<u8>>>::default();
        r.push([1; 4]);

        assert_eq!(
            Ordering::Less,
            <ReadSlice<_, _> as IntoOwned>::borrow_as(&vec![0; 4]).cmp(&r.index(0))
        );
        assert_eq!(
            Ordering::Equal,
            <ReadSlice<_, _> as IntoOwned>::borrow_as(&vec![1; 4]).cmp(&r.index(0))
        );
        assert_eq!(
            Ordering::Greater,
            <ReadSlice<_, _> as IntoOwned>::borrow_as(&vec![2; 4]).cmp(&r.index(0))
        );

        assert_eq!(
            Ordering::Less,
            <ReadSlice<_, _> as IntoOwned>::borrow_as(&vec![1; 3]).cmp(&r.index(0))
        );
        assert_eq!(
            Ordering::Equal,
            <ReadSlice<_, _> as IntoOwned>::borrow_as(&vec![1; 4]).cmp(&r.index(0))
        );
        assert_eq!(
            Ordering::Greater,
            <ReadSlice<_, _> as IntoOwned>::borrow_as(&vec![1; 5]).cmp(&r.index(0))
        );
    }

    #[test]
    fn test_reserve_ref_slice() {
        let mut r = <SliceRegion<Vec<u8>>>::default();
        r.reserve_items(std::iter::once([1; 4].as_slice()));
        let mut cap = 0;
        r.heap_size(|_, ca| {
            cap += ca;
        });
        assert!(cap > 0);
    }

    #[test]
    fn test_reserve_ref_vec() {
        let mut r = <SliceRegion<Vec<u8>>>::default();
        r.reserve_items(std::iter::once(&vec![1; 4]));
        let mut cap = 0;
        r.heap_size(|_, ca| {
            cap += ca;
        });
        assert!(cap > 0);
    }

    #[test]
    fn test_reserve_ref_array() {
        let mut r = <SliceRegion<Vec<u8>>>::default();
        r.reserve_items(std::iter::once(&[1; 4]));
        let mut cap = 0;
        r.heap_size(|_, ca| {
            cap += ca;
        });
        assert!(cap > 0);
    }
}
