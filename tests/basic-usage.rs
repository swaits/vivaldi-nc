// This is just for basic interface and happy path testing. It doesn't do anything other than make
// sure the expected public interface exists and works.

use array_init::array_init;
use core::time::Duration;
use vivaldi_nc::NetworkCoordinate;

const NUM_NODES: usize = 100;

#[test]
fn basic_usage() {
    let mut nc: [NetworkCoordinate<2>; NUM_NODES] = array_init(|_| NetworkCoordinate::<2>::new());
    for i in 0..NUM_NODES {
        for j in 0..NUM_NODES {
            if i == j {
                continue;
            }
            let other = nc[j].clone();
            nc[i].update(&other, Duration::from_millis(100));
        }
    }
}
