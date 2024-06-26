//! A simple benchmark for flatcontainer, adopted from `columnation`'s benchmark.

#![feature(test)]

extern crate test;

use flatcontainer::impls::deduplicate::{CollapseSequence, ConsecutiveOffsetPairs};
use flatcontainer::impls::offsets::OffsetOptimized;
use flatcontainer::impls::tuple::{TupleABCRegion, TupleABRegion};
use flatcontainer::{
    ColumnsRegion, FlatStack, MirrorRegion, OwnedRegion, Push, Region, RegionPreference,
    ReserveItems, SliceRegion, StringRegion,
};
use test::Bencher;

#[bench]
fn empty_copy(bencher: &mut Bencher) {
    _bench_copy(bencher, vec![(); 1024]);
}
#[bench]
fn u64_copy(bencher: &mut Bencher) {
    _bench_copy(bencher, vec![0u64; 1024]);
}
#[bench]
fn u32x2_copy(bencher: &mut Bencher) {
    _bench_copy(bencher, vec![(0u32, 0u32); 1024]);
}
#[bench]
fn u8_u64_copy(bencher: &mut Bencher) {
    _bench_copy(bencher, vec![(0u8, 0u64); 512]);
}
#[bench]
fn str10_copy(bencher: &mut Bencher) {
    _bench_copy(bencher, vec!["grawwwwrr!"; 1024]);
}
#[bench]
fn string10_copy(bencher: &mut Bencher) {
    _bench_copy(bencher, vec![format!("grawwwwrr!"); 1024]);
}
#[bench]
fn string20_copy(bencher: &mut Bencher) {
    _bench_copy(bencher, vec![format!("grawwwwrr!!!!!!!!!!!"); 512]);
}
#[bench]
fn vec_u_s_copy(bencher: &mut Bencher) {
    _bench_copy(
        bencher,
        vec![vec![(0u64, "grawwwwrr!".to_string()); 32]; 32],
    );
}
#[bench]
fn vec_u_vn_s_copy(bencher: &mut Bencher) {
    _bench_copy(
        bencher,
        vec![vec![(0u64, vec![(); 1 << 40], "grawwwwrr!".to_string()); 32]; 32],
    );
}

#[bench]
fn empty_copy_region(bencher: &mut Bencher) {
    _bench_copy_region::<OwnedRegion<_>, _>(bencher, vec![(); 1024]);
}
#[bench]
fn u64_copy_region(bencher: &mut Bencher) {
    _bench_copy_region::<OwnedRegion<_>, _>(bencher, vec![0u64; 1024]);
}
#[bench]
fn u32x2_copy_region(bencher: &mut Bencher) {
    _bench_copy_region::<OwnedRegion<_>, _>(bencher, vec![(0u32, 0u32); 1024]);
}
#[bench]
fn u8_u64_copy_region(bencher: &mut Bencher) {
    _bench_copy_region::<OwnedRegion<_>, _>(bencher, vec![(0u8, 0u64); 512]);
}
#[bench]
fn str10_copy_region(bencher: &mut Bencher) {
    _bench_copy_region::<OwnedRegion<_>, _>(bencher, vec!["grawwwwrr!"; 1024]);
}
#[bench]
fn str100_copy_region(bencher: &mut Bencher) {
    _bench_copy_region::<OwnedRegion<_>, _>(bencher, vec!["grawwwwrrgrawwwwrrgrawwwwrrgrawwwwrrgrawwwwrrgrawwwwrrgrawwwwrrgrawwwwrrgrawwwwrr!!!!!!!!!grawwwwrr!"; 1024]);
}
#[bench]
fn string10_copy_region(bencher: &mut Bencher) {
    _bench_copy_region::<SliceRegion<StringRegion>, _>(bencher, vec![format!("grawwwwrr!"); 1024]);
}
#[bench]
fn string10_copy_region_collapse(bencher: &mut Bencher) {
    _bench_copy_region::<
        SliceRegion<CollapseSequence<ConsecutiveOffsetPairs<StringRegion>>, OffsetOptimized>,
        _,
    >(bencher, vec![format!("grawwwwrr!"); 1024]);
}
#[bench]
fn string20_copy_region(bencher: &mut Bencher) {
    _bench_copy_region::<SliceRegion<StringRegion>, _>(
        bencher,
        vec![format!("grawwwwrr!!!!!!!!!!!"); 512],
    );
}
#[bench]
fn vec_u_s_copy_region(bencher: &mut Bencher) {
    _bench_copy_region::<SliceRegion<SliceRegion<TupleABRegion<MirrorRegion<_>, StringRegion>>>, _>(
        bencher,
        vec![vec![(0u64, "grawwwwrr!".to_string()); 32]; 32],
    );
}
#[bench]
fn vec_u_vn_s_copy_region(bencher: &mut Bencher) {
    _bench_copy_region::<
        SliceRegion<SliceRegion<TupleABCRegion<MirrorRegion<_>, OwnedRegion<_>, StringRegion>>>,
        _,
    >(
        bencher,
        vec![vec![(0u64, vec![(); 1 << 40], "grawwwwrr!".to_string()); 32]; 32],
    );
}
#[bench]
fn vec_u_vn_s_copy_region_column(bencher: &mut Bencher) {
    _bench_copy_region::<
        SliceRegion<
            ColumnsRegion<
                TupleABCRegion<
                    MirrorRegion<_>,
                    CollapseSequence<OwnedRegion<_>>,
                    CollapseSequence<StringRegion>,
                >,
            >,
        >,
        _,
    >(
        bencher,
        vec![vec![(0u64, vec![(); 1 << 40], "grawwwwrr!".to_string()); 32]; 32],
    );
}

#[bench]
fn empty_clone(bencher: &mut Bencher) {
    _bench_clone(bencher, vec![(); 1024]);
}
#[bench]
fn u64_clone(bencher: &mut Bencher) {
    _bench_clone(bencher, vec![0u64; 1024]);
}
#[bench]
fn u32x2_clone(bencher: &mut Bencher) {
    _bench_clone(bencher, vec![(0u32, 0u32); 1024]);
}
#[bench]
fn u8_u64_clone(bencher: &mut Bencher) {
    _bench_clone(bencher, vec![(0u8, 0u64); 512]);
}
#[bench]
fn str10_clone(bencher: &mut Bencher) {
    _bench_clone(bencher, vec!["grawwwwrr!"; 1024]);
}
#[bench]
fn string10_clone(bencher: &mut Bencher) {
    _bench_clone(bencher, vec![format!("grawwwwrr!"); 1024]);
}
#[bench]
fn string20_clone(bencher: &mut Bencher) {
    _bench_clone(bencher, vec![format!("grawwwwrr!!!!!!!!!!!"); 512]);
}
#[bench]
fn vec_u_s_clone(bencher: &mut Bencher) {
    _bench_clone(
        bencher,
        vec![vec![(0u64, "grawwwwrr!".to_string()); 32]; 32],
    );
}
#[bench]
fn vec_u_vn_s_clone(bencher: &mut Bencher) {
    _bench_clone(
        bencher,
        vec![vec![(0u64, vec![(); 1 << 40], "grawwwwrr!".to_string()); 32]; 32],
    );
}

#[bench]
fn empty_realloc(bencher: &mut Bencher) {
    _bench_realloc(bencher, vec![(); 1024]);
}
#[bench]
fn u64_realloc(bencher: &mut Bencher) {
    _bench_realloc(bencher, vec![0u64; 1024]);
}
#[bench]
fn u32x2_realloc(bencher: &mut Bencher) {
    _bench_realloc(bencher, vec![(0u32, 0u32); 1024]);
}
#[bench]
fn u8_u64_realloc(bencher: &mut Bencher) {
    _bench_realloc(bencher, vec![(0u8, 0u64); 512]);
}
#[bench]
fn str10_realloc(bencher: &mut Bencher) {
    _bench_realloc(bencher, vec!["grawwwwrr!"; 1024]);
}
#[bench]
fn string10_realloc(bencher: &mut Bencher) {
    _bench_realloc(bencher, vec![format!("grawwwwrr!"); 1024]);
}
#[bench]
fn string20_realloc(bencher: &mut Bencher) {
    _bench_realloc(bencher, vec![format!("grawwwwrr!!!!!!!!!!!"); 512]);
}
#[bench]
fn vec_u_s_realloc(bencher: &mut Bencher) {
    _bench_realloc(
        bencher,
        vec![vec![(0u64, "grawwwwrr!".to_string()); 32]; 32],
    );
}
#[bench]
fn vec_u_vn_s_realloc(bencher: &mut Bencher) {
    _bench_realloc(
        bencher,
        vec![vec![(0u64, vec![(); 1 << 40], "grawwwwrr!".to_string()); 32]; 32],
    );
}

#[bench]
fn empty_prealloc(bencher: &mut Bencher) {
    _bench_prealloc(bencher, vec![(); 1024]);
}
#[bench]
fn u64_prealloc(bencher: &mut Bencher) {
    _bench_prealloc(bencher, vec![0u64; 1024]);
}
#[bench]
fn u32x2_prealloc(bencher: &mut Bencher) {
    _bench_prealloc(bencher, vec![(0u32, 0u32); 1024]);
}
#[bench]
fn u8_u64_prealloc(bencher: &mut Bencher) {
    _bench_prealloc(bencher, vec![(0u8, 0u64); 512]);
}
#[bench]
fn str10_prealloc(bencher: &mut Bencher) {
    _bench_prealloc(bencher, vec!["grawwwwrr!"; 1024]);
}
#[bench]
fn string10_prealloc(bencher: &mut Bencher) {
    _bench_prealloc(bencher, vec![format!("grawwwwrr!"); 1024]);
}
#[bench]
fn string20_prealloc(bencher: &mut Bencher) {
    _bench_prealloc(bencher, vec![format!("grawwwwrr!!!!!!!!!!!"); 512]);
}
#[bench]
fn vec_u_s_prealloc(bencher: &mut Bencher) {
    _bench_prealloc(
        bencher,
        vec![vec![(0u64, "grawwwwrr!".to_string()); 32]; 32],
    );
}
#[bench]
fn vec_u_vn_s_prealloc(bencher: &mut Bencher) {
    _bench_prealloc(
        bencher,
        vec![vec![(0u64, vec![(); 1 << 40], "grawwwwrr!".to_string()); 32]; 32],
    );
}

#[bench]
fn empty_copy_flat(bencher: &mut Bencher) {
    _bench_copy_flat_preference(bencher, vec![(); 1024]);
}
#[bench]
fn u64_copy_flat(bencher: &mut Bencher) {
    _bench_copy_flat_preference(bencher, vec![0u64; 1024]);
}
#[bench]
fn u32x2_copy_flat(bencher: &mut Bencher) {
    _bench_copy_flat_preference(bencher, vec![(0u32, 0u32); 1024]);
}
#[bench]
fn u8_u64_copy_flat(bencher: &mut Bencher) {
    _bench_copy_flat_preference(bencher, vec![(0u8, 0u64); 512]);
}
#[bench]
fn str10_copy_flat(bencher: &mut Bencher) {
    _bench_copy_flat_preference(bencher, vec!["grawwwwrr!"; 1024]);
}
#[bench]
fn str100_copy_flat(bencher: &mut Bencher) {
    _bench_copy_flat_preference(bencher, vec!["grawwwwrrgrawwwwrrgrawwwwrrgrawwwwrrgrawwwwrrgrawwwwrrgrawwwwrrgrawwwwrrgrawwwwrr!!!!!!!!!grawwwwrr!"; 1024]);
}
#[bench]
fn string10_copy_flat(bencher: &mut Bencher) {
    _bench_copy_flat_preference(bencher, vec![format!("grawwwwrr!"); 1024]);
}
#[bench]
fn string20_copy_flat(bencher: &mut Bencher) {
    _bench_copy_flat_preference(bencher, vec![format!("grawwwwrr!!!!!!!!!!!"); 512]);
}
#[bench]
fn vec_u_s_copy_flat(bencher: &mut Bencher) {
    _bench_copy_flat_preference(
        bencher,
        vec![vec![(0u64, "grawwwwrr!".to_string()); 32]; 32],
    );
}
#[bench]
fn vec_u_vn_s_copy_flat(bencher: &mut Bencher) {
    _bench_copy_flat_preference(
        bencher,
        vec![vec![(0u64, vec![(); 1 << 40], "grawwwwrr!".to_string()); 32]; 32],
    );
}

#[bench]
fn empty_copy_flat_region(bencher: &mut Bencher) {
    _bench_copy_flat::<OwnedRegion<_>, _>(bencher, vec![(); 1024]);
}
#[bench]
fn u64_copy_flat_region(bencher: &mut Bencher) {
    _bench_copy_flat::<OwnedRegion<_>, _>(bencher, vec![0u64; 1024]);
}
#[bench]
fn u32x2_copy_flat_region(bencher: &mut Bencher) {
    _bench_copy_flat::<OwnedRegion<_>, _>(bencher, vec![(0u32, 0u32); 1024]);
}
#[bench]
fn u8_u64_copy_flat_region(bencher: &mut Bencher) {
    _bench_copy_flat::<OwnedRegion<_>, _>(bencher, vec![(0u8, 0u64); 512]);
}
#[bench]
fn str10_copy_flat_region(bencher: &mut Bencher) {
    _bench_copy_flat::<OwnedRegion<_>, _>(bencher, vec!["grawwwwrr!"; 1024]);
}
#[bench]
fn str100_copy_flat_region(bencher: &mut Bencher) {
    _bench_copy_flat::<OwnedRegion<_>, _>(bencher, vec!["grawwwwrrgrawwwwrrgrawwwwrrgrawwwwrrgrawwwwrrgrawwwwrrgrawwwwrrgrawwwwrrgrawwwwrr!!!!!!!!!grawwwwrr!"; 1024]);
}
#[bench]
fn string10_copy_flat_region(bencher: &mut Bencher) {
    _bench_copy_flat::<SliceRegion<StringRegion>, _>(bencher, vec![format!("grawwwwrr!"); 1024]);
}
#[bench]
fn string20_copy_flat_region(bencher: &mut Bencher) {
    _bench_copy_flat::<SliceRegion<StringRegion>, _>(
        bencher,
        vec![format!("grawwwwrr!!!!!!!!!!!"); 512],
    );
}
#[bench]
fn vec_u_s_copy_flat_region(bencher: &mut Bencher) {
    _bench_copy_flat::<SliceRegion<SliceRegion<TupleABRegion<MirrorRegion<_>, StringRegion>>>, _>(
        bencher,
        vec![vec![(0u64, "grawwwwrr!".to_string()); 32]; 32],
    );
}
#[bench]
fn vec_u_vn_s_copy_flat_region(bencher: &mut Bencher) {
    _bench_copy_flat::<
        SliceRegion<SliceRegion<TupleABCRegion<MirrorRegion<_>, OwnedRegion<_>, StringRegion>>>,
        _,
    >(
        bencher,
        vec![vec![(0u64, vec![(); 1 << 40], "grawwwwrr!".to_string()); 32]; 32],
    );
}
#[bench]
fn vec_u_vn_s_copy_flat_region_column(bencher: &mut Bencher) {
    _bench_copy_flat::<
        SliceRegion<
            ColumnsRegion<
                TupleABCRegion<
                    MirrorRegion<_>,
                    CollapseSequence<OwnedRegion<_>>,
                    CollapseSequence<StringRegion>,
                >,
            >,
        >,
        _,
    >(
        bencher,
        vec![vec![(0u64, vec![(); 1 << 40], "grawwwwrr!".to_string()); 32]; 32],
    );
}

fn _bench_copy<T: RegionPreference + Eq>(bencher: &mut Bencher, record: T)
where
    for<'a> <T as RegionPreference>::Region: Push<&'a T>,
{
    // prepare encoded data for bencher.bytes
    let mut arena = FlatStack::default_impl::<T>();

    bencher.iter(|| {
        arena.clear();
        for _ in 0..1024 {
            arena.copy(&record);
        }
    });
    let (mut siz, mut cap) = (0, 0);
    arena.heap_size(|this_siz, this_cap| {
        siz += this_siz;
        cap += this_cap
    });
    bencher.bytes = siz as u64;
    println!("{siz} {cap}");
}

fn _bench_copy_region<R: Region, T>(bencher: &mut Bencher, record: T)
where
    for<'a> R: Push<&'a T>,
{
    // prepare encoded data for bencher.bytes
    let mut arena = FlatStack::<R>::default();

    bencher.iter(|| {
        arena.clear();
        for _ in 0..1024 {
            arena.copy(&record);
        }
    });
    let (mut siz, mut cap) = (0, 0);
    arena.heap_size(|this_siz, this_cap| {
        siz += this_siz;
        cap += this_cap
    });
    bencher.bytes = siz as u64;
    println!("{siz} {cap}");
}

fn _bench_clone<T: RegionPreference + Eq + Clone>(bencher: &mut Bencher, record: T) {
    // prepare encoded data for bencher.bytes
    let mut arena = Vec::new();

    bencher.iter(|| {
        arena.clear();
        for _ in 0..1024 {
            arena.push(record.clone());
        }
    });
}

fn _bench_realloc<T: RegionPreference + Eq>(bencher: &mut Bencher, record: T)
where
    for<'a> <T as RegionPreference>::Region: Push<&'a T>,
{
    let mut arena = FlatStack::default_impl::<T>();
    bencher.iter(|| {
        // prepare encoded data for bencher.bytes
        arena = FlatStack::default_impl::<T>();
        for _ in 0..1024 {
            arena.copy(&record);
        }
    });
    let (mut siz, mut cap) = (0, 0);
    arena.heap_size(|this_siz, this_cap| {
        siz += this_siz;
        cap += this_cap
    });
    bencher.bytes = siz as u64;
}

fn _bench_prealloc<T: RegionPreference + Eq>(bencher: &mut Bencher, record: T)
where
    for<'a> <T as RegionPreference>::Region: ReserveItems<&'a T> + Push<&'a T>,
{
    let mut arena = FlatStack::default_impl::<T>();
    bencher.iter(|| {
        arena = FlatStack::default_impl::<T>();
        // prepare encoded data for bencher.bytes
        arena.reserve_items(std::iter::repeat(&record).take(1024));
        for _ in 0..1024 {
            arena.copy(&record);
        }
    });
    let (mut siz, mut cap) = (0, 0);
    arena.heap_size(|this_siz, this_cap| {
        siz += this_siz;
        cap += this_cap
    });
    bencher.bytes = siz as u64;
}

fn _bench_copy_flat_preference<T>(bencher: &mut Bencher, record: T)
where
    T: RegionPreference,
    for<'a> <T as RegionPreference>::Region:
        Push<&'a T> + Push<<<T as RegionPreference>::Region as Region>::ReadItem<'a>> + Clone,
{
    _bench_copy_flat::<T::Region, T>(bencher, record)
}

fn _bench_copy_flat<R, T>(bencher: &mut Bencher, record: T)
where
    for<'a> R: Region + Push<&'a T> + Push<<R as Region>::ReadItem<'a>> + Clone,
{
    // prepare encoded data for bencher.bytes
    let mut arena = FlatStack::<R>::default();
    for _ in 0..1024 {
        arena.copy(&record);
    }
    let mut target = FlatStack::<R>::default();

    bencher.iter(|| {
        target.clone_from(&arena);
    });
    let (mut siz, mut cap) = (0, 0);
    arena.heap_size(|this_siz, this_cap| {
        siz += this_siz;
        cap += this_cap
    });
    bencher.bytes = siz as u64;
    println!("{siz} {cap}");
}
