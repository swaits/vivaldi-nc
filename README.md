# vivaldi-nc - Vivaldi Network Coordinates

[![docs.rs](https://img.shields.io/docsrs/vivaldi-nc)](https://docs.rs/vivaldi-nc/) [![builds.sr.ht status](https://builds.sr.ht/~swaits/vivaldi-nc.svg)](https://builds.sr.ht/~swaits/vivaldi-nc?) [![Crates.io](https://img.shields.io/crates/v/vivaldi-nc)](https://crates.io/crates/vivaldi-nc) ![Crates.io](https://img.shields.io/crates/d/vivaldi-nc) ![Crates.io](https://img.shields.io/crates/l/vivaldi-nc)

## Introduction

Network Coordinates (NC) are a way to represent a node's position in a network's
latency space. Nodes with low latency (ping or round trip time) between each
other will be close to each other. Nodes with high latency between them will be
far from each other.

This is an implementation of Vivaldi Network Coordinates, a specific NC
algorithm, with a simple interface and few dependencies. Vivaldi coordinates
are typically used in distributed networking applications, like p2p networks.
They allow each node in the network to estimate its position in a latency space
in a distributed way, with no central authority. This enables nodes to
understand estimated latency to any other node in the system.

According to the [Vivaldi coordinates article on Wikipedia](https://en.wikipedia.org/wiki/Vivaldi_coordinates):

> Vivaldi Network Coordinates establish a virtual positioning system that
> has a prime use in networking. The algorithm behind the system uses a
> distributed technique to estimate propagation times between peers in the
> network.
> 
> Through this scheme, network topology awareness can be used to tune the
> network behaviour to more efficiently distribute data. For example, in a
> peer-to-peer network, more responsive identification and delivery of content
> can be achieved. In the Azureus application, Vivaldi is used to improve the
> performance of the distributed hash table that facilitates query matches. 
> 
> **Advantages**
> 
> - Vivaldi is a fully distributed scheme, which achieves good scalability.
> - The Vivaldi algorithm is simple and easy to implement.
> 
> **Drawbacks**
> 
> - Vivaldi is based on the Euclidean distance model, which requires the
>   predicted distances to obey the triangle inequality. However, there are
>   many triangle inequality violations (TIVs) on the Internet.
> - Lack of security design, very easy for malicious nodes to conduct various
>   attacks.

*Citation*: Frank Dabek, Russ Cox, Frans Kaashoek, Robert Morris (2004).
"[Vivaldi: A Decentralized Network Coordinate System](http://conferences.sigcomm.org/sigcomm/2004/papers/p426-dabek111111.pdf)" (PDF).
Proc. of the annual conference of the Special Interest Group on Data Communication (SIGCOMM'04).

### Addressing the Drawbacks

Regarding the two drawbacks mentioned by the Wikiepedia article, quoted above:

- **Euclidean distance**: The original paper proposed a height vector model
  which is superior to the Euclidean distance. This crate implements the height
  vector model. This provides reasonable accuracy and convergence while
  remaining simple to implement.
- **Lack of security**: This is not addressed by this crate. Anyone sharing
  network coordinates among nodes in a network should use appropriate
  cryptography.

## Usage

### Getting Started

Add the crate to your project:

```bash
cargo add vivaldi-nc
```

Each node in the network should create a `NetworkCoordinate` (NC). The node
uses this structure to track its latency position in the entire network.

```rust
use vivaldi_nc::NetworkCoordinate;

// create a 2-dimensional NetworkCoordinate to track my position
let mut my_position = NetworkCoordinate::<2>::new();
```
Normally 2 or 3-dimensions is plenty. Higher dimensions may add some accuracy,
but not enough to be worth the extra costs.

Each node occasionally sends its NC to other nodes. Upon receiving a NC from a
remote node, update your local node's position with the actual measured ping
time to that node.

```rust
my_position.update(&remote_position, Duration::from_millis(measured_rtt));
```

Over time your NC will get more and more accurate as it's updated against more
nodes in the network.

You can estimate your ping time with any other NC you receive, even if it was
forwarded to you indirectly.

```rust
let rtt_estimate = my_position.estimate_rtt(&remote_position);
```

That's the entire interface for creating and iteratively updating NCs.

If you want to save/restore NCs, or send/receive them over a network, you'll
want to serialize/deserialize them. `NetworkCoordinate` supports
[Serde](https://crates.io/crates/serde) by default. Many formats are supported
by Serde, including text formats like [JSON](https://github.com/serde-rs/json)
and compact binary formats like [bincode](https://crates.io/crates/bincode) and
[MessagePack](https://github.com/3Hren/msgpack-rust).

See the [module documentation](https://docs.rs/vivaldi-nc) for more detailed
usage examples.

### Cargo Features

By default, the internal data structures and operations all use `f32`. If you want to use `f64`, you can enable it as a cargo feature:

```toml
[dependencies]
vivaldi-nc = { version = "0.5.2", features = ["f64"] }
```

### Examples

The repository includes an example which loads a n-to-n latency sample from
PlanetLab and iterates on `NetworkCoordinate`s until some low-enough mean error
is reached. The output is a JSON array of elements which contain the NC's
position, its height (or stem latency estimate), and its estimation of error
(lower is better) that look like this:

```json
{
  "position": [
    5.563593,
    -2.332495,
    7.3957834
  ],
  "height": 55.56651,
  "error": 3.241348
}
```
Adding some detail to these three fields:

- `position`: Estimated position of this node (or `NetworkCoordinate`) on the
  network core. Think of this like the node's endpoint into the Internet
  backbone. This is a cartesian coordinate in a multi-dimensional latency space
  (in this case 3D), measured in milliseconds.
- `height`: Estimated stem time, which is the time from the node itself to the
  backbone. So, for example this might represent the latency between your
  node's actual location, like your home, and the point at which it gets into
  the highest-speed, core part of the Internet. Height helps adjust for
  [triangle inequality violations](https://en.wikipedia.org/wiki/Triangle_inequality),
  which are common on the Internet.
- `error`: Estimated error of the current `position`, and `height`. Lower is
  better.

To run the example, first clone the repository locally:

```bash
git clone https://git.sr.ht/~swaits/vivaldi-nc
```

Then `cd` into the local clone and run the example:

```bash
cd vivaldi-nc
cargo run --example planetlab
```

## Dependencies

One design goal of this crate is to minimize dependencies. When dependencies
are required, I try to be very selective about them (re: bloat and licensing).
This crate depends on:

- [`nanorand`](https://crates.io/crates/nanorand): A fast PRNG based on WyRand.
  It pulls in `getrandom` as a portable source of entropy. I chose this over
  the more commonly used, but heavyweight `rand` because it's significantly
  smaller and does just what I need, and not much more.
- [`num-traits`](https://crates.io/crates/num-traits): A brilliant crate which
  makes it easier to operate on numbers in generics; like using `Float` as a
  constraint on a generic type. Its convenience outweighs its cost.
- [`serde`](https://crates.io/crates/serde): I think NCs are usually meant to
  be shared across a network. That requires serialization/deserialization and
  serde is *the* choice for that. It might be big, but it's efficient.
- [`serde_with`](https://crates.io/crates/serde_with): Because the inner
  `Vector<T,N>` uses a const generic length, we use this to help derive
  `Deserialize`.

## Design Goals & Alternatives

Several crates implemented Vivaldi NC before this one. So, why another?

I had several design goals which the existing crates didn't satisfy:

1. Provide the *simplest interface possible*. I just want to have some sort of
   Coordinate struct and be able to update it, and then use it to estimate
   round trip times (ie ping times).
2. Don't require the consumer to bring their own vector or linear algebra
   library. The linear algebra required by this is extraordinarily simple. I
   want it to just work without me needing to inject some large, mostly-unused
   library.
3. Serializable/Deserializable by default. These things are useful for sending
   across networks. The intent is to be able to do that without a bunch of
   rigamarole. Support `serde` traits by default.
4. Well documented. Well tested. This varies by the crates.

### Other Vivaldi NC Implementations

All of that said, those other rust implementations might work best for you.
Here are the ones I know of today:

- [netloc](https://github.com/dgtony/netloc)
- [violin](https://crates.io/crates/violin)
- [vivaldi](https://crates.io/crates/vivaldi)

### Other NC Algorithms

Vivaldi is about the simplest distributed NC algorithm out there. That
simplicity combined with its reasonably good performance is a reason why it's
popular.

But it's far from the only choice. Here are links to other NC algorithms:

- [Pharos](https://en.wikipedia.org/wiki/Pharos_Network_Coordinates)
- [Phoenix](https://en.wikipedia.org/wiki/Phoenix_Network_Coordinates)

Search your favorite research paper index for "network coordinates" and you'll
find many more.

## Getting Help or Contributing

To get help or discuss this crate, [submit a
ticket](https://todo.sr.ht/~swaits/vivaldi-nc) or post on
[`vivaldi-nc-discuss@`](https://lists.sr.ht/~swaits/vivaldi-nc-discuss).

Discussion related to development or patch submissions should go to
[`vivaldi-nc-devel@`](https://lists.sr.ht/~swaits/vivaldi-nc-devel).

Patchsets should use [git-send-email](https://git-send-email.io/) or the [sr.ht
UI](https://git.sr.ht/~swaits/vivaldi-nc/send-email) (easiest IMO).

Patch submitters implicitly agree that all contributions they submit fall under
the MIT license.

## License

> MIT License
> 
> Copyright (c) 2023 Stephen Waits
> 
> Permission is hereby granted, free of charge, to any person obtaining a copy
> of this software and associated documentation files (the "Software"), to deal
> in the Software without restriction, including without limitation the rights
> to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
> copies of the Software, and to permit persons to whom the Software is
> furnished to do so, subject to the following conditions:
> 
> The above copyright notice and this permission notice shall be included in all
> copies or substantial portions of the Software.
> 
> THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
> IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
> FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
> AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
> LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
> OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
> SOFTWARE.
