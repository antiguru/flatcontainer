//! A region that stores slices of copy types.

use std::marker::PhantomData;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::impls::storage::Storage;
use crate::{CopyIter, CopyOnto, Region, ReserveItems};

/// A container for [`Copy`] types.
///
/// # Examples
///
/// ```
/// use flatcontainer::{CopyOnto, CopyRegion, Region};
/// let mut r = <CopyRegion<_>>::default();
///
/// let panagram_en = "The quick fox jumps over the lazy dog";
/// let panagram_de = "Zwölf Boxkämpfer jagen Viktor quer über den großen Sylter Deich";
///
/// let en_index = panagram_en.as_bytes().copy_onto(&mut r);
/// let de_index = panagram_de.as_bytes().copy_onto(&mut r);
///
/// assert_eq!(panagram_de.as_bytes(), r.index(de_index));
/// assert_eq!(panagram_en.as_bytes(), r.index(en_index));
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CopyRegion<T: Copy, S: Storage<T> = Vec<T>> {
    slices: S,
    offset: usize,
    _marker: PhantomData<T>,
}

impl<T: Copy, S: Storage<T>> Region for CopyRegion<T, S> {
    type ReadItem<'a> = &'a [T] where Self: 'a;
    type Index = (usize, usize);

    fn merge_regions<'a>(regions: impl Iterator<Item = &'a Self> + Clone) -> Self
    where
        Self: 'a,
    {
        Self {
            slices: S::merge_regions(regions.map(|r| &r.slices)),
            offset: 0,
            _marker: PhantomData,
        }
    }

    #[inline]
    fn index(&self, (start, end): Self::Index) -> Self::ReadItem<'_> {
        self.slices.index(start, end)
    }

    fn reserve_regions<'a, I>(&mut self, regions: I)
    where
        Self: 'a,
        I: Iterator<Item = &'a Self> + Clone,
    {
        self.slices.reserve_regions(regions.map(|r| &r.slices));
    }

    #[inline]
    fn clear(&mut self) {
        self.slices.clear();
    }

    fn heap_size<F: FnMut(usize, usize)>(&self, callback: F) {
        self.slices.heap_size(callback);
    }
}

impl<T: Copy, S: Storage<T>> Default for CopyRegion<T, S> {
    fn default() -> Self {
        Self {
            slices: S::default(),
            offset: 0,
            _marker: PhantomData,
        }
    }
}

impl<T, S: Storage<T>, const N: usize> CopyOnto<CopyRegion<T, S>> for [T; N]
where
    T: Copy,
{
    #[inline]
    fn copy_onto(self, target: &mut CopyRegion<T, S>) -> <CopyRegion<T, S> as Region>::Index {
        (&self).copy_onto(target)
    }
}

impl<T, S: Storage<T>, const N: usize> CopyOnto<CopyRegion<T, S>> for &[T; N]
where
    T: Copy,
{
    #[inline]
    fn copy_onto(self, target: &mut CopyRegion<T, S>) -> <CopyRegion<T, S> as Region>::Index {
        let start = target.offset;
        target.offset = target.slices.extend_from_slice(self);
        (start, target.offset)
    }
}

impl<T, S: Storage<T>, const N: usize> CopyOnto<CopyRegion<T, S>> for &&[T; N]
where
    T: Copy,
{
    #[inline]
    fn copy_onto(self, target: &mut CopyRegion<T, S>) -> <CopyRegion<T, S> as Region>::Index {
        (*self).copy_onto(target)
    }
}

impl<T: Copy, S: Storage<T>, const N: usize> ReserveItems<CopyRegion<T, S>> for &[T; N] {
    fn reserve_items<I>(target: &mut CopyRegion<T, S>, items: I)
    where
        I: Iterator<Item = Self> + Clone,
    {
        target.slices.reserve(items.map(|i| i.len()).sum());
    }
}

impl<T, S: Storage<T>> CopyOnto<CopyRegion<T, S>> for &[T]
where
    T: Copy,
{
    #[inline]
    fn copy_onto(self, target: &mut CopyRegion<T, S>) -> <CopyRegion<T, S> as Region>::Index {
        let start = target.offset;
        target.offset = target.slices.extend_from_slice(self);
        (start, target.offset)
    }
}

impl<T, S: Storage<T>> CopyOnto<CopyRegion<T, S>> for &&[T]
where
    T: Copy,
{
    #[inline]
    fn copy_onto(self, target: &mut CopyRegion<T, S>) -> <CopyRegion<T, S> as Region>::Index {
        (*self).copy_onto(target)
    }
}

impl<T: Copy, S: Storage<T>> ReserveItems<CopyRegion<T, S>> for &[T] {
    fn reserve_items<I>(target: &mut CopyRegion<T, S>, items: I)
    where
        I: Iterator<Item = Self> + Clone,
    {
        target.slices.reserve(items.map(<[T]>::len).sum());
    }
}

impl<T, S: Storage<T>> CopyOnto<CopyRegion<T, S>> for &Vec<T>
where
    T: Copy,
{
    #[inline]
    fn copy_onto(self, target: &mut CopyRegion<T, S>) -> <CopyRegion<T, S> as Region>::Index {
        self.as_slice().copy_onto(target)
    }
}

impl<T: Copy, S: Storage<T>> ReserveItems<CopyRegion<T, S>> for &Vec<T> {
    fn reserve_items<I>(target: &mut CopyRegion<T, S>, items: I)
    where
        I: Iterator<Item = Self> + Clone,
    {
        ReserveItems::reserve_items(target, items.map(Vec::as_slice));
    }
}

impl<T, S: Storage<T>, I: IntoIterator<Item = T>> CopyOnto<CopyRegion<T, S>> for CopyIter<I>
where
    T: Copy,
{
    #[inline]
    fn copy_onto(self, target: &mut CopyRegion<T, S>) -> <CopyRegion<T, S> as Region>::Index {
        let start = target.offset;
        target.offset = target.slices.extend(self.0);
        (start, target.offset)
    }
}

impl<T: Copy, S: Storage<T>, J: IntoIterator<Item = T>> ReserveItems<CopyRegion<T, S>>
    for CopyIter<J>
{
    fn reserve_items<I>(target: &mut CopyRegion<T, S>, items: I)
    where
        I: Iterator<Item = Self> + Clone,
    {
        target
            .slices
            .reserve(items.flat_map(|i| i.0.into_iter()).count());
    }
}
