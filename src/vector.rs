//! A minimal Euclidean/Cartesian vector implementation.
//!
//! This is the simplest possible implementation of a vector to support the needs of a
//! Vivaldi `HeightVector`.

use num_traits::Float;

use std::ops::{Add, Div, Index, Mul, Sub};

//
// **** Structs ****
//

/// A minimal Euclidean/Cartesian vector implementation
///
/// ## Generic parameters:
/// - `T`: the type to use, must satisfy the `Float` trait (i.e. `f32` or `f64`)
/// - `N`: the number of dimensions (i.e. 2 for 2D vectors, 3 for 3D, etc)
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) struct Vector<T: Float, const N: usize>([T; N]);

//
// **** Implementations ****
//

impl<T: Float, const N: usize> Vector<T, N> {
    /// Create a new Zero vector.
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Compute magnitude of vector.
    pub(crate) fn len(&self) -> T {
        (0..N)
            .map(|i| self.0[i] * self.0[i])
            .fold(T::zero(), |acc, x| acc + x)
            .sqrt()
    }
}

//
// **** Trait Implementations ****
//

impl<T: Float, const N: usize> Default for Vector<T, N> {
    /// A default vector is all zeroes
    fn default() -> Self {
        Self([T::zero(); N])
    }
}

impl<T: Float, const N: usize> From<[T; N]> for Vector<T, N> {
    /// Create a new Vector from an array of type T, N dimensions.
    fn from(value: [T; N]) -> Self {
        Self(value)
    }
}

impl<T: Float, const N: usize> Add for Vector<T, N> {
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

impl<T: Float, const N: usize> Sub for Vector<T, N> {
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

impl<T: Float, const N: usize> Mul<T> for Vector<T, N> {
    type Output = Self;

    /// Vector*scalar multiplication.
    fn mul(self, rhs: T) -> Self::Output {
        let mut ret = Self::Output::new();
        for i in 0..N {
            ret.0[i] = self.0[i] * rhs
        }
        ret
    }
}

impl<T: Float, const N: usize> Div<T> for Vector<T, N> {
    type Output = Self;

    /// Vector/scalar division.
    fn div(self, rhs: T) -> Self::Output {
        let mut ret = Self::Output::new();
        for i in 0..N {
            ret.0[i] = self.0[i] / rhs
        }
        ret
    }
}

impl<T: Float, const N: usize> Index<usize> for Vector<T, N> {
    type Output = T;

    /// Nicer indexing [] for read only references.
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

//
// **** Tests ****
//

#[cfg(test)]
mod tests {
    use super::*;

    use assert_approx_eq::assert_approx_eq;

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
}
