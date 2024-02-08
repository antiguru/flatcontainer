//! A region that stores slices.

use std::ops::Deref;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

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
/// let mut r = SliceRegion::<<String as Containerized>::Region>::default();
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
pub struct SliceRegion<C: Region> {
    /// Container of slices.
    slices: Vec<C::Index>,
    /// Inner region.
    inner: C,
}

impl<C: Region> Region for SliceRegion<C> {
    type ReadItem<'a> = ReadSlice<'a, C> where Self: 'a;
    type Index = (usize, usize);

    #[inline]
    fn index(&self, (start, end): Self::Index) -> Self::ReadItem<'_> {
        let slice = &self.slices[start..end];
        ReadSlice(&self.inner, slice)
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
}

impl<C: Region> Default for SliceRegion<C> {
    fn default() -> Self {
        Self {
            slices: Vec::default(),
            inner: C::default(),
        }
    }
}

/// A helper to read data out of a slice region.
#[derive(Debug)]
pub struct ReadSlice<'a, C: Region>(pub &'a C, pub &'a [C::Index]);

impl<'a, C: Region> ReadSlice<'a, C> {
    /// Read the n-th item from the underlying region.
    #[inline]
    pub fn get(&self, index: usize) -> C::ReadItem<'_> {
        self.0.index(self.1[index])
    }

    /// The number in this slice.
    pub fn len(&self) -> usize {
        self.1.len()
    }

    /// Returns `true` if the slice is empty.
    pub fn is_empty(&self) -> bool {
        self.1.is_empty()
    }

    /// Returns an iterator over all contained items.
    pub fn iter(&self) -> <Self as IntoIterator>::IntoIter {
        self.into_iter()
    }
}

impl<'a, C: Region> Clone for ReadSlice<'a, C> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, C: Region> Copy for ReadSlice<'a, C> {}

impl<'a, C: Region> IntoIterator for ReadSlice<'a, C> {
    type Item = C::ReadItem<'a>;
    type IntoIter = ReadSliceIter<'a, C>;

    fn into_iter(self) -> Self::IntoIter {
        ReadSliceIter(self.0, self.1.iter())
    }
}

/// An iterator over the items read from a slice region.
#[derive(Debug, Clone)]
pub struct ReadSliceIter<'a, C: Region>(&'a C, std::slice::Iter<'a, C::Index>);

impl<'a, C: Region> Iterator for ReadSliceIter<'a, C> {
    type Item = C::ReadItem<'a>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|idx| self.0.index(*idx))
    }
}

impl<'a, C, T: 'a> CopyOnto<SliceRegion<C>> for &'a [T]
where
    C: Region,
    &'a T: CopyOnto<C>,
{
    #[inline]
    fn copy_onto(self, target: &mut SliceRegion<C>) -> <SliceRegion<C> as Region>::Index {
        let start = target.slices.len();
        target
            .slices
            .extend(self.iter().map(|t| t.copy_onto(&mut target.inner)));
        (start, target.slices.len())
    }
}

impl<'a, T, R: Region> ReserveItems<SliceRegion<R>> for &'a [T]
where
    &'a T: ReserveItems<R> + 'a,
{
    fn reserve_items<I>(target: &mut SliceRegion<R>, items: I)
    where
        I: Iterator<Item = Self> + Clone,
    {
        target.slices.reserve(items.clone().map(|i| i.len()).sum());
        ReserveItems::reserve_items(&mut target.inner, items.flat_map(|i| i.iter()));
    }
}

impl<'a, C, T> CopyOnto<SliceRegion<C>> for &'a Vec<T>
where
    C: Region,
    &'a [T]: CopyOnto<SliceRegion<C>>,
{
    #[inline]
    fn copy_onto(self, target: &mut SliceRegion<C>) -> <SliceRegion<C> as Region>::Index {
        self.as_slice().copy_onto(target)
    }
}

impl<'a, T: 'a, R: Region> ReserveItems<SliceRegion<R>> for &'a Vec<T>
where
    &'a T: ReserveItems<R>,
{
    fn reserve_items<I>(target: &mut SliceRegion<R>, items: I)
    where
        I: Iterator<Item = Self> + Clone,
    {
        ReserveItems::reserve_items(target, items.map(Deref::deref))
    }
}

impl<C, T> CopyOnto<SliceRegion<C>> for Vec<T>
where
    C: Region,
    for<'a> &'a [T]: CopyOnto<SliceRegion<C>>,
{
    #[inline]
    fn copy_onto(self, target: &mut SliceRegion<C>) -> <SliceRegion<C> as Region>::Index {
        self.as_slice().copy_onto(target)
    }
}

impl<'a, C: Region + 'a> CopyOnto<SliceRegion<C>> for ReadSlice<'a, C>
where
    C::ReadItem<'a>: CopyOnto<C>,
{
    #[inline]
    fn copy_onto(self, target: &mut SliceRegion<C>) -> <SliceRegion<C> as Region>::Index {
        let ReadSlice(container, indexes) = self;
        let start = target.slices.len();
        target.slices.extend(
            indexes
                .iter()
                .map(|&index| container.index(index).copy_onto(&mut target.inner)),
        );
        (start, target.slices.len())
    }
}

impl<'a, T, R: Region, const N: usize> CopyOnto<SliceRegion<R>> for &'a [T; N]
where
    for<'b> &'b [T]: CopyOnto<SliceRegion<R>>,
{
    #[inline]
    fn copy_onto(self, target: &mut SliceRegion<R>) -> <SliceRegion<R> as Region>::Index {
        self.as_slice().copy_onto(target)
    }
}

impl<'a, T: 'a, R: Region, const N: usize> ReserveItems<SliceRegion<R>> for &'a [T; N]
where
    &'a T: ReserveItems<R>,
{
    fn reserve_items<I>(target: &mut SliceRegion<R>, items: I)
    where
        I: Iterator<Item = Self> + Clone,
    {
        ReserveItems::reserve_items(target, items.map(|item| item.as_slice()))
    }
}

impl<T, R: Region, const N: usize> CopyOnto<SliceRegion<R>> for [T; N]
where
    for<'a> &'a [T]: CopyOnto<SliceRegion<R>>,
{
    #[inline]
    fn copy_onto(self, target: &mut SliceRegion<R>) -> <SliceRegion<R> as Region>::Index {
        self.as_slice().copy_onto(target)
    }
}

impl<'a, C: Region + 'a> ReserveItems<SliceRegion<C>> for &'a (C, &'a [C::Index])
where
    C::ReadItem<'a>: ReserveItems<C>,
{
    fn reserve_items<I>(target: &mut SliceRegion<C>, items: I)
    where
        I: Iterator<Item = Self> + Clone,
    {
        target
            .slices
            .reserve(items.clone().map(|(_c, is)| is.len()).sum());
        ReserveItems::reserve_items(
            &mut target.inner,
            items.flat_map(|(c, is)| is.iter().map(|i| c.index(*i))),
        )
    }
}
