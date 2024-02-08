//! A simple benchmark for flatcontainer, adopted from `columnation`'s benchmark.

#![feature(test)]

extern crate test;

use flatcontainer::impls::tuple::TupleABCRegion;
use flatcontainer::{
    Containerized, CopyOnto, CopyRegion, FlatStack, MirrorRegion, Region, ReserveItems,
    SliceRegion, StringRegion,
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
    _bench_copy_region::<CopyRegion<_>, _>(bencher, vec![(); 1024]);
}
#[bench]
fn u64_copy_region(bencher: &mut Bencher) {
    _bench_copy_region::<CopyRegion<_>, _>(bencher, vec![0u64; 1024]);
}
#[bench]
fn u32x2_copy_region(bencher: &mut Bencher) {
    _bench_copy_region::<CopyRegion<_>, _>(bencher, vec![(0u32, 0u32); 1024]);
}
#[bench]
fn u8_u64_copy_region(bencher: &mut Bencher) {
    _bench_copy_region::<CopyRegion<_>, _>(bencher, vec![(0u8, 0u64); 512]);
}
#[bench]
fn str10_copy_region(bencher: &mut Bencher) {
    _bench_copy_region::<CopyRegion<_>, _>(bencher, vec!["grawwwwrr!"; 1024]);
}
#[bench]
fn str100_copy_region(bencher: &mut Bencher) {
    _bench_copy_region::<CopyRegion<_>, _>(bencher, vec!["grawwwwrrgrawwwwrrgrawwwwrrgrawwwwrrgrawwwwrrgrawwwwrrgrawwwwrrgrawwwwrrgrawwwwrr!!!!!!!!!grawwwwrr!"; 1024]);
}
#[bench]
fn string10_copy_region(bencher: &mut Bencher) {
    _bench_copy_region::<SliceRegion<_>, _>(bencher, vec![format!("grawwwwrr!"); 1024]);
}
#[bench]
fn string20_copy_region(bencher: &mut Bencher) {
    _bench_copy_region::<SliceRegion<_>, _>(bencher, vec![format!("grawwwwrr!!!!!!!!!!!"); 512]);
}
#[bench]
fn vec_u_s_copy_region(bencher: &mut Bencher) {
    _bench_copy_region::<SliceRegion<_>, _>(
        bencher,
        vec![vec![(0u64, "grawwwwrr!".to_string()); 32]; 32],
    );
}
#[bench]
fn vec_u_vn_s_copy_region(bencher: &mut Bencher) {
    _bench_copy_region::<
        SliceRegion<SliceRegion<TupleABCRegion<MirrorRegion<_>, CopyRegion<_>, StringRegion>>>,
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

fn _bench_copy<T: Containerized + Eq>(bencher: &mut Bencher, record: T)
where
    for<'a> &'a T: CopyOnto<<T as Containerized>::Region>,
{
    // prepare encoded data for bencher.bytes
    let mut arena = FlatStack::default_impl::<T>();

    bencher.iter(|| {
        arena.clear();
        for _ in 0..1024 {
            arena.copy(&record);
        }
    });
}

fn _bench_copy_region<R: Region, T>(bencher: &mut Bencher, record: T)
where
    for<'a> &'a T: CopyOnto<R>,
{
    // prepare encoded data for bencher.bytes
    let mut arena = FlatStack::<R>::default();

    bencher.iter(|| {
        arena.clear();
        for _ in 0..1024 {
            arena.copy(&record);
        }
    });
}

fn _bench_clone<T: Containerized + Eq + Clone>(bencher: &mut Bencher, record: T) {
    // prepare encoded data for bencher.bytes
    let mut arena = Vec::new();

    bencher.iter(|| {
        arena.clear();
        for _ in 0..1024 {
            arena.push(record.clone());
        }
    });
}

fn _bench_realloc<T: Containerized + Eq>(bencher: &mut Bencher, record: T)
where
    for<'a> &'a T: CopyOnto<<T as Containerized>::Region>,
{
    bencher.iter(|| {
        // prepare encoded data for bencher.bytes
        let mut arena = FlatStack::default_impl::<T>();
        for _ in 0..1024 {
            arena.copy(&record);
        }
    });
}

fn _bench_prealloc<T: Containerized + Eq>(bencher: &mut Bencher, record: T)
where
    for<'a> &'a T:
        ReserveItems<<T as Containerized>::Region> + CopyOnto<<T as Containerized>::Region>,
{
    bencher.iter(|| {
        // prepare encoded data for bencher.bytes
        let mut arena = FlatStack::default_impl::<T>();
        arena.reserve_items(std::iter::repeat(&record).take(1024));
        for _ in 0..1024 {
            arena.copy(&record);
        }
    });
}
