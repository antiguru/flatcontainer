//! A region that stores slices of copy types.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{CopyIter, CopyOnto, FlatRead, FlatWrite, Flatten, ReadRegion, Region, ReserveItems};

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
/// use flatcontainer::{CopyOnto, OwnedRegion, Region};
/// let mut r = <OwnedRegion<_>>::default();
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
pub struct OwnedRegion<T> {
    slices: Vec<T>,
}

impl<T> ReadRegion for OwnedRegion<T> {
    type ReadItem<'a> = &'a [T] where Self: 'a;
    type Index = (usize, usize);

    #[inline]
    fn index(&self, (start, end): Self::Index) -> Self::ReadItem<'_> {
        &self.slices[start..end]
    }
}

impl<T> Region for OwnedRegion<T> {
    fn merge_regions<'a>(regions: impl Iterator<Item = &'a Self> + Clone) -> Self
    where
        Self: 'a,
    {
        Self {
            slices: Vec::with_capacity(regions.map(|r| r.slices.len()).sum()),
        }
    }

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

    fn heap_size<F: FnMut(usize, usize)>(&self, mut callback: F) {
        let size_of_t = std::mem::size_of::<T>();
        callback(
            self.slices.len() * size_of_t,
            self.slices.capacity() * size_of_t,
        );
    }
}

impl<T> Default for OwnedRegion<T> {
    fn default() -> Self {
        Self {
            slices: Vec::default(),
        }
    }
}

impl<T, const N: usize> CopyOnto<OwnedRegion<T>> for [T; N] {
    #[inline]
    fn copy_onto(self, target: &mut OwnedRegion<T>) -> <OwnedRegion<T> as ReadRegion>::Index {
        let start = target.slices.len();
        target.slices.extend(self);
        (start, target.slices.len())
    }
}

impl<T: Clone, const N: usize> CopyOnto<OwnedRegion<T>> for &[T; N] {
    #[inline]
    fn copy_onto(self, target: &mut OwnedRegion<T>) -> <OwnedRegion<T> as ReadRegion>::Index {
        let start = target.slices.len();
        target.slices.extend_from_slice(self);
        (start, target.slices.len())
    }
}

impl<T: Clone, const N: usize> CopyOnto<OwnedRegion<T>> for &&[T; N] {
    #[inline]
    fn copy_onto(self, target: &mut OwnedRegion<T>) -> <OwnedRegion<T> as ReadRegion>::Index {
        (*self).copy_onto(target)
    }
}

impl<T: Clone, const N: usize> ReserveItems<OwnedRegion<T>> for &[T; N] {
    fn reserve_items<I>(target: &mut OwnedRegion<T>, items: I)
    where
        I: Iterator<Item = Self> + Clone,
    {
        target.slices.reserve(items.map(|i| i.len()).sum());
    }
}

impl<T: Clone> CopyOnto<OwnedRegion<T>> for &[T] {
    #[inline]
    fn copy_onto(self, target: &mut OwnedRegion<T>) -> <OwnedRegion<T> as ReadRegion>::Index {
        let start = target.slices.len();
        target.slices.extend_from_slice(self);
        (start, target.slices.len())
    }
}

impl<T: Clone> CopyOnto<OwnedRegion<T>> for &&[T]
where
    for<'a> &'a [T]: CopyOnto<OwnedRegion<T>>,
{
    #[inline]
    fn copy_onto(self, target: &mut OwnedRegion<T>) -> <OwnedRegion<T> as ReadRegion>::Index {
        (*self).copy_onto(target)
    }
}

impl<T> ReserveItems<OwnedRegion<T>> for &[T] {
    fn reserve_items<I>(target: &mut OwnedRegion<T>, items: I)
    where
        I: Iterator<Item = Self> + Clone,
    {
        target.slices.reserve(items.map(<[T]>::len).sum());
    }
}

impl<T> CopyOnto<OwnedRegion<T>> for Vec<T> {
    #[inline]
    fn copy_onto(mut self, target: &mut OwnedRegion<T>) -> <OwnedRegion<T> as ReadRegion>::Index {
        let start = target.slices.len();
        target.slices.append(&mut self);
        (start, target.slices.len())
    }
}

impl<T: Clone> CopyOnto<OwnedRegion<T>> for &Vec<T> {
    #[inline]
    fn copy_onto(self, target: &mut OwnedRegion<T>) -> <OwnedRegion<T> as ReadRegion>::Index {
        self.as_slice().copy_onto(target)
    }
}

impl<T> ReserveItems<OwnedRegion<T>> for &Vec<T> {
    fn reserve_items<I>(target: &mut OwnedRegion<T>, items: I)
    where
        I: Iterator<Item = Self> + Clone,
    {
        ReserveItems::reserve_items(target, items.map(Vec::as_slice));
    }
}

impl<T: Clone, I: IntoIterator<Item = T>> CopyOnto<OwnedRegion<T>> for CopyIter<I> {
    #[inline]
    fn copy_onto(self, target: &mut OwnedRegion<T>) -> <OwnedRegion<T> as ReadRegion>::Index {
        let start = target.slices.len();
        target.slices.extend(self.0);
        (start, target.slices.len())
    }
}

impl<T, J: IntoIterator<Item = T>> ReserveItems<OwnedRegion<T>> for CopyIter<J> {
    fn reserve_items<I>(target: &mut OwnedRegion<T>, items: I)
    where
        I: Iterator<Item = Self> + Clone,
    {
        target
            .slices
            .reserve(items.flat_map(|i| i.0.into_iter()).count());
    }
}

impl<T: Copy + 'static> Flatten for OwnedRegion<T> {
    type Flat<'a> = BorrowedRegion<'a, T>;
    fn entomb<W: FlatWrite>(&self, write: &mut W) -> std::io::Result<()> {
        write.write_lengthened(&self.slices)
    }

    fn exhume<'a, R: FlatRead<'a>>(buffer: &'a mut R) -> std::io::Result<Self::Flat<'a>> {
        Ok(BorrowedRegion {
            slices: buffer.read_lengthened()?,
        })
    }
}

/// TODO
pub struct BorrowedRegion<'a, T> {
    slices: &'a [T],
}

impl<'data, T> ReadRegion for BorrowedRegion<'data, T> {
    type ReadItem<'a> = &'a [T] where Self: 'a;
    type Index = (usize, usize);

    #[inline]
    fn index(&self, (start, end): Self::Index) -> Self::ReadItem<'_> {
        &self.slices[start..end]
    }
}

#[cfg(test)]
mod tests {
    use crate::{CopyIter, CopyOnto, ReserveItems};

    use super::*;

    #[test]
    fn test_copy_array() {
        let mut r = <OwnedRegion<u8>>::default();
        ReserveItems::reserve_items(&mut r, std::iter::once(&[1; 4]));
        let index = [1; 4].copy_onto(&mut r);
        assert_eq!([1, 1, 1, 1], r.index(index));
    }

    #[test]
    fn test_copy_ref_ref_array() {
        let mut r = <OwnedRegion<u8>>::default();
        ReserveItems::reserve_items(&mut r, std::iter::once(&[1; 4]));
        let index = (&&[1; 4]).copy_onto(&mut r);
        assert_eq!([1, 1, 1, 1], r.index(index));
    }

    #[test]
    fn test_copy_vec() {
        let mut r = <OwnedRegion<u8>>::default();
        ReserveItems::reserve_items(&mut r, std::iter::once(&vec![1; 4]));
        let index = (&vec![1; 4]).copy_onto(&mut r);
        assert_eq!([1, 1, 1, 1], r.index(index));
        let index = vec![2; 4].copy_onto(&mut r);
        assert_eq!([2, 2, 2, 2], r.index(index));
    }

    #[test]
    fn test_copy_iter() {
        let mut r = <OwnedRegion<u8>>::default();
        ReserveItems::reserve_items(
            &mut r,
            std::iter::once(CopyIter(std::iter::repeat(1).take(4))),
        );
        let index = CopyIter(std::iter::repeat(1).take(4)).copy_onto(&mut r);
        assert_eq!([1, 1, 1, 1], r.index(index));
    }
}
