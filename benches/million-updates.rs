use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};
use vivaldi_nc::{NetworkCoordinate2D, NetworkCoordinate3D};

const NUM_NODES: usize = 1_000;

fn million_vivaldi_2d_updates() {
    // create NUM_NODES NCs
    let mut nc: Vec<NetworkCoordinate2D> =
        (0..NUM_NODES).map(|_| NetworkCoordinate2D::new()).collect();

    // loop, update each item for every other item
    for i in 0..NUM_NODES {
        let nc_i = nc[i].clone();
        for (j, nc_j) in nc.iter_mut().enumerate().take(NUM_NODES) {
            if i == j {
                continue;
            }
            let rtt = if i > j { i - j } else { j - i };
            // let nc_j = nc[j].clone();
            nc_j.update(&nc_i, Duration::from_millis(rtt.try_into().unwrap()));
        }
    }
}

fn million_vivaldi_3d_updates() {
    // create NUM_NODES NCs
    let mut nc: Vec<NetworkCoordinate3D> =
        (0..NUM_NODES).map(|_| NetworkCoordinate3D::new()).collect();

    // loop, update each item for every other item
    for i in 0..NUM_NODES {
        let nc_i = nc[i].clone();
        for (j, nc_j) in nc.iter_mut().enumerate().take(NUM_NODES) {
            if i == j {
                continue;
            }
            let rtt = if i > j { i - j } else { j - i };
            // let nc_j = nc[j].clone();
            nc_j.update(&nc_i, Duration::from_millis(rtt.try_into().unwrap()));
        }
    }
}

fn run_benchmarks(c: &mut Criterion) {
    c.bench_function("million 2D updates", |b| b.iter(million_vivaldi_2d_updates));
    c.bench_function("million 3D updates", |b| b.iter(million_vivaldi_3d_updates));
}

criterion_group!(benches, run_benchmarks);
criterion_main!(benches);
