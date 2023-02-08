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

use std::ops::{Add, Mul, Sub};

use nanorand::{Rng, WyRand};
use serde::{Deserialize, Serialize};

use crate::vector::Vector;

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
pub(crate) struct HeightVector<const N: usize>(Vector<f32, N>, f32);

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
        let mut rng = WyRand::new();
        let mut vec = [0.0; N];
        for i in vec.iter_mut().take(N) {
            *i = rng.generate::<f32>();
        }
        let height = rng.generate::<f32>().abs();
        Self(Vector::<f32, N>::from(vec), height).normalized()
    }

    /// The magnitude of a Vivaldi height vector is defined as the magnitude of the vector plus the
    /// height value.
    pub(crate) fn len(&self) -> f32 {
        self.0.len() + self.1
    }

    /// A normalized Vivaldi height vector is just like a normalized vector: the vector scaled
    /// by the inveerse of its length.
    pub(crate) fn normalized(&self) -> Self {
        let len = self.len();
        if len < f32::EPSILON {
            // if we have a bad vector, generate a new random vector
            Self::new()
        } else {
            // otherwise scale it by the inverse of its magnitude like any normal vector
            Self(self.0 / len, self.1 / len)
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
        self.1.is_sign_negative() || self.1.is_nan() || self.1.is_infinite() || self.0.is_invalid()
    }
}

//
// **** Trait Implementations ****
//

impl<const N: usize> Default for HeightVector<N> {
    /// Default value for a Vivaldi height vector is just the defaults of its children types.
    fn default() -> Self {
        Self(Default::default(), Default::default())
    }
}

impl<const N: usize> From<([f32; N], f32)> for HeightVector<N> {
    /// Convert from a `([f32; N], f32)` (vector, height) to a Vivaldi height vector type.
    fn from(value: ([f32; N], f32)) -> Self {
        let ret = Self(Vector::<f32, N>::from(value.0), value.1);
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
        let ret = Self(self.0 + rhs.0, self.1 + rhs.1);
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
        let ret = Self(self.0 - rhs.0, self.1 + rhs.1);
        if ret.is_valid() {
            ret
        } else {
            Self::random()
        }
    }
}

impl<const N: usize> Mul<f32> for HeightVector<N> {
    type Output = Self;

    /// Multiply a Vivaldi height vector by a scalar. Works the same as normal vector scaling.
    fn mul(self, rhs: f32) -> Self::Output {
        let ret = Self(self.0 * rhs, self.1 * rhs);
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
    use assert_approx_eq::assert_approx_eq;

    use super::*;

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
        assert_eq!(a.0[0], 1.0);
        assert_eq!(a.0[1], 2.0);
        assert_eq!(a.1, 3.0);
    }

    #[test]
    fn test_add() {
        let a = HeightVector::<2>::from(([1.0, 2.0], 3.0));
        let b = HeightVector::<2>::from(([3.0, 2.0], 1.0));
        let c = a + b;
        assert_eq!(c.0[0], 4.0);
        assert_eq!(c.0[1], 4.0);
        assert_eq!(c.1, 4.0);
    }

    #[test]
    fn test_sub() {
        let a = HeightVector::<2>::from(([1.0, 2.0], 3.0));
        let b = HeightVector::<2>::from(([3.0, 2.0], 1.0));
        let c = a - b;
        assert_eq!(c.0[0], -2.0);
        assert_eq!(c.0[1], 0.0);
        assert_eq!(c.1, 4.0);
    }

    #[test]
    fn test_mul_scalar() {
        let a = HeightVector::<2>::from(([1.0, 2.0], 3.0)) * 10.0;
        assert_eq!(a.0[0], 10.0);
        assert_eq!(a.0[1], 20.0);
        assert_eq!(a.1, 30.0);
    }

    #[test]
    fn test_serde() {
        // start with JSON, deserialize it
        let s = "[{\"inner\":[1.0,2.0,3.0]},4.0]";
        let a: HeightVector<3> = serde_json::from_str(s).unwrap();

        // make sure it's the right length and works like we expect a normal NC
        assert_approx_eq!(a.len(), 7.741_657, 0.001);

        // serialize it into a new JSON string and make sure it matches the original
        let t = serde_json::to_string(&a);
        assert_eq!(t.as_ref().unwrap(), s);
    }
}
