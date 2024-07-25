//! Test that the types defined by this crate are useful, i.e., they
//! offer implementations over what's absolutely necessary.

use flatcontainer::{
    ColumnsRegion, MirrorRegion, OptionRegion, OwnedRegion, Push, Region, ReserveItems,
    ResultRegion, SliceRegion, StringRegion,
};

trait UsefulRegion:
    Region
    + Push<<Self as Region>::Owned>
    + for<'a> Push<&'a <Self as Region>::Owned>
    + for<'a> Push<<Self as Region>::ReadItem<'a>>
    + for<'a> ReserveItems<&'a <Self as Region>::Owned>
    + for<'a> ReserveItems<<Self as Region>::ReadItem<'a>>
{
}

impl<R> UsefulRegion for R where
    R: Region
        + Push<<Self as Region>::Owned>
        + for<'a> Push<&'a <Self as Region>::Owned>
        + for<'a> Push<<Self as Region>::ReadItem<'a>>
        + for<'a> ReserveItems<&'a <Self as Region>::Owned>
        + for<'a> ReserveItems<<Self as Region>::ReadItem<'a>>
{
}

trait DerefRegion: UsefulRegion
where
    <Self as Region>::Owned: std::ops::Deref,
    Self: for<'a> Push<&'a <<Self as Region>::Owned as std::ops::Deref>::Target>,
    Self: for<'a> ReserveItems<&'a <<Self as Region>::Owned as std::ops::Deref>::Target>,
{
}

impl<R> DerefRegion for R
where
    R: UsefulRegion,
    <Self as Region>::Owned: std::ops::Deref,
    Self: for<'a> Push<&'a <<Self as Region>::Owned as std::ops::Deref>::Target>,
    Self: for<'a> ReserveItems<&'a <<Self as Region>::Owned as std::ops::Deref>::Target>,
{
}

#[test]
fn test_useful_region() {
    fn _useful_region<R: UsefulRegion>() {}
    _useful_region::<MirrorRegion<usize>>();
    _useful_region::<OptionRegion<MirrorRegion<usize>>>();
    _useful_region::<OwnedRegion<usize>>();
    _useful_region::<ResultRegion<MirrorRegion<usize>, MirrorRegion<usize>>>();
    _useful_region::<SliceRegion<MirrorRegion<usize>>>();
    _useful_region::<StringRegion>();
    _useful_region::<Vec<usize>>();
    _useful_region::<ColumnsRegion<MirrorRegion<usize>>>();
    // _useful_region::<CodecRegion<DictionaryCodec>>();
}

#[test]
fn test_deref_region() {
    fn _deref_region<R: DerefRegion>()
    where
        <R as Region>::Owned: std::ops::Deref,
    {
    }
    _deref_region::<OwnedRegion<usize>>();
    _deref_region::<SliceRegion<MirrorRegion<usize>>>();
    _deref_region::<StringRegion>();
}
