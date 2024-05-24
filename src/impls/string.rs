//! A region that stores strings.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::impls::slice_copy::OwnedRegion;
use crate::{Containerized, Push, Region, ReserveItems};

/// A region to store strings and read `&str`.
///
/// Delegates to a region `R` to store `u8` slices. By default, it uses a [`OwnedRegion`], but a
/// different region can be provided, as long as it absorbs and reads items as `&[u8]`.
///
/// Note that all implementations of `Push<T> for StringRegion` must only accept valid utf-8 data
/// because the region does not validate the contents when indexing.
///
/// # Examples
///
/// We fill some data into a string region and use extract it later.
/// ```
/// use flatcontainer::{Containerized, Push, OwnedRegion, Region, StringRegion};
/// let mut r = <StringRegion>::default();
///
/// let panagram_en = "The quick fox jumps over the lazy dog";
/// let panagram_de = "Zwölf Boxkämpfer jagen Viktor quer über den großen Sylter Deich";
///
/// let en_index = r.push(panagram_en);
/// let de_index = r.push(panagram_de);
///
/// assert_eq!(panagram_de, r.index(de_index));
/// assert_eq!(panagram_en, r.index(en_index));
/// ```
#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StringRegion<R = OwnedRegion<u8>>
where
    for<'a> R: Region<ReadItem<'a> = &'a [u8]> + 'a,
{
    inner: R,
}

impl<R> Region for StringRegion<R>
where
    for<'a> R: Region<ReadItem<'a> = &'a [u8]> + 'a,
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
        // SAFETY: All Push implementations only accept correct utf8 data
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

impl Containerized for &str {
    type Region = StringRegion;
}

impl<R> Push<String> for StringRegion<R>
where
    for<'a> R: Region<ReadItem<'a> = &'a [u8]> + 'a,
    for<'a> R: Push<&'a [u8]>,
{
    #[inline]
    fn push(&mut self, item: String) -> <StringRegion<R> as Region>::Index {
        self.push(item.as_str())
    }
}

impl<R> Push<&String> for StringRegion<R>
where
    for<'a> R: Region<ReadItem<'a> = &'a [u8]> + 'a,
    for<'a> R: Push<&'a [u8]>,
{
    #[inline]
    fn push(&mut self, item: &String) -> <StringRegion<R> as Region>::Index {
        self.push(item.as_str())
    }
}

impl<'b, R> ReserveItems<&'b String> for StringRegion<R>
where
    for<'a> R: Region<ReadItem<'a> = &'a [u8]> + 'a,
    for<'a> R: ReserveItems<&'a [u8]>,
{
    fn reserve_items<I>(&mut self, items: I)
    where
        I: Iterator<Item = &'b String> + Clone,
    {
        self.reserve_items(items.map(String::as_str));
    }
}

impl<R> Push<&str> for StringRegion<R>
where
    for<'a> R: Region<ReadItem<'a> = &'a [u8]> + 'a,
    for<'a> R: Push<&'a [u8]>,
{
    #[inline]
    fn push(&mut self, item: &str) -> <StringRegion<R> as Region>::Index {
        self.inner.push(item.as_bytes())
    }
}

impl<R> Push<&&str> for StringRegion<R>
where
    for<'a> R: Region<ReadItem<'a> = &'a [u8]> + 'a,
    for<'a> R: Push<&'a [u8]>,
{
    #[inline]
    fn push(&mut self, item: &&str) -> <StringRegion<R> as Region>::Index {
        self.push(*item)
    }
}

impl<'b, R> ReserveItems<&'b str> for StringRegion<R>
where
    for<'a> R: Region<ReadItem<'a> = &'a [u8]> + 'a,
    for<'a> R: ReserveItems<&'a [u8]>,
{
    #[inline]
    fn reserve_items<I>(&mut self, items: I)
    where
        I: Iterator<Item = &'b str> + Clone,
    {
        self.inner.reserve_items(items.map(str::as_bytes));
    }
}

impl<'b, 'c, R> ReserveItems<&'b &'c str> for StringRegion<R>
where
    for<'a> R: Region<ReadItem<'a> = &'a [u8]> + 'a,
    for<'a> R: ReserveItems<&'a [u8]>,
{
    #[inline]
    fn reserve_items<I>(&mut self, items: I)
    where
        I: Iterator<Item = &'b &'c str> + Clone,
    {
        self.reserve_items(items.copied());
    }
}

#[cfg(test)]
mod tests {
    use crate::{Push, Region, ReserveItems, StringRegion};

    #[test]
    fn test_inner() {
        let mut r = <StringRegion>::default();
        let index = r.push("abc");
        assert_eq!(r.index(index), "abc");
    }

    #[test]
    fn test_reserve_items_str() {
        let mut r = <StringRegion>::default();
        r.reserve_items(std::iter::repeat("abc").take(1000));

        let (mut cap, mut cnt) = (0, 0);
        r.heap_size(|_, c| {
            cap += c;
            cnt += 1;
        });

        assert!(cap > 0);
        assert!(cnt > 0);
    }

    #[test]
    fn test_reserve_items_ref_str() {
        let mut r = <StringRegion>::default();
        r.reserve_items(std::iter::repeat(&"abc").take(1000));

        let (mut cap, mut cnt) = (0, 0);
        r.heap_size(|_, c| {
            cap += c;
            cnt += 1;
        });

        assert!(cap > 0);
        assert!(cnt > 0);
    }

    #[test]
    fn test_reserve_items_string() {
        let mut r = <StringRegion>::default();
        r.reserve_items(std::iter::repeat(&"abc".to_owned()).take(1000));

        let (mut cap, mut cnt) = (0, 0);
        r.heap_size(|_, c| {
            cap += c;
            cnt += 1;
        });

        assert!(cap > 0);
        assert!(cnt > 0);
    }
}
