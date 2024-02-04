use std::ops::Deref;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{CopyOnto, Region, ReserveItems};

/// A container representing slices of data.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SliceRegion<C: Region> {
    slices: Vec<C::Index>,
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

    /// Test if this slice is empty.
    pub fn is_empty(&self) -> bool {
        self.1.is_empty()
    }
}

impl<'a, C: Region> Clone for ReadSlice<'a, C> {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0, self.1)
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

    fn reserve_items<I>(target: &mut SliceRegion<C>, items: I)
    where
        I: Iterator<Item = Self> + Clone,
    {
        target.slices.reserve(items.clone().map(|i| i.len()).sum());
        CopyOnto::reserve_items(&mut target.inner, items.flat_map(|i| i.iter()));
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

    fn reserve_items<I>(target: &mut SliceRegion<C>, items: I)
    where
        I: Iterator<Item = Self> + Clone,
    {
        CopyOnto::reserve_items(target, items.map(Deref::deref))
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

    fn reserve_items<I>(_target: &mut SliceRegion<C>, _items: I)
    where
        I: Iterator<Item = Self> + Clone,
    {
        //CopyOnto::reserve_items(target, items.map(Deref::deref))
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

    fn reserve_items<I>(target: &mut SliceRegion<C>, items: I)
    where
        I: Iterator<Item = Self> + Clone,
    {
        target
            .slices
            .reserve(items.clone().map(|ReadSlice(_c, is)| is.len()).sum());
        CopyOnto::reserve_items(
            &mut target.inner,
            items.flat_map(|ReadSlice(c, is)| is.iter().map(|i| c.index(*i))),
        )
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
