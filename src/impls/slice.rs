//! A region that stores slices.

use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, Range};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::impls::offsets::OffsetContainer;
use crate::{Containerized, IntoOwned, Push, ReadRegion, Region, ReserveItems};

impl<T: Containerized> Containerized for Vec<T> {
    type Region = SliceRegion<T::Region>;
}

impl<T: Containerized> Containerized for [T] {
    type Region = SliceRegion<T::Region>;
}

impl<T: Containerized, const N: usize> Containerized for [T; N] {
    type Region = SliceRegion<T::Region>;
}

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
/// use flatcontainer::{Containerized, Push, ReadRegion, Region, SliceRegion};
/// let mut r = <SliceRegion<<String as Containerized>::Region>>::default();
///
/// let panagram_en = "The quick fox jumps over the lazy dog"
///     .split(" ")
///     .collect::<Vec<_>>();
/// let panagram_de = "Zwölf Boxkämpfer jagen Viktor quer über den großen Sylter Deich"
///     .split(" ")
///     .collect::<Vec<_>>();
///
/// let en_index = r.push(&panagram_en);
/// let de_index = r.push(&panagram_de);
///
/// assert!(panagram_de.into_iter().eq(r.index(de_index)));
/// assert!(panagram_en.into_iter().eq(r.index(en_index)));
///
/// assert_eq!(r.index(de_index).get(2), "jagen");
/// ```
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SliceRegion<R: ReadRegion, O = Vec<<R as ReadRegion>::Index>> {
    /// Container of slices.
    slices: O,
    /// Inner region.
    inner: R,
}

impl<R, O> Clone for SliceRegion<R, O>
where
    R: Region + Clone,
    O: Clone,
{
    fn clone(&self) -> Self {
        Self {
            slices: self.slices.clone(),
            inner: self.inner.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.slices.clone_from(&source.slices);
        self.inner.clone_from(&source.inner);
    }
}

impl<R: ReadRegion, O: OffsetContainer<R::Index>> ReadRegion for SliceRegion<R, O> {
    type Owned = Vec<R::Owned>;
    type ReadItem<'a> = ReadSlice<'a, R, O> where Self: 'a;
    type Index = (usize, usize);

    #[inline]
    fn index(&self, (start, end): Self::Index) -> Self::ReadItem<'_> {
        ReadSlice(Ok(ReadSliceInner {
            region: self,
            start,
            end,
        }))
    }
}

impl<R: Region, O: OffsetContainer<R::Index>> Region for SliceRegion<R, O> {
    #[inline]
    fn merge_regions<'a>(regions: impl Iterator<Item = &'a Self> + Clone) -> Self
    where
        Self: 'a,
    {
        Self {
            slices: O::default(),
            inner: R::merge_regions(regions.map(|r| &r.inner)),
        }
    }

    #[inline]
    fn reserve_regions<'a, I>(&mut self, regions: I)
    where
        Self: 'a,
        I: Iterator<Item = &'a Self> + Clone,
    {
        self.slices
            .reserve(regions.clone().map(|r| r.slices.len()).sum());
        self.inner.reserve_regions(regions.map(|r| &r.inner));
    }

    #[inline]
    fn clear(&mut self) {
        self.slices.clear();
        self.inner.clear();
    }

    #[inline]
    fn heap_size<F: FnMut(usize, usize)>(&self, mut callback: F) {
        self.slices.heap_size(&mut callback);
        self.inner.heap_size(callback);
    }

    #[inline]
    fn reborrow<'b, 'a: 'b>(item: Self::ReadItem<'a>) -> Self::ReadItem<'b>
    where
        Self: 'a,
    {
        item
    }
}

impl<R: Region, O: OffsetContainer<R::Index>> Default for SliceRegion<R, O> {
    #[inline]
    fn default() -> Self {
        Self {
            slices: O::default(),
            inner: R::default(),
        }
    }
}

/// A helper to read data out of a slice region.
pub struct ReadSlice<'a, R: ReadRegion, O: OffsetContainer<R::Index>>(
    Result<ReadSliceInner<'a, R, O>, &'a [R::Owned]>,
);

impl<R: ReadRegion, O: OffsetContainer<R::Index>> ReadSlice<'_, R, O> {
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

impl<R: Region, O: OffsetContainer<R::Index>> PartialEq for ReadSlice<'_, R, O>
where
    for<'a> R::ReadItem<'a>: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.iter().eq(*other)
    }
}

impl<R: Region, O: OffsetContainer<R::Index>> Eq for ReadSlice<'_, R, O> where
    for<'a> R::ReadItem<'a>: Eq
{
}

impl<R: Region, O: OffsetContainer<R::Index>> PartialOrd for ReadSlice<'_, R, O>
where
    for<'a> R::ReadItem<'a>: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.iter().partial_cmp(*other)
    }
}

impl<R: Region, O: OffsetContainer<R::Index>> Ord for ReadSlice<'_, R, O>
where
    for<'a> R::ReadItem<'a>: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.iter().cmp(*other)
    }
}

struct ReadSliceInner<'a, R: ReadRegion, O: OffsetContainer<R::Index>> {
    region: &'a SliceRegion<R, O>,
    start: usize,
    end: usize,
}

impl<R: ReadRegion, O: OffsetContainer<R::Index>> ReadSliceInner<'_, R, O> {
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
        self.region
            .inner
            .index(self.region.slices.index(self.start + index))
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

impl<R: ReadRegion, O: OffsetContainer<R::Index>> Debug for ReadSlice<'_, R, O>
where
    for<'a> R::ReadItem<'a>: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<R: ReadRegion, O: OffsetContainer<R::Index>> Clone for ReadSlice<'_, R, O> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<R: ReadRegion, O: OffsetContainer<R::Index>> Clone for ReadSliceInner<'_, R, O> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<R: ReadRegion, O: OffsetContainer<R::Index>> Copy for ReadSlice<'_, R, O> {}
impl<R: ReadRegion, O: OffsetContainer<R::Index>> Copy for ReadSliceInner<'_, R, O> {}

impl<'a, R, O> IntoOwned<'a> for ReadSlice<'a, R, O>
where
    R: ReadRegion,
    O: OffsetContainer<R::Index>,
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

impl<'a, R: ReadRegion, O: OffsetContainer<R::Index>> IntoIterator for ReadSlice<'a, R, O> {
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
pub struct ReadSliceIter<'a, R: ReadRegion, O: OffsetContainer<R::Index>>(
    Result<ReadSliceIterInner<'a, R, O>, std::slice::Iter<'a, R::Owned>>,
);

impl<'a, R: ReadRegion, O: OffsetContainer<R::Index>> Clone for ReadSliceIter<'a, R, O> {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

/// An iterator over the items read from a slice region.
#[derive(Debug)]
pub struct ReadSliceIterInner<'a, R: ReadRegion, O: OffsetContainer<R::Index>>(
    &'a SliceRegion<R, O>,
    Range<usize>,
);

impl<'a, R: ReadRegion, O: OffsetContainer<R::Index>> Clone for ReadSliceIterInner<'a, R, O> {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0, self.1.clone())
    }
}

impl<'a, R: ReadRegion, O: OffsetContainer<R::Index>> Iterator for ReadSliceIter<'a, R, O> {
    type Item = R::ReadItem<'a>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.0 {
            Ok(inner) => inner.next(),
            Err(iter) => iter.next().map(IntoOwned::borrow_as),
        }
    }
}

impl<'a, R: ReadRegion, O: OffsetContainer<R::Index>> Iterator for ReadSliceIterInner<'a, R, O> {
    type Item = R::ReadItem<'a>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.1
            .next()
            .map(|idx| self.0.inner.index(self.0.slices.index(idx)))
    }
}

impl<'a, R, T, O> Push<&'a [T]> for SliceRegion<R, O>
where
    R: Region + Push<&'a T>,
    O: OffsetContainer<R::Index>,
{
    #[inline]
    fn push(&mut self, item: &'a [T]) -> <SliceRegion<R, O> as ReadRegion>::Index {
        let start = self.slices.len();
        self.slices.extend(item.iter().map(|t| self.inner.push(t)));
        (start, self.slices.len())
    }
}

impl<'a, T, R, O> ReserveItems<&'a [T]> for SliceRegion<R, O>
where
    R: Region + ReserveItems<&'a T>,
    O: OffsetContainer<R::Index>,
{
    #[inline]
    fn reserve_items<I>(&mut self, items: I)
    where
        I: Iterator<Item = &'a [T]> + Clone,
    {
        self.slices.reserve(items.clone().map(<[T]>::len).sum());
        self.inner.reserve_items(items.flatten());
    }
}

impl<R, T, O> Push<Vec<T>> for SliceRegion<R, O>
where
    R: Region + Push<T>,
    O: OffsetContainer<R::Index>,
{
    #[inline]
    fn push(&mut self, item: Vec<T>) -> <SliceRegion<R, O> as ReadRegion>::Index {
        let start = self.slices.len();
        self.slices
            .extend(item.into_iter().map(|t| self.inner.push(t)));
        (start, self.slices.len())
    }
}

impl<R, T, O> Push<&Vec<T>> for SliceRegion<R, O>
where
    for<'a> R: Region + Push<&'a T>,
    O: OffsetContainer<R::Index>,
{
    #[inline]
    fn push(&mut self, item: &Vec<T>) -> <SliceRegion<R, O> as ReadRegion>::Index {
        self.push(item.as_slice())
    }
}

impl<'a, R, T, O> Push<&&'a Vec<T>> for SliceRegion<R, O>
where
    R: Region + Push<&'a T>,
    O: OffsetContainer<R::Index>,
{
    #[inline]
    fn push(&mut self, item: &&'a Vec<T>) -> <SliceRegion<R, O> as ReadRegion>::Index {
        self.push(item.as_slice())
    }
}

impl<'a, T, R, O> ReserveItems<&'a Vec<T>> for SliceRegion<R, O>
where
    for<'b> R: Region + ReserveItems<&'b T>,
    O: OffsetContainer<R::Index>,
{
    #[inline]
    fn reserve_items<I>(&mut self, items: I)
    where
        I: Iterator<Item = &'a Vec<T>> + Clone,
    {
        self.reserve_items(items.map(Deref::deref));
    }
}

impl<'a, R, O> Push<ReadSlice<'a, R, O>> for SliceRegion<R, O>
where
    R: Region + Push<<R as ReadRegion>::ReadItem<'a>>,
    O: OffsetContainer<R::Index>,
{
    #[inline]
    fn push(&mut self, item: ReadSlice<'a, R, O>) -> <SliceRegion<R, O> as ReadRegion>::Index {
        match item.0 {
            Ok(inner) => self.push(inner),
            Err(slice) => {
                let start_len = self.slices.len();
                for item in slice.iter().map(IntoOwned::borrow_as) {
                    let index = self.inner.push(item);
                    self.slices.push(index);
                }
                (start_len, self.slices.len())
            }
        }
    }
}

impl<'a, R, O> Push<ReadSliceInner<'a, R, O>> for SliceRegion<R, O>
where
    R: Region + Push<<R as ReadRegion>::ReadItem<'a>>,
    O: OffsetContainer<R::Index>,
{
    #[inline]
    fn push(&mut self, item: ReadSliceInner<'a, R, O>) -> <SliceRegion<R, O> as ReadRegion>::Index {
        let ReadSliceInner { region, start, end } = item;
        let start_len = self.slices.len();
        for index in start..end {
            let index = region.slices.index(index);
            let index = self.inner.push(region.inner.index(index));
            self.slices.push(index);
        }
        (start_len, self.slices.len())
    }
}

impl<T, R, O, const N: usize> Push<[T; N]> for SliceRegion<R, O>
where
    for<'a> R: Region + Push<&'a T>,
    O: OffsetContainer<R::Index>,
{
    #[inline]
    fn push(&mut self, item: [T; N]) -> <SliceRegion<R, O> as ReadRegion>::Index {
        self.push(item.as_slice())
    }
}

impl<'a, T, R, O, const N: usize> Push<&'a [T; N]> for SliceRegion<R, O>
where
    R: Region + Push<&'a T>,
    O: OffsetContainer<R::Index>,
{
    #[inline]
    fn push(&mut self, item: &'a [T; N]) -> <SliceRegion<R, O> as ReadRegion>::Index {
        self.push(item.as_slice())
    }
}

impl<'a, T, R, O, const N: usize> Push<&&'a [T; N]> for SliceRegion<R, O>
where
    R: Region + Push<&'a T>,
    O: OffsetContainer<R::Index>,
{
    #[inline]
    fn push(&mut self, item: &&'a [T; N]) -> <SliceRegion<R, O> as ReadRegion>::Index {
        self.push(item.as_slice())
    }
}

impl<'a, T, R, O, const N: usize> ReserveItems<&'a [T; N]> for SliceRegion<R, O>
where
    R: Region + ReserveItems<&'a T>,
    O: OffsetContainer<R::Index>,
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
    R: Region + ReserveItems<<R as ReadRegion>::ReadItem<'a>> + 'a,
    O: OffsetContainer<R::Index>,
{
    fn reserve_items<I>(&mut self, items: I)
    where
        I: Iterator<Item = ReadSlice<'a, R, O>> + Clone,
    {
        self.slices
            .reserve(items.clone().map(|read_slice| read_slice.len()).sum());
        self.inner.reserve_items(items.flatten());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{MirrorRegion, Push, Region};

    #[test]
    fn read_slice() {
        let s = [1, 2, 3, 4];
        let mut r = <SliceRegion<MirrorRegion<u8>>>::default();

        let index = r.push(s);

        assert!(s.iter().copied().eq(r.index(index).iter()));

        let index = r.push(s);
        let slice = r.index(index);
        assert_eq!(s.len(), slice.len());
        assert!(!slice.is_empty());
        assert_eq!(s.get(0), Some(&1));
        assert_eq!(s.get(1), Some(&2));
        assert_eq!(s.get(2), Some(&3));
        assert_eq!(s.get(3), Some(&4));

        let index = <_ as Push<[u8; 0]>>::push(&mut r, []);
        let slice = r.index(index);
        assert_eq!(0, slice.len());
        assert!(slice.is_empty());
    }

    #[test]
    #[should_panic]
    fn test_get_out_of_bounds() {
        let mut r = <SliceRegion<MirrorRegion<u8>>>::default();
        let index = r.push([1; 4]);

        // Offset 4 is out of bounds and expected to panic.
        let _ = r.index(index).get(4);
    }

    #[test]
    fn test_read_slice_debug() {
        let mut r = <SliceRegion<MirrorRegion<u8>>>::default();
        let index = r.push([1; 4]);

        assert_eq!("[1, 1, 1, 1]", format!("{:?}", r.index(index)));
    }

    #[test]
    fn test_read_slice_clone() {
        let mut r = <SliceRegion<MirrorRegion<u8>>>::default();
        let index = r.push([1; 4]);

        assert_eq!("[1, 1, 1, 1]", format!("{:?}", r.index(index).clone()));
    }

    #[test]
    fn test_read_slice_eq() {
        let mut r = <SliceRegion<MirrorRegion<u8>>>::default();
        let index = r.push([1; 4]);

        assert_eq!(
            <ReadSlice<_, _> as IntoOwned>::borrow_as(&vec![1; 4]),
            r.index(index)
        );
        assert_ne!(
            <ReadSlice<_, _> as IntoOwned>::borrow_as(&vec![0; 4]),
            r.index(index)
        );
        assert_ne!(
            <ReadSlice<_, _> as IntoOwned>::borrow_as(&vec![1; 5]),
            r.index(index)
        );
    }

    #[test]
    fn test_read_slice_cmp() {
        let mut r = <SliceRegion<MirrorRegion<u8>>>::default();
        let index = r.push([1; 4]);

        assert_eq!(
            Ordering::Less,
            <ReadSlice<_, _> as IntoOwned>::borrow_as(&vec![0; 4]).cmp(&r.index(index))
        );
        assert_eq!(
            Ordering::Equal,
            <ReadSlice<_, _> as IntoOwned>::borrow_as(&vec![1; 4]).cmp(&r.index(index))
        );
        assert_eq!(
            Ordering::Greater,
            <ReadSlice<_, _> as IntoOwned>::borrow_as(&vec![2; 4]).cmp(&r.index(index))
        );

        assert_eq!(
            Ordering::Less,
            <ReadSlice<_, _> as IntoOwned>::borrow_as(&vec![1; 3]).cmp(&r.index(index))
        );
        assert_eq!(
            Ordering::Equal,
            <ReadSlice<_, _> as IntoOwned>::borrow_as(&vec![1; 4]).cmp(&r.index(index))
        );
        assert_eq!(
            Ordering::Greater,
            <ReadSlice<_, _> as IntoOwned>::borrow_as(&vec![1; 5]).cmp(&r.index(index))
        );
    }

    #[test]
    fn test_reserve_ref_slice() {
        let mut r = <SliceRegion<MirrorRegion<u8>>>::default();
        r.reserve_items(std::iter::once([1; 4].as_slice()));
        let mut cap = 0;
        r.heap_size(|_, ca| {
            cap += ca;
        });
        assert!(cap > 0);
    }

    #[test]
    fn test_reserve_ref_vec() {
        let mut r = <SliceRegion<MirrorRegion<u8>>>::default();
        r.reserve_items(std::iter::once(&vec![1; 4]));
        let mut cap = 0;
        r.heap_size(|_, ca| {
            cap += ca;
        });
        assert!(cap > 0);
    }

    #[test]
    fn test_reserve_ref_array() {
        let mut r = <SliceRegion<MirrorRegion<u8>>>::default();
        r.reserve_items(std::iter::once(&[1; 4]));
        let mut cap = 0;
        r.heap_size(|_, ca| {
            cap += ca;
        });
        assert!(cap > 0);
    }
}
