//! A region that stores slices of copy types.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

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
pub struct CopyRegion<T: Copy> {
    slices: Vec<T>,
}

impl<T: Copy> Region for CopyRegion<T> {
    type ReadItem<'a> = &'a [T] where Self: 'a;
    type Index = (usize, usize);

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
}

impl<T: Copy> Default for CopyRegion<T> {
    fn default() -> Self {
        Self {
            slices: Vec::default(),
        }
    }
}

impl<T> CopyOnto<CopyRegion<T>> for &[T]
where
    T: Copy,
{
    #[inline]
    fn copy_onto(self, target: &mut CopyRegion<T>) -> <CopyRegion<T> as Region>::Index {
        let start = target.slices.len();
        target.slices.extend_from_slice(self);
        (start, target.slices.len())
    }
}

impl<T: Copy> ReserveItems<CopyRegion<T>> for &[T] {
    fn reserve_items<I>(target: &mut CopyRegion<T>, items: I)
    where
        I: Iterator<Item = Self> + Clone,
    {
        target.slices.reserve(items.map(|i| i.len()).sum());
    }
}

impl<T> CopyOnto<CopyRegion<T>> for &Vec<T>
where
    T: Copy,
{
    #[inline]
    fn copy_onto(self, target: &mut CopyRegion<T>) -> <CopyRegion<T> as Region>::Index {
        self.as_slice().copy_onto(target)
    }
}

impl<T: Copy> ReserveItems<CopyRegion<T>> for &Vec<T> {
    fn reserve_items<I>(target: &mut CopyRegion<T>, items: I)
    where
        I: Iterator<Item = Self> + Clone,
    {
        ReserveItems::reserve_items(target, items.map(Vec::as_slice))
    }
}

impl<T, I: IntoIterator<Item = T>> CopyOnto<CopyRegion<T>> for CopyIter<I>
where
    T: Copy,
{
    #[inline]
    fn copy_onto(self, target: &mut CopyRegion<T>) -> <CopyRegion<T> as Region>::Index {
        let start = target.slices.len();
        target.slices.extend(self.0);
        (start, target.slices.len())
    }
}

impl<T: Copy, J: IntoIterator<Item = T>> ReserveItems<CopyRegion<T>> for CopyIter<J> {
    fn reserve_items<I>(target: &mut CopyRegion<T>, items: I)
    where
        I: Iterator<Item = Self> + Clone,
    {
        target
            .slices
            .reserve(items.flat_map(|i| i.0.into_iter()).count());
    }
}
