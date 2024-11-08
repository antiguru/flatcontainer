//! A region that stores slices of copy types.

use std::marker::PhantomData;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// use crate::impls::storage::{PushStorage, Storage};
use crate::{
    Clear, HeapSize, Index, IndexAs, Len, Push, PushIter, PushSlice, Region, Reserve, ReserveItems,
};

type Idx = u64;

/// A container for owned types.
///
/// The container can absorb any type, and stores an owned version of the type, similarly to what
/// vectors do. We recommend using this container for copy types, but there is no restriction in
/// the implementation, and in fact it can correctly store owned values, although any data owned
/// by `T` is regular heap-allocated data, and not contained in regions.
///
/// # Examples
///
/// ```
/// use flatcontainer::{Push, OwnedRegion, Region, Index};
/// let mut r = <OwnedRegion<_>>::default();
///
/// let panagram_en = "The quick fox jumps over the lazy dog";
/// let panagram_de = "Zwölf Boxkämpfer jagen Viktor quer über den großen Sylter Deich";
///
/// r.push(panagram_en.as_bytes());
/// r.push(panagram_de.as_bytes());
///
/// assert_eq!(panagram_en.as_bytes(), r.index(0));
/// assert_eq!(panagram_de.as_bytes(), r.index(1));
/// ```
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct OwnedRegion<T, S = Vec<T>, B = Vec<Idx>> {
    slices: S,
    bounds: B,
    _marker: PhantomData<T>,
}

impl<T, S: Clone, B: Clone> Clone for OwnedRegion<T, S, B> {
    fn clone(&self) -> Self {
        Self {
            slices: self.slices.clone(),
            bounds: self.bounds.clone(),
            _marker: PhantomData,
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.slices.clone_from(&source.slices);
        self.bounds.clone_from(&source.bounds);
    }
}

impl<T, S, B> Region for OwnedRegion<T, S, B>
where
    [T]: ToOwned,
    S: Region + std::ops::Index<std::ops::Range<usize>, Output = [T]>,
    B: Region,
{
    #[inline]
    fn merge_regions<'a>(regions: impl Iterator<Item = &'a Self> + Clone) -> Self
    where
        Self: 'a,
    {
        Self {
            slices: S::merge_regions(regions.clone().map(|r| &r.slices)),
            bounds: B::merge_regions(regions.map(|r| &r.bounds)),
            _marker: PhantomData,
        }
    }

    #[inline]
    fn reserve_regions<'a, I>(&mut self, regions: I)
    where
        Self: 'a,
        I: Iterator<Item = &'a Self> + Clone,
    {
        self.slices
            .reserve_regions(regions.clone().map(|r| &r.slices));
        self.bounds.reserve_regions(regions.map(|r| &r.bounds));
    }
}

impl<T, S: Default, B: Default> Default for OwnedRegion<T, S, B> {
    #[inline]
    fn default() -> Self {
        Self {
            slices: S::default(),
            bounds: B::default(),
            _marker: PhantomData,
        }
    }
}

impl<T, S, B> HeapSize for OwnedRegion<T, S, B>
where
    S: HeapSize,
    B: HeapSize,
{
    #[inline]
    fn heap_size<F: FnMut(usize, usize)>(&self, mut callback: F) {
        self.slices.heap_size(&mut callback);
        self.bounds.heap_size(&mut callback);
    }
}

impl<T, S, B> Clear for OwnedRegion<T, S, B>
where
    S: Clear,
    B: Clear,
{
    #[inline]
    fn clear(&mut self) {
        self.slices.clear();
        self.bounds.clear();
    }
}

impl<T, S, B> Len for OwnedRegion<T, S, B>
where
    B: Len,
{
    #[inline]
    fn len(&self) -> usize {
        self.bounds.len()
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.bounds.is_empty()
    }
}

impl<T, S, B> Index for OwnedRegion<T, S, B>
where
    [T]: ToOwned,
    S: std::ops::Index<std::ops::Range<usize>, Output = [T]>,
    B: IndexAs<Idx>,
{
    type Owned = <[T] as ToOwned>::Owned;
    type ReadItem<'a> = &'a [T] where Self: 'a;

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
        &self.slices[start..end]
    }

    #[inline]
    fn reborrow<'b, 'a: 'b>(item: Self::ReadItem<'a>) -> Self::ReadItem<'b>
    where
        Self: 'a,
    {
        item
    }
}

impl<T, S, B, const N: usize> Push<[T; N]> for OwnedRegion<T, S, B>
where
    S: Push<T> + Len,
    B: Push<Idx>,
{
    #[inline]
    fn push(&mut self, items: [T; N]) {
        self.slices.push_extend(items);
        self.bounds
            .push(self.slices.len().try_into().expect("must fit"));
    }
}

impl<T, S, B, const N: usize> Push<&[T; N]> for OwnedRegion<T, S, B>
where
    OwnedRegion<T, S, B>: for<'a> Push<&'a [T]>,
{
    #[inline]
    fn push(&mut self, item: &[T; N]) {
        self.push(item.as_slice())
    }
}

impl<T, S, B, const N: usize> Push<&&[T; N]> for OwnedRegion<T, S, B>
where
    Self: for<'a> Push<&'a [T]>,
{
    #[inline]
    fn push(&mut self, item: &&[T; N]) {
        self.push(*item)
    }
}

impl<'b, T, S, B, const N: usize> ReserveItems<&'b [T; N]> for OwnedRegion<T, S, B>
where
    T: Clone,
    S: Reserve,
    B: Reserve,
{
    #[inline]
    fn reserve_items<I>(&mut self, items: I)
    where
        I: Iterator<Item = &'b [T; N]> + Clone,
    {
        self.slices.reserve(items.map(|i| i.len()).sum());
    }
}

impl<T, S, B> Push<&[T]> for OwnedRegion<T, S, B>
where
    T: Clone,
    S: PushSlice<T> + Len,
    B: Push<Idx>,
{
    #[inline]
    fn push(&mut self, items: &[T]) {
        self.slices.push_slice(items);
        self.bounds
            .push(self.slices.len().try_into().expect("must fit"));
    }
}

impl<T, S> Push<&&[T]> for OwnedRegion<T, S>
where
    T: Clone,
    for<'a> Self: Push<&'a [T]>,
{
    #[inline]
    fn push(&mut self, item: &&[T]) {
        self.push(*item)
    }
}

impl<'b, T, S, B> ReserveItems<&'b [T]> for OwnedRegion<T, S, B>
where
    S: Reserve,
    B: Reserve,
{
    #[inline]
    fn reserve_items<I>(&mut self, items: I)
    where
        I: Iterator<Item = &'b [T]> + Clone,
    {
        self.slices.reserve(items.map(<[T]>::len).sum());
        self.bounds.reserve(1);
    }
}

impl<T, S, B> Push<Vec<T>> for OwnedRegion<T, S, B>
where
    S: Push<T> + Len,
    B: Push<Idx>,
{
    #[inline]
    fn push(&mut self, items: Vec<T>) {
        for item in items {
            self.slices.push(item);
        }
        self.bounds
            .push(self.slices.len().try_into().expect("must fit"));
    }
}

impl<T, S, B> Push<&Vec<T>> for OwnedRegion<T, S, B>
where
    Self: for<'a> Push<&'a [T]>,
{
    #[inline]
    fn push(&mut self, item: &Vec<T>) {
        self.push(item.as_slice())
    }
}

impl<'a, T, S, B> ReserveItems<&'a Vec<T>> for OwnedRegion<T, S, B>
where
    Self: ReserveItems<&'a [T]>,
{
    #[inline]
    fn reserve_items<I>(&mut self, items: I)
    where
        I: Iterator<Item = &'a Vec<T>> + Clone,
    {
        self.reserve_items(items.map(Vec::as_slice));
    }
}

impl<T, S, B, I> Push<PushIter<I>> for OwnedRegion<T, S, B>
where
    I: IntoIterator,
    S: Push<I::Item> + Len,
    B: Push<Idx>,
{
    #[inline]
    fn push(&mut self, items: PushIter<I>) {
        for item in items {
            self.slices.push(item);
        }
        self.bounds
            .push(self.slices.len().try_into().expect("must fit"));
    }
}

impl<T, S, B, J> ReserveItems<PushIter<J>> for OwnedRegion<T, S, B>
where
    S: Reserve,
    J: IntoIterator,
{
    #[inline]
    fn reserve_items<I>(&mut self, items: I)
    where
        I: Iterator<Item = PushIter<J>> + Clone,
    {
        self.slices
            .reserve(items.flat_map(|i| i.into_iter()).count());
    }
}

#[cfg(test)]
mod tests {
    use crate::{Push, PushIter, ReserveItems};

    use super::*;

    #[test]
    fn test_copy_array() {
        let mut r = <OwnedRegion<u8>>::default();
        r.reserve_items(std::iter::once(&[1; 4]));
        assert_eq!(0, r.len());
        r.push([1; 4]);
        assert_eq!([1, 1, 1, 1], r.index(0));
        assert_eq!(1, r.len());
    }

    #[test]
    fn test_copy_ref_ref_array() {
        let mut r = <OwnedRegion<u8>>::default();
        ReserveItems::reserve_items(&mut r, std::iter::once(&[1; 4]));
        r.push(&&[1; 4]);
        assert_eq!([1, 1, 1, 1], r.index(0));
        assert_eq!(1, r.len());
    }

    #[test]
    fn test_copy_vec() {
        let mut r = <OwnedRegion<u8>>::default();
        ReserveItems::reserve_items(&mut r, std::iter::once(&vec![1; 4]));
        r.push(&vec![1; 4]);
        assert_eq!([1, 1, 1, 1], r.index(0));
        assert_eq!(1, r.len());
        r.push(vec![2; 4]);
        assert_eq!([2, 2, 2, 2], r.index(1));
        assert_eq!(2, r.len());
    }

    #[test]
    fn test_copy_iter() {
        let mut r = <OwnedRegion<u8>>::default();
        let iter = [1; 4].into_iter();
        r.reserve_items(std::iter::once(PushIter(iter.clone())));
        r.push(PushIter(iter));
        assert_eq!([1, 1, 1, 1], r.index(0));
        assert_eq!(1, r.len());
    }
}
