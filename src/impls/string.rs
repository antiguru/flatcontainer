//! A region that stores strings.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::impls::slice_copy::OwnedRegion;
use crate::{Containerized, OpinionatedRegion, Push, Region, ReserveItems};

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

    fn reborrow<'b, 'a: 'b>(item: Self::ReadItem<'a>) -> Self::ReadItem<'b>
    where
        Self: 'a,
    {
        item
    }
}

impl OpinionatedRegion for StringRegion {
    type Owned = String;

    fn item_to_owned(item: Self::ReadItem<'_>) -> Self::Owned {
        item.to_string()
    }
    fn item_to_owned_into(item: Self::ReadItem<'_>, target: &mut Self::Owned) {
        <str as ToOwned>::clone_into(item, target);
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
    for<'a> R: Region<ReadItem<'a> = &'a [u8]> + Push<&'a [u8]> + 'a,
{
    #[inline]
    fn push(&mut self, item: String) -> <StringRegion<R> as Region>::Index {
        self.push(item.as_str())
    }
}

impl<R> Push<&String> for StringRegion<R>
where
    for<'a> R: Region<ReadItem<'a> = &'a [u8]> + Push<&'a [u8]> + 'a,
{
    #[inline]
    fn push(&mut self, item: &String) -> <StringRegion<R> as Region>::Index {
        self.push(item.as_str())
    }
}

impl<'b, R> ReserveItems<&'b String> for StringRegion<R>
where
    for<'a> R: Region<ReadItem<'a> = &'a [u8]> + ReserveItems<&'a [u8]> + 'a,
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
    for<'a> R: Region<ReadItem<'a> = &'a [u8]> + Push<&'a [u8]> + 'a,
{
    #[inline]
    fn push(&mut self, item: &str) -> <StringRegion<R> as Region>::Index {
        self.inner.push(item.as_bytes())
    }
}

impl<R> Push<&&str> for StringRegion<R>
where
    for<'a> R: Region<ReadItem<'a> = &'a [u8]> + Push<&'a [u8]> + 'a,
{
    #[inline]
    fn push(&mut self, item: &&str) -> <StringRegion<R> as Region>::Index {
        self.push(*item)
    }
}

impl<'b, R> ReserveItems<&'b str> for StringRegion<R>
where
    for<'a> R: Region<ReadItem<'a> = &'a [u8]> + ReserveItems<&'a [u8]> + 'a,
{
    #[inline]
    fn reserve_items<I>(&mut self, items: I)
    where
        I: Iterator<Item = &'b str> + Clone,
    {
        self.inner.reserve_items(items.map(str::as_bytes));
    }
}

impl<'a, 'b: 'a, R> ReserveItems<&'a &'b str> for StringRegion<R>
where
    for<'c> R: Region<ReadItem<'c> = &'c [u8]> + ReserveItems<&'c [u8]> + 'c,
{
    #[inline]
    fn reserve_items<I>(&mut self, items: I)
    where
        I: Iterator<Item = &'a &'b str> + Clone,
    {
        self.reserve_items(items.copied());
    }
}

#[cfg(test)]
mod tests {
    use crate::{Push, ReadToOwned, Region, ReserveItems, StringRegion};

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

    #[test]
    fn owned() {
        let mut r = <StringRegion>::default();

        let idx = r.push("abc");
        let reference = r.index(idx);
        let owned = reference.read_to_owned();
        let idx = r.push(owned);
        assert_eq!("abc", r.index(idx));
    }
}
