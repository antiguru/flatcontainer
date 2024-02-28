//! A region that stores slices.

use std::fmt::{Debug, Formatter};
use std::ops::{Deref, Range};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::impls::offsets::OffsetContainer;
use crate::{Containerized, CopyOnto, Region, ReserveItems};

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
/// use flatcontainer::{Containerized, CopyOnto, Region, SliceRegion};
/// let mut r = <SliceRegion<<String as Containerized>::Region>>::default();
///
/// let panagram_en = "The quick fox jumps over the lazy dog"
///     .split(" ")
///     .collect::<Vec<_>>();
/// let panagram_de = "Zwölf Boxkämpfer jagen Viktor quer über den großen Sylter Deich"
///     .split(" ")
///     .collect::<Vec<_>>();
///
/// let en_index = (&panagram_en).copy_onto(&mut r);
/// let de_index = (&panagram_de).copy_onto(&mut r);
///
/// assert!(panagram_de.into_iter().eq(r.index(de_index)));
/// assert!(panagram_en.into_iter().eq(r.index(en_index)));
///
/// assert_eq!(r.index(de_index).get(2), "jagen");
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SliceRegion<C: Region, O: OffsetContainer<C::Index> = Vec<<C as Region>::Index>> {
    /// Container of slices.
    slices: O,
    /// Inner region.
    inner: C,
}

impl<C: Region, O: OffsetContainer<C::Index>> Region for SliceRegion<C, O> {
    type ReadItem<'a> = ReadSlice<'a, C, O> where Self: 'a;
    type Index = (usize, usize);

    fn merge_regions<'a>(regions: impl Iterator<Item = &'a Self> + Clone) -> Self
    where
        Self: 'a,
    {
        Self {
            slices: O::default(),
            inner: C::merge_regions(regions.map(|r| &r.inner)),
        }
    }

    #[inline]
    fn index(&self, (start, end): Self::Index) -> Self::ReadItem<'_> {
        ReadSlice {
            region: self,
            start,
            end,
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

    fn heap_size<F: FnMut(usize, usize)>(&self, mut callback: F) {
        self.slices.heap_size(&mut callback);
        self.inner.heap_size(callback);
    }
}

impl<C: Region, O: OffsetContainer<C::Index>> Default for SliceRegion<C, O> {
    fn default() -> Self {
        Self {
            slices: O::default(),
            inner: C::default(),
        }
    }
}

/// A helper to read data out of a slice region.
pub struct ReadSlice<'a, C: Region, O: OffsetContainer<C::Index> = Vec<<C as Region>::Index>> {
    region: &'a SliceRegion<C, O>,
    start: usize,
    end: usize,
}

impl<'a, C: Region, O: OffsetContainer<C::Index>> ReadSlice<'a, C, O> {
    /// Read the n-th item from the underlying region.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds, i.e., it is larger than the
    /// length of this slice representation.
    #[inline]
    #[must_use]
    pub fn get(&self, index: usize) -> C::ReadItem<'_> {
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

    /// Returns an iterator over all contained items.
    #[must_use]
    pub fn iter(&self) -> <Self as IntoIterator>::IntoIter {
        self.into_iter()
    }
}

impl<'a, C: Region, O: OffsetContainer<C::Index>> Debug for ReadSlice<'a, C, O>
where
    C::ReadItem<'a>: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<'a, C: Region, O: OffsetContainer<C::Index>> Clone for ReadSlice<'a, C, O> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, C: Region, O: OffsetContainer<C::Index>> Copy for ReadSlice<'a, C, O> {}

impl<'a, C: Region, O: OffsetContainer<C::Index>> IntoIterator for ReadSlice<'a, C, O> {
    type Item = C::ReadItem<'a>;
    type IntoIter = ReadSliceIter<'a, C, O>;

    fn into_iter(self) -> Self::IntoIter {
        ReadSliceIter(self.region, self.start..self.end)
    }
}

/// An iterator over the items read from a slice region.
#[derive(Debug, Clone)]
pub struct ReadSliceIter<'a, C: Region, O: OffsetContainer<C::Index>>(
    &'a SliceRegion<C, O>,
    Range<usize>,
);

impl<'a, C: Region, O: OffsetContainer<C::Index>> Iterator for ReadSliceIter<'a, C, O> {
    type Item = C::ReadItem<'a>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.1
            .next()
            .map(|idx| self.0.inner.index(self.0.slices.index(idx)))
    }
}

impl<'a, C, T: 'a, O: OffsetContainer<C::Index>> CopyOnto<SliceRegion<C, O>> for &'a [T]
where
    C: Region,
    &'a T: CopyOnto<C>,
{
    #[inline]
    fn copy_onto(self, target: &mut SliceRegion<C, O>) -> <SliceRegion<C, O> as Region>::Index {
        let start = target.slices.len();
        target
            .slices
            .extend(self.iter().map(|t| t.copy_onto(&mut target.inner)));
        (start, target.slices.len())
    }
}

impl<'a, T, R: Region, O: OffsetContainer<R::Index>> ReserveItems<SliceRegion<R, O>> for &'a [T]
where
    &'a T: ReserveItems<R> + 'a,
{
    fn reserve_items<I>(target: &mut SliceRegion<R, O>, items: I)
    where
        I: Iterator<Item = Self> + Clone,
    {
        target.slices.reserve(items.clone().map(<[T]>::len).sum());
        ReserveItems::reserve_items(&mut target.inner, items.flatten());
    }
}

impl<'a, C, T, O: OffsetContainer<C::Index>> CopyOnto<SliceRegion<C, O>> for &'a Vec<T>
where
    C: Region,
    &'a [T]: CopyOnto<SliceRegion<C, O>>,
{
    #[inline]
    fn copy_onto(self, target: &mut SliceRegion<C, O>) -> <SliceRegion<C, O> as Region>::Index {
        self.as_slice().copy_onto(target)
    }
}

impl<'a, T: 'a, R: Region, O: OffsetContainer<R::Index>> ReserveItems<SliceRegion<R, O>>
    for &'a Vec<T>
where
    &'a T: ReserveItems<R>,
{
    fn reserve_items<I>(target: &mut SliceRegion<R, O>, items: I)
    where
        I: Iterator<Item = Self> + Clone,
    {
        ReserveItems::reserve_items(target, items.map(Deref::deref));
    }
}

impl<C, T, O: OffsetContainer<C::Index>> CopyOnto<SliceRegion<C, O>> for Vec<T>
where
    C: Region,
    T: CopyOnto<C>,
{
    #[inline]
    fn copy_onto(self, target: &mut SliceRegion<C, O>) -> <SliceRegion<C, O> as Region>::Index {
        let start = target.slices.len();
        target
            .slices
            .extend(self.into_iter().map(|t| t.copy_onto(&mut target.inner)));
        (start, target.slices.len())
    }
}

impl<'a, C: Region + 'a, O: OffsetContainer<C::Index>> CopyOnto<SliceRegion<C, O>>
    for ReadSlice<'a, C, O>
where
    C::ReadItem<'a>: CopyOnto<C>,
{
    #[inline]
    fn copy_onto(self, target: &mut SliceRegion<C, O>) -> <SliceRegion<C, O> as Region>::Index {
        let ReadSlice { region, start, end } = self;
        let start_len = target.slices.len();
        for index in start..end {
            let index = region.slices.index(index);
            let index = region.inner.index(index).copy_onto(&mut target.inner);
            target.slices.push(index);
        }
        (start_len, target.slices.len())
    }
}

impl<'a, T, R: Region, O: OffsetContainer<R::Index>, const N: usize> CopyOnto<SliceRegion<R, O>>
    for &'a [T; N]
where
    for<'b> &'b [T]: CopyOnto<SliceRegion<R, O>>,
{
    #[inline]
    fn copy_onto(self, target: &mut SliceRegion<R, O>) -> <SliceRegion<R, O> as Region>::Index {
        self.as_slice().copy_onto(target)
    }
}

impl<'a, T: 'a, R: Region, O: OffsetContainer<R::Index>, const N: usize>
    ReserveItems<SliceRegion<R, O>> for &'a [T; N]
where
    &'a T: ReserveItems<R>,
{
    fn reserve_items<I>(target: &mut SliceRegion<R, O>, items: I)
    where
        I: Iterator<Item = Self> + Clone,
    {
        ReserveItems::reserve_items(target, items.map(<[T; N]>::as_slice));
    }
}

impl<T, R: Region, O: OffsetContainer<R::Index>, const N: usize> CopyOnto<SliceRegion<R, O>>
    for [T; N]
where
    for<'a> &'a [T]: CopyOnto<SliceRegion<R, O>>,
{
    #[inline]
    fn copy_onto(self, target: &mut SliceRegion<R, O>) -> <SliceRegion<R, O> as Region>::Index {
        self.as_slice().copy_onto(target)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CopyOnto, MirrorRegion, Region};

    #[test]
    fn read_slice() {
        let s = [1, 2, 3, 4];
        let mut r = <SliceRegion<MirrorRegion<u8>>>::default();

        let index = s.copy_onto(&mut r);

        assert!(s.iter().copied().eq(r.index(index).iter()));

        let index = s.copy_onto(&mut r);
        let slice = r.index(index);
        assert_eq!(s.len(), slice.len());
        assert!(!slice.is_empty());
        assert_eq!(s.get(0), Some(&1));
        assert_eq!(s.get(1), Some(&2));
        assert_eq!(s.get(2), Some(&3));
        assert_eq!(s.get(3), Some(&4));

        let index = <[u8; 0] as CopyOnto<_>>::copy_onto([], &mut r);
        let slice = r.index(index);
        assert_eq!(0, slice.len());
        assert!(slice.is_empty());
    }

    #[test]
    #[should_panic]
    fn test_get_out_of_bounds() {
        let mut r = <SliceRegion<MirrorRegion<u8>>>::default();
        let index = [1; 4].copy_onto(&mut r);

        // Offset 4 is out of bounds and expected to panic.
        let _ = r.index(index).get(4);
    }

    #[test]
    fn test_read_slice_debug() {
        let mut r = <SliceRegion<MirrorRegion<u8>>>::default();
        let index = [1; 4].copy_onto(&mut r);

        assert_eq!("[1, 1, 1, 1]", format!("{:?}", r.index(index)));
    }

    #[test]
    fn test_read_slice_clone() {
        let mut r = <SliceRegion<MirrorRegion<u8>>>::default();
        let index = [1; 4].copy_onto(&mut r);

        assert_eq!("[1, 1, 1, 1]", format!("{:?}", r.index(index).clone()));
    }

    #[test]
    fn test_reserve_ref_slice() {
        let mut r = <SliceRegion<MirrorRegion<u8>>>::default();
        ReserveItems::reserve_items(&mut r, std::iter::once([1; 4].as_slice()));
        let mut cap = 0;
        r.heap_size(|_, ca| {
            cap += ca;
        });
        assert!(cap > 0);
    }

    #[test]
    fn test_reserve_ref_vec() {
        let mut r = <SliceRegion<MirrorRegion<u8>>>::default();
        ReserveItems::reserve_items(&mut r, std::iter::once(&vec![1; 4]));
        let mut cap = 0;
        r.heap_size(|_, ca| {
            cap += ca;
        });
        assert!(cap > 0);
    }

    #[test]
    fn test_reserve_ref_array() {
        let mut r = <SliceRegion<MirrorRegion<u8>>>::default();
        ReserveItems::reserve_items(&mut r, std::iter::once(&[1; 4]));
        let mut cap = 0;
        r.heap_size(|_, ca| {
            cap += ca;
        });
        assert!(cap > 0);
    }
}
