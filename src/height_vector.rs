//! Implementation of a Vivaldi height vector or coordinate.
//!
//! A Vivaldi height vector consists of an N-dimensional Euclidean/Cartesian space vector and a
//! greater-than-zero height. This vector is intented to represent latency; hence the distance
//! between two Vivaldi coordinates should be close to the actual round-trip-time (i.e. ping time).
//!
//! The height component is included to help solve for the triangle inequality; the fact that three
//! points in the real world won't necessarily follow the triangle equality principle. In other
//! words, ping times aren't perfectly correlated to geographic distance. Thus, the height can be
//! considered a representation of a sort of "stem time" which is the time from a node to the core
//! internet; and the vectors can be considered locations on the core internet backbone where ping
//! times are more consistently correlated with geographic distance.
//!
//! Quoting the paper:
//!
//! > A height vector consists of a Euclidean coordinate augmented
//! > with a height. The Euclidean portion models a high-speed Internet
//! > core with latencies proportional to geographic distance, while the
//! > height models the time it takes packets to travel the access link
//! > from the node to the core. The cause of the access link latency may
//! > be queuing delay (as in the case of an oversubscribed cable line),
//! > low bandwidth (as in the case of DSL, cable modems, or telephone
//! > modems), or even the sheer length of the link (as in the case of
//! > long-distance fiber-optic cables).
//! > A packet sent from one node to another must travel the source
//! > node’s height, then travel in the Euclidean space, then travel the
//! > destination node’s height. Even if the two nodes have the same
//! > height, the distance between them is their Euclidean distance plus
//! > the two heights. This is the fundamental difference between height
//! > vectors and adding a dimension to the Euclidean space. Intuitively,
//! > packet transmission can only be done in the core, not above it.
//! > The height vector model is implemented by redefining the usual
//! > vector operations (note the + on the right hand side of the subtrac-
//! > tion equation):
//!
//! ```text
//! [x, xh] − [y, y] = [(x − y), xh + yh]
//! ∥∥∥[x, xh]∥∥∥ = ∥∥∥x∥∥∥ + xh
//! α × [x, xh] = [αx, αxh ]
//! ```
//!
//! > Each node has a positive height element in its coordinates, so that
//! > its height can always be scaled up or down.

use core::ops::{Add, Mul, Sub};

use rand::prelude::*;
use serde::{Deserialize, Serialize};

use crate::vector::{self, Vector};

//
// **** Features ****
//

/// `FloatType` is a type alias for either `f32` or `f64` depending on cargo features
#[cfg(feature = "f32")]
type FloatType = f32;

/// `FloatType` is a type alias for either `f32` or `f64` depending on cargo features
#[cfg(not(feature = "f32"))]
type FloatType = f64;

//
// **** Constants ****
//

//
// **** Structs ****
//

/// The `HeightVector` is a tuple containing an `N`-dimensional Cartesian vector and a `> 0` height.
///
/// ## Generic Parameters
///
/// - `N`: the dimensionality of the vector portion (i.e. non-height) of the Vivaldi height vector
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct HeightVector<const N: usize> {
    /// `position` is the Euclidean coordinate part of the `HeightVector`, representing a position
    /// in the network's latency space
    #[serde(flatten)]
    position: Vector<FloatType, N>,

    /// `height` is a representation of a node's stemp time, or its latency to get into the network
    /// core (or backbone). It's a way to account for triangle inequality violations.
    /// in the network's latency space
    height: FloatType,
}

//
// **** Implementations ****
//

impl<const N: usize> HeightVector<N> {
    /// A new height vector is a random unit vector
    pub(crate) fn new() -> Self {
        Self::random()
    }

    /// A new height vector is a random unit vector
    pub(crate) fn random() -> Self {
        let mut rng = rand::thread_rng();
        let mut vec = [0.0; N];
        for i in vec.iter_mut().take(N) {
            *i = rng.gen::<FloatType>() - 0.5;
        }
        let height = rng.gen::<FloatType>().abs();
        Self {
            position: Vector::<FloatType, N>::from(vec),
            height,
        }
        .normalized()
    }

    /// The magnitude of a Vivaldi height vector is defined as the magnitude of the vector plus the
    /// height value.
    pub(crate) fn len(&self) -> FloatType {
        self.position.len() + self.height
    }

    /// A normalized Vivaldi height vector is just like a normalized vector: the vector scaled
    /// by the inveerse of its length.
    pub(crate) fn normalized(&self) -> Self {
        let len = self.len();
        // scale it by the inverse of its magnitude like any normal vector
        let ret = Self {
            position: self.position / len,
            height: self.height / len,
        };
        if ret.is_valid() {
            ret
        } else {
            // if we have a bad vector, generate a new random vector
            Self::new()
        }
    }

    /// Checks whether the `HeightVector` is valid.
    ///
    /// In this case, valid means the height is positive, and none of the components are NaN or
    /// Inf.
    pub(crate) fn is_valid(&self) -> bool {
        !self.is_invalid()
    }

    /// Checks whether the `HeightVector` is invalid.
    ///
    /// In this case, valid means the height is positive, and none of the components are NaN or
    /// Inf.
    pub(crate) fn is_invalid(&self) -> bool {
        self.position.is_invalid()
            || self.height.is_nan()
            || self.height.is_infinite()
            || self.height < 0.0
    }
}

//
// **** Trait Implementations ****
//

impl<const N: usize> Default for HeightVector<N> {
    /// Default value for a Vivaldi height vector is just the defaults of its children types.
    fn default() -> Self {
        Self {
            position: vector::Vector::default(),
            height: Default::default(),
        }
    }
}

impl<const N: usize> From<([FloatType; N], FloatType)> for HeightVector<N> {
    /// Convert from a `([FloatType; N], FloatType)` (vector, height) to a Vivaldi height vector type.
    fn from(value: ([FloatType; N], FloatType)) -> Self {
        let ret = Self {
            position: Vector::<FloatType, N>::from(value.0),
            height: value.1,
        };
        if ret.is_valid() {
            ret
        } else {
            Self::random()
        }
    }
}

impl<const N: usize> Add for HeightVector<N> {
    type Output = Self;

    /// Add two Vivaldi height vectors.
    fn add(self, rhs: Self) -> Self::Output {
        let ret = Self {
            position: self.position + rhs.position,
            height: self.height + rhs.height,
        };
        if ret.is_valid() {
            ret
        } else {
            Self::random()
        }
    }
}

impl<const N: usize> Sub for HeightVector<N> {
    type Output = Self;

    /// Subtract two Vivaldi height vectors. Note that this is the difference in the vectors
    /// and the summation of the heights, as defined by Vivaldi's author.
    fn sub(self, rhs: Self) -> Self::Output {
        let ret = Self {
            position: self.position - rhs.position,
            height: self.height + rhs.height,
        };
        if ret.is_valid() {
            ret
        } else {
            Self::random()
        }
    }
}

impl<const N: usize> Mul<FloatType> for HeightVector<N> {
    type Output = Self;

    /// Multiply a Vivaldi height vector by a scalar. Works the same as normal vector scaling.
    fn mul(self, rhs: FloatType) -> Self::Output {
        let ret = Self {
            position: self.position * rhs,
            height: self.height * rhs,
        };
        if ret.is_valid() {
            ret
        } else {
            Self::random()
        }
    }
}

//
// **** Tests ****
//
#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn proptest_len(x: FloatType, y: FloatType, h: FloatType) {
            let len = x.hypot(y) + h.abs();
            let a = HeightVector::<2>::from(([x,y],h));
            println!("a = {a:#?}");
            if x.is_nan() || x.is_infinite() || y.is_nan() || y.is_infinite() || h.is_nan() || h.is_infinite() || h < 0.0 {
                // we should've gottne a random univ vector here
                assert_approx_eq!(a.len(), 1.0);
            } else {
                // `HeightVector` we created from proptest values should be valid and have a length
                if len.is_nan() || len.is_infinite() {
                    assert!(a.len().is_nan() || a.len().is_infinite());
                } else {
                    assert_approx_eq!(a.len(), len);
                }
            }
        }

        #[test]
        fn proptest_add(x0 in -1_000_000_000..1_000_000_000i32, y0 in -1_000_000_000..1_000_000_000i32, h0 in 0..1_000_000_000i32, x1 in -1_000_000_000..1_000_000_000i32, y1 in -1_000_000_000..1_000_000_000i32, h1 in 0..1_000_000i32) {
            // convert our integer range inputs to FloatType
            let (fx0,fy0,fh0) = (x0 as FloatType / 1_000.0, y0 as FloatType / 1_000.0, h0 as FloatType / 1_000.0);
            let (fx1,fy1,fh1) = (x1 as FloatType / 1_000.0, y1 as FloatType / 1_000.0, h1 as FloatType / 1_000.0);

            let a = HeightVector::<2>::from(([fx0 ,fy0 ],fh0 ));
            let b = HeightVector::<2>::from(([fx1 ,fy1 ],fh1 ));
            let c = a+b;
            assert_approx_eq!(c.position[0], fx0 + fx1);
            assert_approx_eq!(c.position[1], fy0 + fy1);
            assert_approx_eq!(c.height, fh0 + fh1);
        }

        #[test]
        fn proptest_sub(x0 in -1_000_000_000..1_000_000_000i32, y0 in -1_000_000_000..1_000_000_000i32, h0 in 0..1_000_000_000i32, x1 in -1_000_000_000..1_000_000_000i32, y1 in -1_000_000_000..1_000_000_000i32, h1 in 0..1_000_000i32) {
            // convert our integer range inputs to FloatType
            let (fx0,fy0,fh0) = (x0 as FloatType / 1_000.0, y0 as FloatType / 1_000.0, h0 as FloatType / 1_000.0);
            let (fx1,fy1,fh1) = (x1 as FloatType / 1_000.0, y1 as FloatType / 1_000.0, h1 as FloatType / 1_000.0);

            let a = HeightVector::<2>::from(([fx0 ,fy0 ],fh0 ));
            let b = HeightVector::<2>::from(([fx1 ,fy1 ],fh1 ));
            let c = a-b;
            assert_approx_eq!(c.position[0], fx0 - fx1);
            assert_approx_eq!(c.position[1], fy0 - fy1);
            assert_approx_eq!(c.height, fh0 + fh1);
        }

        #[test]
        fn proptest_mul(x in -1_000_000_000..1_000_000_000i32, y in -1_000_000_000..1_000_000_000i32, h in 0..1_000_000_000i32, m in -1_000_000_000..1_000_000_000i32) {
            // convert our integer range inputs to FloatType
            let (fx,fy,fh,fm) = (x as FloatType / 1_000.0, y as FloatType / 1_000.0, h as FloatType / 1_000.0, m as FloatType / 1_000.0);

            let a = HeightVector::<2>::from(([fx, fy], fh));
            let b = a * fm;
            if fm < 0.0 {
                assert_approx_eq!(b.len(), 1.0);
            } else {
                assert_approx_eq!(b.position[0], fx * fm);
                assert_approx_eq!(b.position[1], fy * fm);
                assert_approx_eq!(b.height, fh * fm);
            }
        }
    }

    #[test]
    fn test_len() {
        let a = HeightVector::<3>::from(([1.0, 2.0, 3.0], 4.0));
        assert_approx_eq!(a.len(), 7.741_657, 0.00001);
    }

    #[test]
    fn test_new() {
        // new gives us a random unit length vec
        let a = HeightVector::<3>::new();
        assert_approx_eq!(a.len(), 1.0);
    }

    #[test]
    fn test_default() {
        // default() should give us a zero vec
        let a = HeightVector::<3>::default();
        assert_approx_eq!(a.len(), 0.0);
    }

    #[test]
    fn test_from() {
        let a = HeightVector::<2>::from(([1.0, 2.0], 3.0));
        assert_approx_eq!(a.position[0], 1.0);
        assert_approx_eq!(a.position[1], 2.0);
        assert_approx_eq!(a.height, 3.0);
    }

    #[test]
    fn test_add() {
        let a = HeightVector::<2>::from(([1.0, 2.0], 3.0));
        let b = HeightVector::<2>::from(([3.0, 2.0], 1.0));
        let c = a + b;
        assert_approx_eq!(c.position[0], 4.0);
        assert_approx_eq!(c.position[1], 4.0);
        assert_approx_eq!(c.height, 4.0);
    }

    #[test]
    fn test_sub() {
        let a = HeightVector::<2>::from(([1.0, 2.0], 3.0));
        let b = HeightVector::<2>::from(([3.0, 2.0], 1.0));
        let c = a - b;
        assert_approx_eq!(c.position[0], -2.0);
        assert_approx_eq!(c.position[1], 0.0);
        assert_approx_eq!(c.height, 4.0);
    }

    #[test]
    fn test_mul_scalar() {
        let a = HeightVector::<2>::from(([1.0, 2.0], 3.0)) * 10.0;
        assert_approx_eq!(a.position[0], 10.0);
        assert_approx_eq!(a.position[1], 20.0);
        assert_approx_eq!(a.height, 30.0);
    }

    #[test]
    fn test_serde() {
        // start with JSON, deserialize it
        let s = "{\"position\":[1.0,2.0,3.0],\"height\":4.0}";
        let a: HeightVector<3> = serde_json::from_str(s).unwrap();

        // make sure it's the right length and works like we expect a normal NC
        assert_approx_eq!(a.len(), 7.741_657, 0.001);

        // serialize it into a new JSON string and make sure it matches the original
        let t = serde_json::to_string(&a);
        assert_eq!(t.as_ref().unwrap(), s);
    }

    #[test]
    fn test_sub_invalid() {
        let valid = HeightVector::<2>::from(([1.0, 2.0], 3.0));
        let mut invalid = HeightVector::<2>::from(([1.0, 2.0], 3.0));
        // force invalidity here - because `::from()` actually also catches this
        invalid.height = 1.0 / 0.0;
        let result = valid - invalid;
        assert_approx_eq!(result.len(), 1.0);
    }

    #[test]
    fn test_add_invalid() {
        let valid = HeightVector::<2>::from(([1.0, 2.0], 3.0));
        let mut invalid = HeightVector::<2>::from(([1.0 / 0.0, 2.0], 3.0));

        // make sure `from()` caught and replaced the invalid vec
        assert_approx_eq!(invalid.len(), 1.0);

        // force invalidity here - because `::from()` actually also catches this
        invalid.height = 1.0 / 0.0;
        let result = valid + invalid;
        assert_approx_eq!(result.len(), 1.0);
    }

    #[test]
    fn test_zero_norm() {
        let a = HeightVector::<2>::from(([0.0, 0.0], 0.0));
        let b = a.normalized();
        let len = b.len();
        assert_approx_eq!(len, 1.0);
    }
}
