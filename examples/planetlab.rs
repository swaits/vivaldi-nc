// this uses a copy of data from https://github.com/uofa-rzhu3/NetLatency-Data
//
// for info on the format, see `NetLatency-Data/README.md`

//
// **** Features ****
//

/// `FloatType` is a type alias for either `f32` or `f64` depending on cargo features
#[cfg(feature = "f32")]
type FloatType = f32;

/// `FloatType` is a type alias for either `f32` or `f64` depending on cargo features
#[cfg(not(feature = "f32"))]
type FloatType = f64;

use std::{
    fs::File,
    io::{BufReader, Read},
    time::Duration,
};

use rand::Rng;
use vivaldi_nc::NetworkCoordinate;

// this will return a Vec<Vec<Duration>>
fn load_planetlab_file(filename: &str) -> Vec<Vec<Duration>> {
    // we're expecting 490 lines, each with 490 numbers representing ms
    let f = File::open(filename).expect("unable to open file");
    let mut str = String::new();
    BufReader::new(f)
        .read_to_string(&mut str)
        .expect("unable to read file");

    str.lines()
        .map(|line| {
            // ...
            line.split_whitespace()
                .map(|token| {
                    Duration::from_secs_f32(token.parse::<f32>().expect("parse error") / 1000.0)
                })
                .collect()
        })
        .collect()
}

const NUM_LATENCY_SETS: usize = 18;
const NUM_NODES: usize = 490;

fn load_data() -> Vec<Vec<Vec<Duration>>> {
    // our input filenames (note naming is 1-based, not 0-based)
    (1..=NUM_LATENCY_SETS)
        .map(|i| {
            let filename = format!("examples/NetLatency-Data/PlanetLab/PlanetLabData_{i}");
            load_planetlab_file(&filename)
        })
        .collect()
}

fn main() {
    // fetch all the PlanetLab latency data
    let data = load_data();

    // make check to see that we got NUM_NODES worth of data
    assert_eq!(data.len(), NUM_LATENCY_SETS);
    assert_eq!(data[0].len(), NUM_NODES);
    assert_eq!(data[0][0].len(), NUM_NODES);

    // create our network coordinates
    let mut nc: Vec<NetworkCoordinate<3>> = (0..NUM_NODES)
        .map(|_| NetworkCoordinate::<3>::new())
        .collect();

    // the main loop:
    // 1. choose a local and remote NC
    // 2. choose one of the 18 latency sample sets
    // 3. lookup the latency in our data
    // 4. adjust local NC
    let mut error = 0.0;
    for _ in 0..30_000 {
        let mut rng = rand::thread_rng();
        // 1. choose a local and remote NC
        let i_local = rng.gen_range(0..NUM_NODES);
        let i_remote = rng.gen_range(0..NUM_NODES);
        if i_local == i_remote {
            continue;
        }

        // 2. choose one of the 18 latency sample sets
        let i_set = 0; //rng.generate::<usize>() % NUM_LATENCY_SETS;

        // 3. lookup the latency in our data
        let rtt_measured = data[i_set][i_local][i_remote];

        // 4. adjust local NC
        let nc_remote = nc[i_remote].clone();
        nc[i_local].update(&nc_remote, rtt_measured);

        // complete, now let's sum up the error
        error = nc.iter().map(|n| n.error()).sum::<FloatType>() / NUM_NODES as FloatType;
        assert!(error.is_finite());
        if error < 5.0 {
            break;
        }
    }
    assert!(error < 5.0);

    // output the NC array as JSON
    let json = serde_json::to_string_pretty(&nc).expect("JSON serialization error");
    println!("{json}");
}
