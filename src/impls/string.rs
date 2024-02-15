//! A region that stores strings.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::impls::slice_copy::CopyRegion;
use crate::{Containerized, CopyOnto, Region, ReserveItems};

/// A region to store strings and read `&str`.
///
/// Delegates to a region `R` to store `u8` slices. By default, it uses a [`CopyRegion`], but a
/// different region can be provided, as long as it absorbs and reads items as `&[u8]`.
///
/// # Examples
///
/// We fill some data into a string region and use extract it later.
/// ```
/// use flatcontainer::{Containerized, CopyOnto, CopyRegion, Region, StringRegion};
/// let mut r = <StringRegion>::default();
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
pub struct StringRegion<R = CopyRegion<u8>>
where
    for<'a> R: Region<ReadItem<'a> = &'a [u8]> + 'a,
    for<'a> &'a [u8]: CopyOnto<R>,
{
    inner: R,
}

impl<R> Region for StringRegion<R>
where
    for<'a> R: Region<ReadItem<'a> = &'a [u8]> + 'a,
    for<'a> &'a [u8]: CopyOnto<R>,
{
    type ReadItem<'a> = &'a str where Self: 'a ;
    type Index = R::Index;

    fn merge_regions<'a>(regions: impl Iterator<Item = &'a Self> + Clone) -> Self
    where
        Self: 'a,
    {
        Self {
            inner: R::merge_regions(regions.map(|r| &r.inner)),
        }
    }

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

    fn heap_size<F: FnMut(usize, usize)>(&self, callback: F) {
        self.inner.heap_size(callback);
    }
}

impl Containerized for String {
    type Region = StringRegion;
}

impl<'a> Containerized for &'a str {
    type Region = StringRegion;
}

impl<R> CopyOnto<StringRegion<R>> for String
where
    for<'a> R: Region<ReadItem<'a> = &'a [u8]> + 'a,
    for<'a> &'a [u8]: CopyOnto<R>,
{
    #[inline]
    fn copy_onto(self, target: &mut StringRegion<R>) -> <StringRegion<R> as Region>::Index {
        self.as_str().copy_onto(target)
    }
}

impl<R> CopyOnto<StringRegion<R>> for &String
where
    for<'a> R: Region<ReadItem<'a> = &'a [u8]> + 'a,
    for<'a> &'a [u8]: CopyOnto<R>,
{
    #[inline]
    fn copy_onto(self, target: &mut StringRegion<R>) -> <StringRegion<R> as Region>::Index {
        self.as_str().copy_onto(target)
    }
}

impl<R> ReserveItems<StringRegion<R>> for &String
where
    for<'a> R: Region<ReadItem<'a> = &'a [u8]> + 'a,
    for<'a> &'a [u8]: CopyOnto<R> + ReserveItems<R>,
{
    fn reserve_items<I>(target: &mut StringRegion<R>, items: I)
    where
        I: Iterator<Item = Self> + Clone,
    {
        ReserveItems::reserve_items(target, items.map(String::as_str))
    }
}

impl<R> CopyOnto<StringRegion<R>> for &str
where
    for<'a> R: Region<ReadItem<'a> = &'a [u8]> + 'a,
    for<'a> &'a [u8]: CopyOnto<R>,
{
    #[inline]
    fn copy_onto(self, target: &mut StringRegion<R>) -> <StringRegion<R> as Region>::Index {
        self.as_bytes().copy_onto(&mut target.inner)
    }
}

impl<R> CopyOnto<StringRegion<R>> for &&str
where
    for<'a> R: Region<ReadItem<'a> = &'a [u8]> + 'a,
    for<'a> &'a [u8]: CopyOnto<R>,
{
    #[inline]
    fn copy_onto(self, target: &mut StringRegion<R>) -> <StringRegion<R> as Region>::Index {
        self.as_bytes().copy_onto(&mut target.inner)
    }
}

impl<R> ReserveItems<StringRegion<R>> for &str
where
    for<'a> R: Region<ReadItem<'a> = &'a [u8]> + 'a,
    for<'a> &'a [u8]: CopyOnto<R> + ReserveItems<R>,
{
    fn reserve_items<I>(target: &mut StringRegion<R>, items: I)
    where
        I: Iterator<Item = Self> + Clone,
    {
        ReserveItems::reserve_items(&mut target.inner, items.map(str::as_bytes))
    }
}

impl<R> ReserveItems<StringRegion<R>> for &&str
where
    for<'a> R: Region<ReadItem<'a> = &'a [u8]> + 'a,
    for<'a> &'a [u8]: CopyOnto<R> + ReserveItems<R>,
{
    fn reserve_items<I>(target: &mut StringRegion<R>, items: I)
    where
        I: Iterator<Item = Self> + Clone,
    {
        ReserveItems::reserve_items(&mut target.inner, items.map(|s| s.as_bytes()))
    }
}

#[cfg(test)]
mod tests {
    use crate::{CopyOnto, Region, StringRegion};

    #[test]
    fn test_inner() {
        let mut r = <StringRegion>::default();
        let index = "abc".copy_onto(&mut r);
        assert_eq!(r.index(index), "abc");
    }
}
