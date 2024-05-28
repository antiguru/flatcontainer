//! A region that stores slices of copy types.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{CopyIter, Push, Region, ReserveItems};

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
/// use flatcontainer::{Push, OwnedRegion, Region};
/// let mut r = <OwnedRegion<_>>::default();
///
/// let panagram_en = "The quick fox jumps over the lazy dog";
/// let panagram_de = "Zwölf Boxkämpfer jagen Viktor quer über den großen Sylter Deich";
///
/// let en_index = r.push(panagram_en.as_bytes());
/// let de_index = r.push(panagram_de.as_bytes());
///
/// assert_eq!(panagram_de.as_bytes(), r.index(de_index));
/// assert_eq!(panagram_en.as_bytes(), r.index(en_index));
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct OwnedRegion<T> {
    slices: Vec<T>,
}

impl<T> Region for OwnedRegion<T>
where
    [T]: ToOwned,
{
    type Owned = <[T] as ToOwned>::Owned;
    type ReadItem<'a> = &'a [T] where Self: 'a;
    type Index = (usize, usize);

    #[inline]
    fn merge_regions<'a>(regions: impl Iterator<Item = &'a Self> + Clone) -> Self
    where
        Self: 'a,
    {
        Self {
            slices: Vec::with_capacity(regions.map(|r| r.slices.len()).sum()),
        }
    }

    #[inline]
    fn index(&self, (start, end): Self::Index) -> Self::ReadItem<'_> {
        &self.slices[start..end]
    }

    #[inline]
    fn reserve_regions<'a, I>(&mut self, regions: I)
    where
        Self: 'a,
        I: Iterator<Item = &'a Self> + Clone,
    {
        self.slices.reserve(regions.map(|r| r.slices.len()).sum());
    }

    #[inline]
    fn clear(&mut self) {
        self.slices.clear();
    }

    #[inline]
    fn heap_size<F: FnMut(usize, usize)>(&self, mut callback: F) {
        let size_of_t = std::mem::size_of::<T>();
        callback(
            self.slices.len() * size_of_t,
            self.slices.capacity() * size_of_t,
        );
    }

    #[inline]
    fn reborrow<'b, 'a: 'b>(item: Self::ReadItem<'a>) -> Self::ReadItem<'b>
    where
        Self: 'a,
    {
        item
    }
}

impl<T> Default for OwnedRegion<T> {
    #[inline]
    fn default() -> Self {
        Self {
            slices: Vec::default(),
        }
    }
}

impl<T, const N: usize> Push<[T; N]> for OwnedRegion<T>
where
    [T]: ToOwned,
{
    #[inline]
    fn push(&mut self, item: [T; N]) -> <OwnedRegion<T> as Region>::Index {
        let start = self.slices.len();
        self.slices.extend(item);
        (start, self.slices.len())
    }
}

impl<T: Clone, const N: usize> Push<&[T; N]> for OwnedRegion<T> {
    #[inline]
    fn push(&mut self, item: &[T; N]) -> <OwnedRegion<T> as Region>::Index {
        let start = self.slices.len();
        self.slices.extend_from_slice(item);
        (start, self.slices.len())
    }
}

impl<T: Clone, const N: usize> Push<&&[T; N]> for OwnedRegion<T> {
    #[inline]
    fn push(&mut self, item: &&[T; N]) -> <OwnedRegion<T> as Region>::Index {
        self.push(*item)
    }
}

impl<'b, T: Clone, const N: usize> ReserveItems<&'b [T; N]> for OwnedRegion<T> {
    #[inline]
    fn reserve_items<I>(&mut self, items: I)
    where
        I: Iterator<Item = &'b [T; N]> + Clone,
    {
        self.slices.reserve(items.map(|i| i.len()).sum());
    }
}

impl<T: Clone> Push<&[T]> for OwnedRegion<T> {
    #[inline]
    fn push(&mut self, item: &[T]) -> <OwnedRegion<T> as Region>::Index {
        let start = self.slices.len();
        self.slices.extend_from_slice(item);
        (start, self.slices.len())
    }
}

impl<T: Clone> Push<&&[T]> for OwnedRegion<T>
where
    for<'a> Self: Push<&'a [T]>,
{
    #[inline]
    fn push(&mut self, item: &&[T]) -> <OwnedRegion<T> as Region>::Index {
        self.push(*item)
    }
}

impl<'b, T> ReserveItems<&'b [T]> for OwnedRegion<T>
where
    [T]: ToOwned,
{
    #[inline]
    fn reserve_items<I>(&mut self, items: I)
    where
        I: Iterator<Item = &'b [T]> + Clone,
    {
        self.slices.reserve(items.map(<[T]>::len).sum());
    }
}

impl<T> Push<Vec<T>> for OwnedRegion<T>
where
    [T]: ToOwned,
{
    #[inline]
    fn push(&mut self, mut item: Vec<T>) -> <OwnedRegion<T> as Region>::Index {
        let start = self.slices.len();
        self.slices.append(&mut item);
        (start, self.slices.len())
    }
}

impl<T: Clone> Push<&Vec<T>> for OwnedRegion<T> {
    #[inline]
    fn push(&mut self, item: &Vec<T>) -> <OwnedRegion<T> as Region>::Index {
        self.push(item.as_slice())
    }
}

impl<'a, T> ReserveItems<&'a Vec<T>> for OwnedRegion<T>
where
    [T]: ToOwned,
{
    #[inline]
    fn reserve_items<I>(&mut self, items: I)
    where
        I: Iterator<Item = &'a Vec<T>> + Clone,
    {
        self.reserve_items(items.map(Vec::as_slice));
    }
}

impl<T: Clone, I: IntoIterator<Item = T>> Push<CopyIter<I>> for OwnedRegion<T> {
    #[inline]
    fn push(&mut self, item: CopyIter<I>) -> <OwnedRegion<T> as Region>::Index {
        let start = self.slices.len();
        self.slices.extend(item.0);
        (start, self.slices.len())
    }
}

impl<T, J: IntoIterator<Item = T>> ReserveItems<CopyIter<J>> for OwnedRegion<T>
where
    [T]: ToOwned,
{
    #[inline]
    fn reserve_items<I>(&mut self, items: I)
    where
        I: Iterator<Item = CopyIter<J>> + Clone,
    {
        self.slices
            .reserve(items.flat_map(|i| i.0.into_iter()).count());
    }
}

#[cfg(test)]
mod tests {
    use crate::{CopyIter, Push, Region, ReserveItems};

    use super::*;

    #[test]
    fn test_copy_array() {
        let mut r = <OwnedRegion<u8>>::default();
        r.reserve_items(std::iter::once(&[1; 4]));
        let index = r.push([1; 4]);
        assert_eq!([1, 1, 1, 1], r.index(index));
    }

    #[test]
    fn test_copy_ref_ref_array() {
        let mut r = <OwnedRegion<u8>>::default();
        ReserveItems::reserve_items(&mut r, std::iter::once(&[1; 4]));
        let index = r.push(&&[1; 4]);
        assert_eq!([1, 1, 1, 1], r.index(index));
    }

    #[test]
    fn test_copy_vec() {
        let mut r = <OwnedRegion<u8>>::default();
        ReserveItems::reserve_items(&mut r, std::iter::once(&vec![1; 4]));
        let index = r.push(&vec![1; 4]);
        assert_eq!([1, 1, 1, 1], r.index(index));
        let index = r.push(vec![2; 4]);
        assert_eq!([2, 2, 2, 2], r.index(index));
    }

    #[test]
    fn test_copy_iter() {
        let mut r = <OwnedRegion<u8>>::default();
        r.reserve_items(std::iter::once(CopyIter(std::iter::repeat(1).take(4))));
        let index = r.push(CopyIter(std::iter::repeat(1).take(4)));
        assert_eq!([1, 1, 1, 1], r.index(index));
    }
}
