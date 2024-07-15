//! A simple benchmark for flatcontainer, adopted from `columnation`'s benchmark.

use codspeed_bencher_compat::{benchmark_group, benchmark_main, Bencher};
use flatcontainer::impls::deduplicate::{CollapseSequence, ConsecutiveIndexPairs};
use flatcontainer::impls::index::IndexOptimized;
use flatcontainer::impls::tuple::{TupleABCRegion, TupleABRegion};
use flatcontainer::{
    ColumnsRegion, FlatStack, MirrorRegion, OwnedRegion, Push, Region, RegionPreference,
    ReserveItems, SliceRegion, StringRegion,
};

fn empty_copy(bencher: &mut Bencher) {
    _bench_copy(bencher, vec![(); 1024]);
}
fn u64_copy(bencher: &mut Bencher) {
    _bench_copy(bencher, vec![0u64; 1024]);
}
fn u32x2_copy(bencher: &mut Bencher) {
    _bench_copy(bencher, vec![(0u32, 0u32); 1024]);
}
fn u8_u64_copy(bencher: &mut Bencher) {
    _bench_copy(bencher, vec![(0u8, 0u64); 512]);
}
fn str10_copy(bencher: &mut Bencher) {
    _bench_copy(bencher, vec!["grawwwwrr!"; 1024]);
}
fn string10_copy(bencher: &mut Bencher) {
    _bench_copy(bencher, vec![format!("grawwwwrr!"); 1024]);
}
fn string20_copy(bencher: &mut Bencher) {
    _bench_copy(bencher, vec![format!("grawwwwrr!!!!!!!!!!!"); 512]);
}
fn vec_u_s_copy(bencher: &mut Bencher) {
    _bench_copy(
        bencher,
        vec![vec![(0u64, "grawwwwrr!".to_string()); 32]; 32],
    );
}
fn vec_u_vn_s_copy(bencher: &mut Bencher) {
    _bench_copy(
        bencher,
        vec![vec![(0u64, vec![(); 1 << 40], "grawwwwrr!".to_string()); 32]; 32],
    );
}

fn empty_copy_region(bencher: &mut Bencher) {
    _bench_copy_region::<OwnedRegion<_>, _>(bencher, vec![(); 1024]);
}
fn u64_copy_region(bencher: &mut Bencher) {
    _bench_copy_region::<OwnedRegion<_>, _>(bencher, vec![0u64; 1024]);
}
fn u32x2_copy_region(bencher: &mut Bencher) {
    _bench_copy_region::<OwnedRegion<_>, _>(bencher, vec![(0u32, 0u32); 1024]);
}
fn u8_u64_copy_region(bencher: &mut Bencher) {
    _bench_copy_region::<OwnedRegion<_>, _>(bencher, vec![(0u8, 0u64); 512]);
}
fn str10_copy_region(bencher: &mut Bencher) {
    _bench_copy_region::<OwnedRegion<_>, _>(bencher, vec!["grawwwwrr!"; 1024]);
}
fn str100_copy_region(bencher: &mut Bencher) {
    _bench_copy_region::<OwnedRegion<_>, _>(bencher, vec!["grawwwwrrgrawwwwrrgrawwwwrrgrawwwwrrgrawwwwrrgrawwwwrrgrawwwwrrgrawwwwrrgrawwwwrr!!!!!!!!!grawwwwrr!"; 1024]);
}
fn string10_copy_region(bencher: &mut Bencher) {
    _bench_copy_region::<SliceRegion<StringRegion>, _>(bencher, vec![format!("grawwwwrr!"); 1024]);
}
fn string10_copy_region_collapse(bencher: &mut Bencher) {
    _bench_copy_region::<
        SliceRegion<CollapseSequence<ConsecutiveIndexPairs<StringRegion>>, IndexOptimized>,
        _,
    >(bencher, vec![format!("grawwwwrr!"); 1024]);
}
fn string20_copy_region(bencher: &mut Bencher) {
    _bench_copy_region::<SliceRegion<StringRegion>, _>(
        bencher,
        vec![format!("grawwwwrr!!!!!!!!!!!"); 512],
    );
}
fn vec_u_s_copy_region(bencher: &mut Bencher) {
    _bench_copy_region::<SliceRegion<SliceRegion<TupleABRegion<MirrorRegion<_>, StringRegion>>>, _>(
        bencher,
        vec![vec![(0u64, "grawwwwrr!".to_string()); 32]; 32],
    );
}
fn vec_u_vn_s_copy_region(bencher: &mut Bencher) {
    _bench_copy_region::<
        SliceRegion<SliceRegion<TupleABCRegion<MirrorRegion<_>, OwnedRegion<_>, StringRegion>>>,
        _,
    >(
        bencher,
        vec![vec![(0u64, vec![(); 1 << 40], "grawwwwrr!".to_string()); 32]; 32],
    );
}
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

fn empty_clone(bencher: &mut Bencher) {
    _bench_clone(bencher, vec![(); 1024]);
}
fn u64_clone(bencher: &mut Bencher) {
    _bench_clone(bencher, vec![0u64; 1024]);
}
fn u32x2_clone(bencher: &mut Bencher) {
    _bench_clone(bencher, vec![(0u32, 0u32); 1024]);
}
fn u8_u64_clone(bencher: &mut Bencher) {
    _bench_clone(bencher, vec![(0u8, 0u64); 512]);
}
fn str10_clone(bencher: &mut Bencher) {
    _bench_clone(bencher, vec!["grawwwwrr!"; 1024]);
}
fn string10_clone(bencher: &mut Bencher) {
    _bench_clone(bencher, vec![format!("grawwwwrr!"); 1024]);
}
fn string20_clone(bencher: &mut Bencher) {
    _bench_clone(bencher, vec![format!("grawwwwrr!!!!!!!!!!!"); 512]);
}
fn vec_u_s_clone(bencher: &mut Bencher) {
    _bench_clone(
        bencher,
        vec![vec![(0u64, "grawwwwrr!".to_string()); 32]; 32],
    );
}
fn vec_u_vn_s_clone(bencher: &mut Bencher) {
    _bench_clone(
        bencher,
        vec![vec![(0u64, vec![(); 1 << 40], "grawwwwrr!".to_string()); 32]; 32],
    );
}

fn empty_realloc(bencher: &mut Bencher) {
    _bench_realloc(bencher, vec![(); 1024]);
}
fn u64_realloc(bencher: &mut Bencher) {
    _bench_realloc(bencher, vec![0u64; 1024]);
}
fn u32x2_realloc(bencher: &mut Bencher) {
    _bench_realloc(bencher, vec![(0u32, 0u32); 1024]);
}
fn u8_u64_realloc(bencher: &mut Bencher) {
    _bench_realloc(bencher, vec![(0u8, 0u64); 512]);
}
fn str10_realloc(bencher: &mut Bencher) {
    _bench_realloc(bencher, vec!["grawwwwrr!"; 1024]);
}
fn string10_realloc(bencher: &mut Bencher) {
    _bench_realloc(bencher, vec![format!("grawwwwrr!"); 1024]);
}
fn string20_realloc(bencher: &mut Bencher) {
    _bench_realloc(bencher, vec![format!("grawwwwrr!!!!!!!!!!!"); 512]);
}
fn vec_u_s_realloc(bencher: &mut Bencher) {
    _bench_realloc(
        bencher,
        vec![vec![(0u64, "grawwwwrr!".to_string()); 32]; 32],
    );
}
fn vec_u_vn_s_realloc(bencher: &mut Bencher) {
    _bench_realloc(
        bencher,
        vec![vec![(0u64, vec![(); 1 << 40], "grawwwwrr!".to_string()); 32]; 32],
    );
}

fn empty_prealloc(bencher: &mut Bencher) {
    _bench_prealloc(bencher, vec![(); 1024]);
}
fn u64_prealloc(bencher: &mut Bencher) {
    _bench_prealloc(bencher, vec![0u64; 1024]);
}
fn u32x2_prealloc(bencher: &mut Bencher) {
    _bench_prealloc(bencher, vec![(0u32, 0u32); 1024]);
}
fn u8_u64_prealloc(bencher: &mut Bencher) {
    _bench_prealloc(bencher, vec![(0u8, 0u64); 512]);
}
fn str10_prealloc(bencher: &mut Bencher) {
    _bench_prealloc(bencher, vec!["grawwwwrr!"; 1024]);
}
fn string10_prealloc(bencher: &mut Bencher) {
    _bench_prealloc(bencher, vec![format!("grawwwwrr!"); 1024]);
}
fn string20_prealloc(bencher: &mut Bencher) {
    _bench_prealloc(bencher, vec![format!("grawwwwrr!!!!!!!!!!!"); 512]);
}
fn vec_u_s_prealloc(bencher: &mut Bencher) {
    _bench_prealloc(
        bencher,
        vec![vec![(0u64, "grawwwwrr!".to_string()); 32]; 32],
    );
}
fn vec_u_vn_s_prealloc(bencher: &mut Bencher) {
    _bench_prealloc(
        bencher,
        vec![vec![(0u64, vec![(); 1 << 40], "grawwwwrr!".to_string()); 32]; 32],
    );
}

fn empty_copy_flat(bencher: &mut Bencher) {
    _bench_copy_flat_preference(bencher, vec![(); 1024]);
}
fn u64_copy_flat(bencher: &mut Bencher) {
    _bench_copy_flat_preference(bencher, vec![0u64; 1024]);
}
fn u32x2_copy_flat(bencher: &mut Bencher) {
    _bench_copy_flat_preference(bencher, vec![(0u32, 0u32); 1024]);
}
fn u8_u64_copy_flat(bencher: &mut Bencher) {
    _bench_copy_flat_preference(bencher, vec![(0u8, 0u64); 512]);
}
fn str10_copy_flat(bencher: &mut Bencher) {
    _bench_copy_flat_preference(bencher, vec!["grawwwwrr!"; 1024]);
}
fn str100_copy_flat(bencher: &mut Bencher) {
    _bench_copy_flat_preference(bencher, vec!["grawwwwrrgrawwwwrrgrawwwwrrgrawwwwrrgrawwwwrrgrawwwwrrgrawwwwrrgrawwwwrrgrawwwwrr!!!!!!!!!grawwwwrr!"; 1024]);
}
fn string10_copy_flat(bencher: &mut Bencher) {
    _bench_copy_flat_preference(bencher, vec![format!("grawwwwrr!"); 1024]);
}
fn string20_copy_flat(bencher: &mut Bencher) {
    _bench_copy_flat_preference(bencher, vec![format!("grawwwwrr!!!!!!!!!!!"); 512]);
}
fn vec_u_s_copy_flat(bencher: &mut Bencher) {
    _bench_copy_flat_preference(
        bencher,
        vec![vec![(0u64, "grawwwwrr!".to_string()); 32]; 32],
    );
}
fn vec_u_vn_s_copy_flat(bencher: &mut Bencher) {
    _bench_copy_flat_preference(
        bencher,
        vec![vec![(0u64, vec![(); 1 << 40], "grawwwwrr!".to_string()); 32]; 32],
    );
}

fn empty_copy_flat_region(bencher: &mut Bencher) {
    _bench_copy_flat::<OwnedRegion<_>, _>(bencher, vec![(); 1024]);
}
fn u64_copy_flat_region(bencher: &mut Bencher) {
    _bench_copy_flat::<OwnedRegion<_>, _>(bencher, vec![0u64; 1024]);
}
fn u32x2_copy_flat_region(bencher: &mut Bencher) {
    _bench_copy_flat::<OwnedRegion<_>, _>(bencher, vec![(0u32, 0u32); 1024]);
}
fn u8_u64_copy_flat_region(bencher: &mut Bencher) {
    _bench_copy_flat::<OwnedRegion<_>, _>(bencher, vec![(0u8, 0u64); 512]);
}
fn str10_copy_flat_region(bencher: &mut Bencher) {
    _bench_copy_flat::<OwnedRegion<_>, _>(bencher, vec!["grawwwwrr!"; 1024]);
}
fn str100_copy_flat_region(bencher: &mut Bencher) {
    _bench_copy_flat::<OwnedRegion<_>, _>(bencher, vec!["grawwwwrrgrawwwwrrgrawwwwrrgrawwwwrrgrawwwwrrgrawwwwrrgrawwwwrrgrawwwwrrgrawwwwrr!!!!!!!!!grawwwwrr!"; 1024]);
}
fn string10_copy_flat_region(bencher: &mut Bencher) {
    _bench_copy_flat::<SliceRegion<StringRegion>, _>(bencher, vec![format!("grawwwwrr!"); 1024]);
}
fn string20_copy_flat_region(bencher: &mut Bencher) {
    _bench_copy_flat::<SliceRegion<StringRegion>, _>(
        bencher,
        vec![format!("grawwwwrr!!!!!!!!!!!"); 512],
    );
}
fn vec_u_s_copy_flat_region(bencher: &mut Bencher) {
    _bench_copy_flat::<SliceRegion<SliceRegion<TupleABRegion<MirrorRegion<_>, StringRegion>>>, _>(
        bencher,
        vec![vec![(0u64, "grawwwwrr!".to_string()); 32]; 32],
    );
}
fn vec_u_vn_s_copy_flat_region(bencher: &mut Bencher) {
    _bench_copy_flat::<
        SliceRegion<SliceRegion<TupleABCRegion<MirrorRegion<_>, OwnedRegion<_>, StringRegion>>>,
        _,
    >(
        bencher,
        vec![vec![(0u64, vec![(); 1 << 40], "grawwwwrr!".to_string()); 32]; 32],
    );
}
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

fn set_bytes(target: &mut u64, bytes: usize) {
    if std::env::var("BYTES").is_ok() {
        *target = bytes as u64;
    }
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
    set_bytes(&mut bencher.bytes, siz);
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
    set_bytes(&mut bencher.bytes, siz);
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
    set_bytes(&mut bencher.bytes, siz);
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
    set_bytes(&mut bencher.bytes, siz);
}

benchmark_group!(
    clone,
    empty_clone,
    str10_clone,
    string10_clone,
    string20_clone,
    u32x2_clone,
    u64_clone,
    u8_u64_clone,
    vec_u_s_clone,
    vec_u_vn_s_clone,
);
benchmark_group!(
    copy,
    empty_copy,
    str10_copy,
    string10_copy,
    string20_copy,
    u32x2_copy,
    u64_copy,
    u8_u64_copy,
    vec_u_s_copy,
    vec_u_vn_s_copy,
);
benchmark_group!(
    copy_flat,
    empty_copy_flat,
    str100_copy_flat,
    str10_copy_flat,
    string10_copy_flat,
    string20_copy_flat,
    u32x2_copy_flat,
    u64_copy_flat,
    u8_u64_copy_flat,
    vec_u_s_copy_flat,
    vec_u_vn_s_copy_flat,
);
benchmark_group!(
    copy_region,
    empty_copy_flat_region,
    empty_copy_region,
    str100_copy_flat_region,
    str100_copy_region,
    str10_copy_flat_region,
    str10_copy_region,
    string10_copy_flat_region,
    string10_copy_region,
    string10_copy_region_collapse,
    string20_copy_flat_region,
    string20_copy_region,
    u32x2_copy_flat_region,
    u32x2_copy_region,
    u64_copy_flat_region,
    u64_copy_region,
    u8_u64_copy_flat_region,
    u8_u64_copy_region,
    vec_u_s_copy_flat_region,
    vec_u_s_copy_region,
    vec_u_vn_s_copy_flat_region,
    vec_u_vn_s_copy_flat_region_column,
    vec_u_vn_s_copy_region,
    vec_u_vn_s_copy_region_column,
);
benchmark_group!(
    alloc,
    empty_prealloc,
    empty_realloc,
    str10_prealloc,
    str10_realloc,
    string10_prealloc,
    string10_realloc,
    string20_prealloc,
    string20_realloc,
    u32x2_prealloc,
    u32x2_realloc,
    u64_prealloc,
    u64_realloc,
    u8_u64_prealloc,
    u8_u64_realloc,
    vec_u_s_prealloc,
    vec_u_s_realloc,
    vec_u_vn_s_prealloc,
    vec_u_vn_s_realloc,
);
benchmark_main!(clone, copy, copy_flat, copy_region, alloc);
