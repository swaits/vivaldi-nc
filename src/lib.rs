//! Vivaldi network coordinates for fast, distributed latency estimates in multinode networks, with
//! a clean and simple interface.
//!
//! Network Coordinates (NC) are a way to represent a node's position in a network's
//! latency space. Nodes with low latency (ping or round trip time) between each
//! other will be close to each other in this latency space. Nodes with high latency between them
//! will be far from each other.
//!
//! This is an implementation of Vivaldi Network Coordinates, a specific NC
//! algorithm, with a simple interface and few dependencies. Vivaldi coordinates
//! are typically used in distributed networking applications, like p2p networks.
//! They allow each node in the network to estimate its position in a latency space
//! in a distributed way, with no central authority. This enables nodes to
//! understand estimated latency to any other node in the system.
//!
//! To learn more, see the [Vivaldi coordinates article on
//! Wikipedia](https://en.wikipedia.org/wiki/Vivaldi_coordinates) or [the original
//! paper](http://conferences.sigcomm.org/sigcomm/2004/papers/p426-dabek111111.pdf) (Frank Dabek,
//! Russ Cox, Frans Kaashoek, Robert Morris (2004)).
//!
//! # Usage
//!
//! This crate exports a single struct, [`NetworkCoordinate`] and two type aliases
//! ([`NetworkCoordinate2D`] and [`NetworkCoordinate3D`]). Typical use of Vivaldi NCs for a
//! distributed network works like this:
//!
//! 1. Each node in the network has its own instance of [`NetworkCoordinate`]. *See "Note on
//!    dimensionality" below.*
//! 2. Every node occasionally sends its [`NetworkCoordinate`] to some or all other nodes through
//!    the network overlay.
//! 3. Whenever a node receives a remote coordinate, it updates its local coordinate with the
//!    actual measured round trip time (RTT or "ping time") between itself and that node to improve
//!    its own position and thus reduce its estimation error.
//! 4. Any node can estimate its RTT to any other [`NetworkCoordinate`] it's aware of.
//!
//! Importantly, all updates in this system are performed locally. No central authority is needed
//! to build the network latency maps. It's fully distributed.
//!
//! **Note on dimensionality:** Typically 2D or 3D Vivaldi coordinates (i.e. `NetworkCoordinate<2>` or
//! `NetworkCoordinate<3>`) are sufficient. Higher dimensions don't add much accuracy and aren't
//! generally worth it. That said, you're welcome to use any dimension you like.
//!
//! # Examples
//!
//! In the first example, we simulate receving a remote NC, measuring ping, and updating our NC
//! accordingly.
//!
//! ```
//! use vivaldi_nc::NetworkCoordinate;
//!
//! // create our local NC as a 3-dimensional Vivaldi coordinate
//! let mut my_nc = NetworkCoordinate::<3>::new();
//!
//! // mock up a remote NC for the sake of this example (normally the node would actually receive
//! // this from a remote node)
//! let received_msg = "{\"position\":[1.5,0.5,2.0],\"height\":0.1,\"error\":1.0}";
//! let remote_nc: NetworkCoordinate<3> = serde_json::from_str(received_msg).unwrap();
//!
//! // mock up a measured RTT between us and that node (normally the nodes would coordinate
//! // measuring this together)
//! let rtt_actual = std::time::Duration::from_millis(78);
//!
//! // with the actual RTT in hand, we can update our NC to improve its accuracy
//! my_nc.update(&remote_nc, rtt_actual);
//!
//! // by doing this iteratively (and forever), all NCs in the network will converge to better
//! // represent the actual latency space of the network
//! ```
//!
//! In this second example, we simulate receving a NC from another node indirectly; likely
//! forwarded to us by someone else. And in this case, we aren't measuring the actual RTT. Instead,
//! we're using the NCs to estimate RTT.
//!
//! This example also uses the type alias [`NetworkCoordinate3D`] instead of its equivalent
//! [`NetworkCoordinate<3>`].
//!
//! ```
//! use vivaldi_nc::NetworkCoordinate3D;
//!
//! // create our local NC as a 3-dimensional Vivaldi coordinate
//! let mut my_nc = NetworkCoordinate3D::new();
//!
//! // mock up a remote NC for the sake of this example (normally the node would actually receive
//! // this from a remote node)
//! let received_msg = "{\"position\":[1.5,0.5,2.0],\"height\":0.1,\"error\":1.0}";
//! let remote_nc: NetworkCoordinate3D = serde_json::from_str(received_msg).unwrap();
//!
//! // estimate the RTT to this remote node without measuring it directly
//! let rtt_estimated = my_nc.estimated_rtt(&remote_nc);
//!
//! // so with just the knowledge of another NC, we can get reasonably good estimates of RTT
//! // without actually needing to measure it
//! ```
//!

#![deny(clippy::all)]
#![deny(clippy::correctness)]
#![deny(clippy::style)]
#![deny(clippy::suspicious)]
#![deny(clippy::complexity)]
#![deny(clippy::perf)]
#![deny(clippy::cargo)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(clippy::unwrap_used)]
#![allow(clippy::type_repetition_in_bounds)]

mod height_vector;
mod vector;

// publish our interface
pub mod network_coordinate;
pub use network_coordinate::NetworkCoordinate;
pub use network_coordinate::NetworkCoordinate2D;
pub use network_coordinate::NetworkCoordinate3D;
