//! A region that stores strings.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::impls::slice_copy::CopyRegion;
use crate::{Containerized, CopyOnto, Region, ReserveItems};

/// A region to store strings and read `&str`.
///
/// # Examples
///
/// We fill some data into a string region and use extract it later.
/// ```
/// use flatcontainer::{Containerized, CopyOnto, Region, StringRegion};
/// let mut r = StringRegion::default();
///
/// let panagram_en = "The quick fox jumps over the lazy dog";
/// let panagram_de = "Zwölf Boxkämpfer jagen Viktor quer über den großen Sylter Deich";
///
/// let en_index = panagram_en.copy_onto(&mut r);
/// let de_index = panagram_de.copy_onto(&mut r);
///
/// assert_eq!(panagram_de, r.index(de_index));
/// assert_eq!(panagram_en, r.index(en_index));
/// ```
#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StringRegion {
    inner: CopyRegion<u8>,
}

impl Region for StringRegion {
    type ReadItem<'a> = &'a str where Self: 'a ;
    type ReadItemMut<'a> = &'a mut str where Self: 'a;
    type Index = <CopyRegion<u8> as Region>::Index;

    fn merge_regions<'a>(regions: impl Iterator<Item = &'a Self> + Clone) -> Self
    where
        Self: 'a,
    {
        Self {
            inner: CopyRegion::merge_regions(regions.map(|r| &r.inner)),
        }
    }

    #[inline]
    fn index(&self, index: Self::Index) -> Self::ReadItem<'_> {
        unsafe { std::str::from_utf8_unchecked(self.inner.index(index)) }
    }

    fn index_mut(&mut self, index: Self::Index) -> Self::ReadItemMut<'_> {
        unsafe { std::str::from_utf8_unchecked_mut(self.inner.index_mut(index)) }
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

impl<'a> Containerized for &'a str {
    type Region = StringRegion;
}

impl CopyOnto<StringRegion> for String {
    #[inline]
    fn copy_onto(self, target: &mut StringRegion) -> <StringRegion as Region>::Index {
        self.as_str().copy_onto(target)
    }
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

impl CopyOnto<StringRegion> for &mut str {
    #[inline]
    fn copy_onto(self, target: &mut StringRegion) -> <StringRegion as Region>::Index {
        <&str as CopyOnto<StringRegion>>::copy_onto(self, target)
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

impl ReserveItems<StringRegion> for &&str {
    fn reserve_items<I>(target: &mut StringRegion, items: I)
    where
        I: Iterator<Item = Self> + Clone,
    {
        ReserveItems::reserve_items(&mut target.inner, items.map(|s| s.as_bytes()))
    }
}
