//! A minimal Euclidean/Cartesian vector implementation.
//!
//! This is the simplest possible implementation of a vector to support the needs of a
//! Vivaldi `HeightVector`.

use array_init::array_init;
use num_traits::Float;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use std::ops::{Add, Div, Index, Mul, Sub};

//
// **** Structs ****
//

/// A minimal Euclidean/Cartesian vector implementation
///
/// ## Generic parameters:
/// - `T`: the type to use, must satisfy the `Float` trait (i.e. `f32` or `f64`)
/// - `N`: the number of dimensions (i.e. 2 for 2D vectors, 3 for 3D, etc)
#[serde_as]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Vector<T, const N: usize>
where
    T: Float + Serialize + for<'d> Deserialize<'d>,
{
    #[serde(rename = "position")]
    #[serde_as(as = "[_; N]")]
    inner: [T; N],
}

//
// **** Implementations ****
//

impl<T, const N: usize> Vector<T, N>
where
    T: Float + Serialize + for<'d> Deserialize<'d>,
{
    /// Compute magnitude of vector.
    pub(crate) fn len(&self) -> T {
        // use hypot() instead of sqrt() of sum of squares to avoid overflows
        self.inner.iter().fold(T::zero(), |acc, x| acc.hypot(*x))
    }

    /// Checks whether the `Vector` is invalid.
    ///
    /// In this case, valid means tnone of the components are NaN or Inf.
    pub(crate) fn is_invalid(&self) -> bool {
        self.inner.iter().any(|x| x.is_nan() || x.is_infinite())
    }
}

//
// **** Trait Implementations ****
//

impl<T, const N: usize> Default for Vector<T, N>
where
    T: Float + Serialize + for<'d> Deserialize<'d>,
{
    /// A default vector is all zeroes
    fn default() -> Self {
        Self {
            inner: [T::zero(); N],
        }
    }
}

impl<T, const N: usize> From<[T; N]> for Vector<T, N>
where
    T: Float + Serialize + for<'d> Deserialize<'d>,
{
    /// Create a new Vector from an array of type T, N dimensions.
    fn from(value: [T; N]) -> Self {
        Self { inner: value }
    }
}

impl<T, const N: usize> Add for Vector<T, N>
where
    T: Float + Serialize + for<'d> Deserialize<'d>,
{
    type Output = Self;

    /// Vector+Vector addition.
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            inner: array_init(|i| self[i] + rhs[i]),
        }
    }
}

impl<T, const N: usize> Sub for Vector<T, N>
where
    T: Float + Serialize + for<'d> Deserialize<'d>,
{
    type Output = Self;

    /// Vector-Vector subtraction.
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            inner: array_init(|i| self[i] - rhs[i]),
        }
    }
}

impl<T, const N: usize> Mul<T> for Vector<T, N>
where
    T: Float + Serialize + for<'d> Deserialize<'d>,
{
    type Output = Self;

    /// Vector*scalar multiplication.
    fn mul(self, rhs: T) -> Self::Output {
        Self::Output {
            inner: array_init(|i| self[i] * rhs),
        }
    }
}

impl<T, const N: usize> Div<T> for Vector<T, N>
where
    T: Float + Serialize + for<'d> Deserialize<'d>,
{
    type Output = Self;

    /// Vector/scalar division.
    fn div(self, rhs: T) -> Self::Output {
        Self::Output {
            inner: array_init(|i| self[i] / rhs),
        }
    }
}

impl<T, const N: usize> Index<usize> for Vector<T, N>
where
    T: Float + Serialize + for<'d> Deserialize<'d>,
{
    type Output = T;

    /// Nicer indexing [] for read only references.
    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
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
        fn proptest_new(x: f32, y: f32, z: f32){
            let v = Vector::<f32, 3>::from([x,y,z]);
            assert_approx_eq!(v[0], x);
            assert_approx_eq!(v[1], y);
            assert_approx_eq!(v[2], z);
            assert!(v[0].is_finite());
            assert!(v[1].is_finite());
            assert!(v[1].is_finite());
        }

        #[test]
        fn proptest_mul(x: f32, y: f32, z: f32, m: f32) {
            let v = Vector::<f32, 3>::from([x,y,z]);
            let w = v * m;

            let (x,y,z) = (v[0]*m, v[1]*m, v[2]*m);
            if x.is_nan() || x.is_infinite() {
                assert!(w[0].is_nan() || w[0].is_infinite());
            } else {
                assert_approx_eq!(w[0], x);
            }

            if y.is_nan() || y.is_infinite() {
                assert!(w[1].is_nan() || w[1].is_infinite());
            } else {
                assert_approx_eq!(w[1], y);
            }

            if z.is_nan() || z.is_infinite() {
                assert!(w[2].is_nan() || w[2].is_infinite());
            } else {
                assert_approx_eq!(w[2], z);
            }
        }

        #[test]
        fn proptest_div(x: f32, y: f32, z: f32, d: f32) {
            let v = Vector::<f32, 3>::from([x,y,z]);
            let w = v / d;

            let (x,y,z) = (v[0]/d, v[1]/d, v[2]/d);
            if x.is_nan() || x.is_infinite() {
                assert!(w[0].is_nan() ||w[0].is_infinite());
            } else {
                assert_approx_eq!(w[0], x);
            }

            if y.is_nan() || y.is_infinite() {
                assert!(w[1].is_nan() || w[1].is_infinite());
            } else {
                assert_approx_eq!(w[1], y);
            }

            if z.is_nan() || z.is_infinite() {
                assert!(w[2].is_nan() || w[2].is_infinite());
            } else {
                assert_approx_eq!(w[2], z);
            }
        }
    }

    #[test]
    fn test_default_new() {
        let a = Vector::<f32, 3>::default();
        let b = Vector::<f32, 3>::from([1.0; 3]) * 0.0;
        let c = Vector::<f32, 3>::from([0.0; 3]);
        assert_eq!(a, b);
        assert_eq!(b, c);
    }

    #[test]
    fn test_add() {
        let a = Vector::<f32, 3>::from([1.0, 2.0, 3.0]);
        let b = Vector::<f32, 3>::from([3.0, 2.0, 1.0]);
        let c = a + b;
        assert_eq!(c, Vector::<f32, 3>::from([4.0, 4.0, 4.0]));
    }

    #[test]
    fn test_sub() {
        let a = Vector::<f32, 3>::from([1.0, 2.0, 3.0]);
        let b = Vector::<f32, 3>::from([3.0, 2.0, 1.0]);
        let c = a - b;
        assert_eq!(c, Vector::<f32, 3>::from([-2.0, 0.0, 2.0]));
    }

    #[test]
    fn test_mul_scalar() {
        let a = Vector::<f32, 3>::from([1.0, 2.0, 3.0]);
        let b = 10.0;
        let c = a * b;
        assert_eq!(c, Vector::<f32, 3>::from([10.0, 20.0, 30.0]));
    }

    #[test]
    fn test_div_scalar() {
        let a = Vector::<f32, 3>::from([1.0, 2.0, 3.0]);
        let b = 10.0;
        let c = a / b;
        assert_eq!(c, Vector::<f32, 3>::from([0.1, 0.2, 0.3]));
    }

    #[test]
    fn test_index() {
        let a = Vector::<f32, 3>::from([1.0, 2.0, 3.0]);
        assert_approx_eq!(a[0], 1.0);
        assert_approx_eq!(a[1], 2.0);
        assert_approx_eq!(a[2], 3.0);
    }
    #[test]
    fn test_length() {
        let a = Vector::<f32, 3>::from([1.0, 2.0, 3.0]);
        assert_approx_eq!(a.len(), 3.741_65, 0.0001);

        let b = a * 10.0;
        assert_approx_eq!(b.len(), 37.416_57, 0.0001);
    }

    #[test]
    fn test_serde() {
        let a = Vector::<f32, 3>::from([1.0, 2.0, 3.0]);
        let s = serde_json::to_string(&a);
        assert_eq!(s.as_ref().unwrap(), "{\"position\":[1.0,2.0,3.0]}");
        assert!(s.is_ok());
    }
}
