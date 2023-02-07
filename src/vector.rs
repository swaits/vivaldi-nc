//! A minimal Euclidean/Cartesian vector implementation.
//!
//! This is the simplest possible implementation of a vector to support the needs of a
//! Vivaldi `HeightVector`.

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
pub(crate) struct Vector<T, const N: usize>
where
    T: Float + Serialize + for<'d> Deserialize<'d>,
{
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
    /// Create a new Zero vector.
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Compute magnitude of vector.
    pub(crate) fn len(&self) -> T {
        (0..N)
            .map(|i| self.inner[i] * self.inner[i])
            .fold(T::zero(), |sum_sq, x| sum_sq + x)
            .sqrt()
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
        let mut ret = [T::zero(); N]; //Self::Output::new();
        for i in 0..N {
            ret[i] = self[i] + rhs[i];
        }
        Self::Output::from(ret)
    }
}

impl<T, const N: usize> Sub for Vector<T, N>
where
    T: Float + Serialize + for<'d> Deserialize<'d>,
{
    type Output = Self;

    /// Vector-Vector subtraction.
    fn sub(self, rhs: Self) -> Self::Output {
        let mut ret = [T::zero(); N]; //Self::Output::new();
        for i in 0..N {
            ret[i] = self[i] - rhs[i];
        }
        Self::Output::from(ret)
    }
}

impl<T, const N: usize> Mul<T> for Vector<T, N>
where
    T: Float + Serialize + for<'d> Deserialize<'d>,
{
    type Output = Self;

    /// Vector*scalar multiplication.
    fn mul(self, rhs: T) -> Self::Output {
        let mut ret = Self::Output::new();
        for i in 0..N {
            ret.inner[i] = self.inner[i] * rhs;
            if !ret.inner[i].is_normal() {
                ret.inner[i] = T::zero();
            }
        }
        ret
    }
}

impl<T, const N: usize> Div<T> for Vector<T, N>
where
    T: Float + Serialize + for<'d> Deserialize<'d>,
{
    type Output = Self;

    /// Vector/scalar division.
    fn div(self, rhs: T) -> Self::Output {
        let mut ret = Self::Output::new();
        for i in 0..N {
            ret.inner[i] = self.inner[i] / rhs;
            if !ret.inner[i].is_normal() {
                ret.inner[i] = T::zero();
            }
        }
        ret
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
    use num_traits::Zero;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn proptest_new(x: f32, y: f32, z: f32){
            let v = Vector::<f32, 3>::from([x,y,z]);
            assert_eq!(v[0], x);
            assert_eq!(v[1], y);
            assert_eq!(v[2], z);
            assert!(v[0].is_finite());
            assert!(v[1].is_finite());
            assert!(v[1].is_finite());
        }

        #[test]
        fn proptest_mul(x: f32, y: f32, z: f32, m: f32) {
            let v = Vector::<f32, 3>::from([x,y,z]);
            let w = v * m;
            if m.is_zero() {
                assert_eq!(w[0], 0.0);
                assert_eq!(w[1], 0.0);
                assert_eq!(w[2], 0.0);
            }
            assert!(v[0].is_finite());
            assert!(v[1].is_finite());
            assert!(v[1].is_finite());
            assert!(w[0].is_finite());
            assert!(w[1].is_finite());
            assert!(w[1].is_finite());
        }
        #[test]
        fn proptest_div(x: f32, y: f32, z: f32, d: f32) {
            let v = Vector::<f32, 3>::from([x,y,z]);
            let w = v / d;
            if d.is_zero() {
                assert_eq!(w[0], 0.0);
                assert_eq!(w[1], 0.0);
                assert_eq!(w[2], 0.0);
            }
            assert!(v[0].is_finite());
            assert!(v[1].is_finite());
            assert!(v[1].is_finite());
            assert!(w[0].is_finite());
            assert!(w[1].is_finite());
            assert!(w[1].is_finite());
        }
    }

    #[test]
    fn test_default_new() {
        let a = Vector::<f32, 3>::default();
        let b = Vector::<f32, 3>::new();
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
        assert_eq!(a[0], 1.0);
        assert_eq!(a[1], 2.0);
        assert_eq!(a[2], 3.0);
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
        assert_eq!(s.as_ref().unwrap(), "{\"inner\":[1.0,2.0,3.0]}");
        assert!(s.is_ok());
    }
}
