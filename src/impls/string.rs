#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::impls::slice_copy::CopyRegion;
use crate::{Containerized, CopyOnto, Region, ReserveItems};

/// A region to store strings and read `&str`.
#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StringRegion {
    inner: CopyRegion<u8>,
}

impl Region for StringRegion {
    type ReadItem<'a> = &'a str where Self: 'a ;
    type Index = <CopyRegion<u8> as Region>::Index;

    #[inline]
    fn index(&self, index: Self::Index) -> Self::ReadItem<'_> {
        unsafe { std::str::from_utf8_unchecked(self.inner.index(index)) }
    }

    fn reserve_regions<'a, I>(&mut self, regions: I)
    where
        Self: 'a,
        I: Iterator<Item = &'a Self> + Clone,
    {
        self.inner.reserve_regions(regions.map(|r| &r.inner));
    }

    #[inline]
    fn clear(&mut self) {
        self.inner.clear();
    }
}

impl Containerized for String {
    type Region = StringRegion;
}

impl CopyOnto<StringRegion> for &String {
    #[inline]
    fn copy_onto(self, target: &mut StringRegion) -> <StringRegion as Region>::Index {
        self.as_str().copy_onto(target)
    }
}

impl ReserveItems<StringRegion> for &String {
    fn reserve_items<I>(target: &mut StringRegion, items: I)
    where
        I: Iterator<Item = Self> + Clone,
    {
        ReserveItems::reserve_items(target, items.map(String::as_str))
    }
}

impl CopyOnto<StringRegion> for &str {
    #[inline]
    fn copy_onto(self, target: &mut StringRegion) -> <StringRegion as Region>::Index {
        self.as_bytes().copy_onto(&mut target.inner)
    }
}

impl CopyOnto<StringRegion> for &&str {
    #[inline]
    fn copy_onto(self, target: &mut StringRegion) -> <StringRegion as Region>::Index {
        self.as_bytes().copy_onto(&mut target.inner)
    }
}

impl ReserveItems<StringRegion> for &str {
    fn reserve_items<I>(target: &mut StringRegion, items: I)
    where
        I: Iterator<Item = Self> + Clone,
    {
        ReserveItems::reserve_items(&mut target.inner, items.map(str::as_bytes))
    }
}
