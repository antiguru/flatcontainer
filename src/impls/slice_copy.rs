#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{CopyOnto, Region, ReserveItems};

/// A container for [`Copy`] types.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CopyRegion<T: Copy> {
    slices: Vec<T>,
}

impl<T: Copy> Region for CopyRegion<T> {
    type ReadItem<'a> = &'a [T] where Self: 'a;
    type Index = (usize, usize);

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
    fn copy_onto(self, target: &mut CopyRegion<T>) -> <CopyRegion<T> as Region>::Index {
        let start = target.slices.len();
        target.slices.extend(self);
        (start, target.slices.len())
    }
}

impl<T: Copy> ReserveItems<CopyRegion<T>> for &[T] {
    fn reserve_items<I>(target: &mut CopyRegion<T>, items: I)
    where
        I: Iterator<Item = Self> + Clone,
    {
        target.slices.reserve(items.clone().map(|i| i.len()).sum());
    }
}
