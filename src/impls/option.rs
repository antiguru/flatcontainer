#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{Containerized, CopyOnto, Region, ReserveItems};

impl<T: Containerized> Containerized for Option<T> {
    type Region = OptionRegion<T::Region>;
}

/// A region to hold [`Option`]s.
#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct OptionRegion<T> {
    inner: T,
}

impl<T> Region for OptionRegion<T>
where
    T: Region,
{
    type ReadItem<'a> = Option<T::ReadItem<'a>> where Self: 'a;
    type Index = Option<T::Index>;

    #[inline]
    fn index(&self, index: Self::Index) -> Self::ReadItem<'_> {
        index.map(|t| self.inner.index(t))
    }

    #[inline]
    fn reserve_regions<'a, I>(&mut self, regions: I)
    where
        Self: 'a,
        I: Iterator<Item = &'a Self> + Clone,
    {
        self.inner
            .reserve_regions(regions.clone().map(|r| &r.inner));
    }

    #[inline]
    fn clear(&mut self) {
        self.inner.clear();
    }
}

impl<T, TC> CopyOnto<OptionRegion<TC>> for Option<T>
where
    TC: Region,
    T: CopyOnto<TC>,
{
    #[inline]
    fn copy_onto(self, target: &mut OptionRegion<TC>) -> <OptionRegion<TC> as Region>::Index {
        self.map(|t| t.copy_onto(&mut target.inner))
    }
}

impl<'a, T: 'a, TC> CopyOnto<OptionRegion<TC>> for &'a Option<T>
where
    TC: Region,
    &'a T: CopyOnto<TC>,
{
    #[inline]
    fn copy_onto(self, target: &mut OptionRegion<TC>) -> <OptionRegion<TC> as Region>::Index {
        self.as_ref().map(|t| t.copy_onto(&mut target.inner))
    }
}

impl<'a, T: 'a, TC> ReserveItems<OptionRegion<TC>> for &'a Option<T>
where
    TC: Region,
    &'a T: ReserveItems<TC>,
{
    fn reserve_items<I>(target: &mut OptionRegion<TC>, items: I)
    where
        I: Iterator<Item = Self> + Clone,
    {
        ReserveItems::reserve_items(&mut target.inner, items.clone().flat_map(|r| r.as_ref()));
    }
}
