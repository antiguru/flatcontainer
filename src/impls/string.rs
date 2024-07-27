//! A region that stores strings.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::impls::slice_owned::OwnedRegion;
use crate::{CanPush, Push, Region, RegionPreference, Reserve, ReserveItems, TryPush};

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
/// use flatcontainer::{RegionPreference, Push, OwnedRegion, Region, StringRegion};
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
#[derive(Default, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StringRegion<R = OwnedRegion<u8>> {
    inner: R,
}

impl<R: Clone> Clone for StringRegion<R> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.inner.clone_from(&source.inner);
    }
}

impl<R> Region for StringRegion<R>
where
    for<'a> R: Region<ReadItem<'a> = &'a [u8]> + 'a,
{
    type Owned = String;
    type ReadItem<'a> = &'a str where Self: 'a ;
    type Index = R::Index;

    #[inline]
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

    #[inline]
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

    #[inline]
    fn heap_size<F: FnMut(usize, usize)>(&self, callback: F) {
        self.inner.heap_size(callback);
    }

    #[inline]
    fn reborrow<'b, 'a: 'b>(item: Self::ReadItem<'a>) -> Self::ReadItem<'b>
    where
        Self: 'a,
    {
        item
    }
}

impl RegionPreference for String {
    type Owned = Self;
    type Region = StringRegion;
}

impl RegionPreference for &str {
    type Owned = String;
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

impl<R> TryPush<String> for StringRegion<R>
where
    for<'a> R: Region<ReadItem<'a> = &'a [u8]>
        + TryPush<&'a [u8]>
        + Push<&'a [u8]>
        + CanPush<&'a [u8]>
        + 'a,
{
    #[inline]
    fn try_push(&mut self, item: String) -> Result<Self::Index, String> {
        if self.can_push(std::iter::once(item.as_str())) {
            Ok(self.push(item))
        } else {
            Err(item)
        }
    }
}

impl<'a, R> CanPush<&'a String> for StringRegion<R>
where
    R: CanPush<&'a [u8]>,
{
    #[inline]
    fn can_push<I>(&self, items: I) -> bool
    where
        I: Iterator<Item = &'a String> + Clone,
    {
        self.inner.can_push(items.map(|item| item.as_bytes()))
    }
}

impl<'b, R> ReserveItems<&'b String> for StringRegion<R>
where
    for<'a> R: Region<ReadItem<'a> = &'a [u8]> + ReserveItems<&'a [u8]> + 'a,
{
    #[inline]
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

impl<R> TryPush<&str> for StringRegion<R>
where
    for<'a> R: Region<ReadItem<'a> = &'a [u8]>
        + TryPush<&'a [u8]>
        + Push<&'a [u8]>
        + CanPush<&'a [u8]>
        + 'a,
{
    #[inline]
    fn try_push<'a>(&mut self, item: &'a str) -> Result<Self::Index, &'a str> {
        if self.can_push(std::iter::once(item)) {
            Ok(self.push(item))
        } else {
            Err(item)
        }
    }
}

impl<'a, R> CanPush<&'a str> for StringRegion<R>
where
    R: CanPush<&'a [u8]>,
{
    #[inline]
    fn can_push<I>(&self, items: I) -> bool
    where
        I: Iterator<Item = &'a str> + Clone,
    {
        self.inner.can_push(items.map(|item| item.as_bytes()))
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

impl<R> TryPush<&&str> for StringRegion<R>
where
    for<'a> R: Region<ReadItem<'a> = &'a [u8]>
        + TryPush<&'a [u8]>
        + Push<&'a [u8]>
        + CanPush<&'a [u8]>
        + 'a,
{
    #[inline]
    fn try_push<'a, 'b>(&mut self, item: &'a &'b str) -> Result<Self::Index, &'a &'b str> {
        if self.can_push(std::iter::once(*item)) {
            Ok(self.push(item))
        } else {
            Err(item)
        }
    }
}

impl<'a, 'b, R> CanPush<&'a &'b str> for StringRegion<R>
where
    R: CanPush<&'b [u8]>,
    'b: 'a,
{
    #[inline]
    fn can_push<I>(&self, items: I) -> bool
    where
        I: Iterator<Item = &'a &'b str> + Clone,
    {
        self.inner.can_push(items.map(|item| item.as_bytes()))
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

impl<R: Reserve> Reserve for StringRegion<R> {
    type Reserve = R::Reserve;

    fn reserve(&mut self, size: &Self::Reserve) {
        self.inner.reserve(size);
    }
}

#[cfg(test)]
mod tests {
    use crate::{IntoOwned, Push, Region, ReserveItems, StringRegion};

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
        let owned = reference.into_owned();
        let idx = r.push(owned);
        assert_eq!("abc", r.index(idx));
    }
}
