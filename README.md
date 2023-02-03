# vivaldi-nc - Vivaldi Network Coordinates

[![docs.rs](https://img.shields.io/docsrs/vivaldi-nc)](https://docs.rs/vivaldi-nc/) [![builds.sr.ht status](https://builds.sr.ht/~swaits/vivaldi-nc.svg)](https://builds.sr.ht/~swaits/vivaldi-nc?) [![Crates.io](https://img.shields.io/crates/v/vivaldi-nc)](https://crates.io/crates/vivaldi-nc) ![Crates.io](https://img.shields.io/crates/d/vivaldi-nc) ![Crates.io](https://img.shields.io/crates/l/vivaldi-nc)

## Introduction

Network Coordinates (NC) are a way to represent a node's position in a network's
latency space. Nodes with low latency (ping or round trip time) between each
other will be close to each other. Nodes with high latency between them will be
far from each other.

This is an implementation of Vivaldi Network Coordinates, a specific NC
algorithm, with a public simple interface and few dependencies. Vivaldi
coordinates are typically used in distributed networking applications, like p2p
networks. They allow each node in the network to estimate its position in a
latency space in a distributed way, with no central authority. This enables
nodes to understand estimated latency to any other node in the system.

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

### Features

### Examples

## Dependencies

One design goal of this crate is to minimize dependencies. When dependencies
are required, I try to be very selective about them (re: bloat and licensing).
This crate depends on:

- `nanorand`: A fast PRNG based on WyRand. It pulls in `getrandom` as a
  portable source of entropy. I chose this over the more commonly used, but
  heavyweight `rand` because it's significantly smaller and does just what I
  need, and not much more.
- `num-traits`: A brilliant crate which makes it easier to operate on numbers
  in generics; like using `Float` as a constraint on a generic type. Its
  convenience outweighs its cost.

## Alternative implementations

Several crates implemented Vivaldi NC before this one. So, why another?

I had several design goals which the existing crates didn't satisfy:

1. Provide the simplest interface possible. I just want to have some sort of
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

All of that said, those other implementations might work best for you. Here are
the ones I know of today:

- TODO: list crates
- TODO: list crates
- TODO: list crates

## Other Algorithms

Vivaldi is about the simplest distributed NC algorithm out there. That
simplicity combined with its reasonably good performance is a reason why it's
popular.

But it's far from the only choice. Here are links to other NC algorithms:

- TODO: list algorithms
- TODO: list algorithms
- TODO: list algorithms
