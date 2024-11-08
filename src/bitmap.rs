//! Bitmaps

use crate::{Clear, HeapSize, Index, IndexAs, Len, Push, Region, Reserve, ReserveItems};
use serde::{Deserialize, Serialize};

/// TODO
#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Bools<V = Vec<u64>> {
    values: V,
    last_word: u64,
    last_bits: u8,
}

impl<V: Len + IndexAs<u64>> Bools<V> {
    /// Number of blocks in the bitmap.
    #[inline(always)]
    pub fn blocks(&self) -> usize {
        self.values.len() + (self.last_bits > 0) as usize
    }

    /// Value of the last block.
    #[inline(always)]
    pub fn last_block(&self) -> (u64, u8) {
        if self.last_bits > 0 || self.values.is_empty() {
            (self.last_word, self.last_bits)
        } else {
            (self.values.index_as(self.values.len() - 1), 64)
        }
    }

    /// Value of the block at the given index.
    pub fn block(&self, index: usize) -> (u64, u8) {
        if index == self.values.len() {
            (self.last_word, self.last_bits)
        } else {
            (self.values.index_as(index), 64)
        }
    }
}

impl<V: Len> Len for Bools<V> {
    #[inline(always)]
    fn len(&self) -> usize {
        self.values.len() * 64 + self.last_bits as usize
    }
}

impl<V: Clear> Clear for Bools<V> {
    #[inline(always)]
    fn clear(&mut self) {
        self.values.clear();
        self.last_word = 0;
        self.last_bits = 0;
    }
}

impl<V: HeapSize> HeapSize for Bools<V> {
    fn heap_size<F: FnMut(usize, usize)>(&self, callback: F) {
        self.values.heap_size(callback);
    }
}

impl<V> Region for Bools<V>
where
    V: Region,
{
    fn merge_regions<'a>(regions: impl Iterator<Item = &'a Self> + Clone) -> Self
    where
        Self: 'a,
    {
        Self {
            values: V::merge_regions(regions.map(|r| &r.values)),
            last_word: 0,
            last_bits: 0,
        }
    }

    fn reserve_regions<'a, I>(&mut self, regions: I)
    where
        Self: 'a,
        I: Iterator<Item = &'a Self> + Clone,
    {
        self.values.reserve_regions(regions.map(|r| &r.values));
    }
}

impl<V> ReserveItems<bool> for Bools<V>
where
    V: Reserve,
{
    fn reserve_items<I>(&mut self, items: I)
    where
        I: Iterator<Item = bool> + Clone,
    {
        self.values.reserve((items.count() + 63) / 64);
    }
}

impl<V> Index for Bools<V>
where
    V: Len + IndexAs<u64>,
{
    type Owned = bool;
    type ReadItem<'a> = bool where Self: 'a;

    fn index(&self, index: usize) -> Self::ReadItem<'_> {
        let block = index / 64;
        let word = if block == self.values.len() {
            self.last_word
        } else {
            self.values.index_as(block)
        };
        let bit = index % 64;
        word & (1 << bit) != 0
    }

    fn reborrow<'b, 'a: 'b>(item: Self::ReadItem<'a>) -> Self::ReadItem<'b>
    where
        Self: 'a,
    {
        item
    }
}

impl<V> IndexAs<bool> for Bools<V>
where
    V: IndexAs<u64> + Len,
{
    #[inline(always)]
    fn index_as(&self, index: usize) -> bool {
        self.index(index)
    }
}

impl<V: Push<u64>> Push<bool> for Bools<V> {
    #[inline(always)]
    fn push(&mut self, item: bool) {
        self.last_word |= (item as u64) << self.last_bits;
        self.last_bits += 1;
        // If we have filled the last word, push it to the values and reset the last word
        if self.last_bits == 64 {
            self.values.push(self.last_word);
            self.last_word = 0;
            self.last_bits = 0;
        }
    }
}

impl<V> Push<&bool> for Bools<V>
where
    Self: Push<bool>,
{
    #[inline(always)]
    fn push(&mut self, item: &bool) {
        self.push(*item);
    }
}

#[cfg(test)]
mod tests {
    use crate::{Index, Push};

    use super::*;

    #[test]
    fn test_bitmap() {
        let mut bitmap = <Bools>::default();
        for i in 0..100 {
            bitmap.push(i % 2 == 0);
        }
        assert_eq!(bitmap.len(), 100);
        for i in 0..100 {
            assert_eq!(bitmap.index(i), i % 2 == 0);
        }

        let mut bitmap = <Bools>::default();
        assert_eq!((0, 0), bitmap.last_block());
        bitmap.push(true);
        assert_eq!((0b1, 1), bitmap.last_block());
        for _ in 0..63 {
            bitmap.push(false);
        }
        bitmap.push(true);
        assert_eq!((0b1, 1), bitmap.last_block());
        assert_eq!((0b1, 64), bitmap.block(0));
    }
}
