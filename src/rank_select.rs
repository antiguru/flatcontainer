//! Rank select structures

use crate::bitmap::Bools;
use crate::{Clear, HeapSize, Index, IndexAs, Len, Push, Region, Reserve, ReserveItems};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RankSelect<C = Vec<u64>, V = Vec<u64>> {
    counts: C,
    values: Bools<V>,
}

impl<C: Region, V: Region> Region for RankSelect<C, V> {
    fn merge_regions<'a>(regions: impl Iterator<Item = &'a Self> + Clone) -> Self
    where
        Self: 'a,
    {
        Self {
            counts: C::merge_regions(regions.clone().map(|r| &r.counts)),
            values: Bools::merge_regions(regions.map(|r| &r.values)),
        }
    }

    fn reserve_regions<'a, I>(&mut self, regions: I)
    where
        Self: 'a,
        I: Iterator<Item = &'a Self> + Clone,
    {
        self.counts
            .reserve_regions(regions.clone().map(|r| &r.counts));
        self.values.reserve_regions(regions.map(|r| &r.values));
    }
}

impl<C, V> HeapSize for RankSelect<C, V>
where
    C: HeapSize,
    V: HeapSize,
{
    fn heap_size<F: FnMut(usize, usize)>(&self, mut callback: F) {
        self.counts.heap_size(&mut callback);
        self.values.heap_size(callback);
    }
}

impl<C, V> Clear for RankSelect<C, V>
where
    C: Clear,
    V: Clear,
{
    fn clear(&mut self) {
        self.counts.clear();
        self.values.clear();
    }
}

impl<C, V> Len for RankSelect<C, V>
where
    V: Len,
{
    fn len(&self) -> usize {
        self.values.len()
    }
}

impl<C, V> RankSelect<C, V>
where
    C: IndexAs<u64> + Len,
    V: IndexAs<u64> + Len,
{
    pub fn rank(&self, index: usize) -> usize {
        let bit = index % 64;
        let block = index / 64;
        let chunk = block / 16;
        let mut count = if chunk > 0 {
            self.counts.index_as(chunk - 1) as usize
        } else {
            0
        };
        for index in (chunk * 16)..block {
            count += self.counts.index_as(index).count_ones() as usize;
        }
        let word = self.values.last_block().0;
        let mask = (1 << bit) - 1;
        let bits = word & mask;
        count + bits.count_ones() as usize
    }

    #[allow(unused)]
    pub fn select(&self, rank: u64) -> Option<usize> {
        let mut chunk = 0;
        while chunk < self.counts.len() && self.counts.index_as(chunk) <= rank {
            chunk += 1;
        }
        let mut count = if chunk < self.counts.len() {
            self.counts.index_as(chunk)
        } else {
            0
        };
        let mut block = chunk * 16;
        while block < self.values.blocks()
            && count + <u64>::from(self.values.block(block).0.count_ones()) <= rank
        {
            count += <u64>::from(self.values.block(block).0.count_ones());
            block += 1;
        }

        let (last_word, last_bits) = self.values.last_block();
        for shift in 0..last_bits {
            if last_word & (1 << shift) != 0 && count + 1 == rank {
                return Some(block * 64 + shift as usize);
            }
            count += last_word & (1 << shift);
        }

        None
    }
}

impl<C, V> Push<bool> for RankSelect<C, V>
where
    C: Push<u64> + Len + IndexAs<u64>,
    V: Push<u64> + Len + IndexAs<u64>,
{
    fn push(&mut self, item: bool) {
        self.values.push(item);
        while self.counts.len() < self.values.len() / 1024 {
            let mut count = if self.counts.is_empty() {
                0
            } else {
                self.counts.index_as(self.counts.len() - 1)
            };
            let lower = self.counts.len() * 16;
            let upper = lower + 16;
            for index in lower..upper {
                count += self.values.block(index).0.count_ones() as u64;
            }
            self.counts.push(count);
        }
    }
}

impl<C, V> IndexAs<bool> for RankSelect<C, V>
where
    V: IndexAs<u64> + Len,
{
    fn index_as(&self, index: usize) -> bool {
        self.values.index(index)
    }
}

impl<C, V> ReserveItems<bool> for RankSelect<C, V>
where
    C: Reserve,
    V: Reserve,
{
    fn reserve_items<I>(&mut self, items: I)
    where
        I: Iterator<Item = bool> + Clone,
    {
        self.counts.reserve((items.clone().count() + 1023) / 1024);
        self.values.reserve_items(items.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_rank() {
        let mut rs = <RankSelect>::default();
        rs.push(true);
        assert_eq!(rs.rank(0), 0);
        assert_eq!(rs.select(0), None);
        assert_eq!(rs.select(1), None);
        rs.push(true);
        assert_eq!(rs.rank(0), 0);
        assert_eq!(rs.select(0), None);
        assert_eq!(rs.select(1), Some(0));
    }
}
