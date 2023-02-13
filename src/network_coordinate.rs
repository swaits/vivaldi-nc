//! Main interface module for Vivaldi network coordinates.
//!
//! For usage explanation and examples, please see the main [`crate`] documentation.

use core::time::Duration;

use serde::{Deserialize, Serialize};

use crate::height_vector::HeightVector;

//
// **** Features ****
//
#[cfg(feature = "f32")]
type FloatType = f32;

#[cfg(not(feature = "f32"))]
type FloatType = f64;

//
// **** Constants ****
//

// Vivaldi tuning parameters
const C_ERROR: FloatType = 0.25;
const C_DELTA: FloatType = 0.25;

// initial error value
const DEFAULT_ERROR: FloatType = 200.0;

// error should always be greater than zero
const MIN_ERROR: FloatType = FloatType::EPSILON;

//
// **** Structs ****
//

/// A `NetworkCoordinate<N>` is the main interface to a Vivaldi network coordinate.
///
/// # Generic Parameters
///
/// - `N`: Const generic for number of dimensions. For example, `NetworkCoordinate<3>` is a
/// 3-Dimentionsal Euclidean coordinate plus a height. Should be a positive number greater than
/// zero.
///
/// **Note:** Dimensions other than 2D or 3D are usually not useful. If you want to use one of
/// those dimensions, you can use type aliases ([`NetworkCoordinate2D`] or [`NetworkCoordinate3D`])
/// which are a little more ergonomic than using the generic here.
///
/// # Examples
///
/// For an explanation and examples of usage, please see the main [`crate`] documentation.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NetworkCoordinate<const N: usize> {
    #[serde(flatten)]
    heightvec: HeightVector<N>,
    error: FloatType,
}

// type aliases for convenience

/// A 2D [`NetworkCoordinate`]. Includes a 2D Euclidean position and a height.
///
/// This type alias is just for convenience. It's functionally equivalent to
/// `NetworkCoordinate<2>`. For more information, see [`NetworkCoordinate`].
pub type NetworkCoordinate2D = NetworkCoordinate<2>;

/// A 3D [`NetworkCoordinate`]. Includes a 3D Euclidean position and a height.
///
/// This type alias is just for convenience. It's functionally equivalent to
/// `NetworkCoordinate<3>`. For more information, see [`NetworkCoordinate`].
pub type NetworkCoordinate3D = NetworkCoordinate<3>;

//
// **** Implementations ****
//

impl<const N: usize> NetworkCoordinate<N> {
    /// Creates a new random [`NetworkCoordinate`]
    ///
    /// # Example
    ///
    /// ```
    /// use vivaldi_nc::NetworkCoordinate;
    ///
    /// // create a new 3-dimensional random NC
    /// let a: NetworkCoordinate<3> = NetworkCoordinate::new();
    ///
    /// // print the NC
    /// println!("Our new NC is: {:#?}", a);
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Given another Vivaldi [`NetworkCoordinate`], estimate the round trip time (ie ping) between them.
    ///
    /// This is done by computing the height vector distance between between the two coordinates.
    /// Vivaldi uses this distance as a representation of estimated round trip time.
    ///
    /// # Parameters
    ///
    /// - `rhs`: the other coordinate
    ///
    /// # Returns
    ///
    /// - the estimated round trip time as a `Duration`
    ///
    /// # Example
    ///
    /// ```
    /// use vivaldi_nc::NetworkCoordinate;
    ///
    /// // create some 2-dimensional NCs for the sake of this example. These will just be random
    /// // NCs. In a real usecase these would have meaningful values.
    /// let a: NetworkCoordinate<2> = NetworkCoordinate::new();
    /// let b: NetworkCoordinate<2> = NetworkCoordinate::new();
    ///
    /// // get the estimated RTT, convert to milliseconds, and print
    /// println!("Estimated RTT: {}", a.estimated_rtt(&b).as_millis());
    /// ```
    ///
    #[must_use]
    pub fn estimated_rtt(&self, rhs: &Self) -> Duration {
        // estimated rss is euclidean distance between the two plus the sum of the heights
        #[cfg(feature = "f32")]
        return Duration::from_secs_f32((self.heightvec - rhs.heightvec).len() / 1000.0);

        #[cfg(not(feature = "f32"))]
        return Duration::from_secs_f64((self.heightvec - rhs.heightvec).len() / 1000.0);
    }

    /// Given another Vivaldi [`NetworkCoordinate`], adjust our coordinateto better represent the actual round
    /// trip time (aka distance) between us.
    ///
    /// # Parameters
    ///
    /// - `rhs`: the other coordinate
    /// - `rtt`: the measured round trip time between `self` and `rhs`
    ///
    /// # Returns
    ///
    /// - a reference to `self`
    ///
    /// # Example
    ///
    /// ```
    /// use core::time::Duration;
    /// use vivaldi_nc::NetworkCoordinate;
    ///
    /// // We always have our own NC:
    /// let mut local: NetworkCoordinate<2> = NetworkCoordinate::new();
    ///
    /// // Assume we received a NC from a remote node:
    /// let remote: NetworkCoordinate<2> = NetworkCoordinate::new();
    ///
    /// // And we measured the RTT between us and the remote node:
    /// let rtt = Duration::from_millis(100);
    ///
    /// // Now we can update our NC to adjust our position relative to the remote node:
    /// local.update(&remote, rtt);
    /// ```
    ///
    /// # Algorithm
    ///
    /// This is an implementation of Vivaldi NCs per the original paper. It implements the following
    /// alogirthm (quoting from paper):
    ///
    /// ```text
    /// // Incorporate new information: node j has been
    /// // measured to be rtt ms away, has coordinates xj,
    /// // and an error estimate of ej .
    /// //
    /// // Our own coordinates and error estimate are xi and ei.
    /// //
    /// // The constants ce and cc are tuning parameters.
    ///
    /// vivaldi(rtt, xj, ej)
    ///     // Sample weight balances local and remote error. (1)
    ///     w = ei /(ei + ej)
    ///     // Compute relative error of this sample. (2)
    ///     es = ∣∣∣‖xi − xj‖ − rtt∣∣∣/rtt
    ///     // Update weighted moving average of local error. (3)
    ///     ei = es × ce × w + ei × (1 − ce × w)
    ///     // Update local coordinates. (4)
    ///     δ = cc × w
    ///     xi = xi + δ × (rtt − ‖xi − xj ‖) × u(xi − xj)
    /// ```
    ///
    pub fn update(&mut self, rhs: &Self, rtt: Duration) -> &Self {
        // convert Durations into FloatType as fractional milliseconds for convenience
        #[cfg(feature = "f32")]
        let rtt_ms = rtt.as_secs_f32() * 1000.0;
        #[cfg(feature = "f32")]
        let rtt_estimated_ms = self.estimated_rtt(rhs).as_secs_f32() * 1000.0;

        #[cfg(not(feature = "f32"))]
        let rtt_ms = rtt.as_secs_f64() * 1000.0;
        #[cfg(not(feature = "f32"))]
        let rtt_estimated_ms = self.estimated_rtt(rhs).as_secs_f64() * 1000.0;

        // rtt needs to be positive
        if rtt_ms < 0.0 {
            // Note: `rtt` is guaranteed to be positive because `Duration` enforces it.
            //       If this panics, something changed where `Duration` now allows for negative
            //       values.
            unreachable!();
        }

        // Sample weight balances local and remote error. (1)
        // w = ei /(ei + ej )
        let w = self.error / (self.error + rhs.error);

        // Compute relative error of this sample. (2)
        // es = ∣∣∣‖xi − xj‖ − rtt∣∣∣/rtt
        let error = rtt_ms - rtt_estimated_ms;
        let es = error.abs() / rtt_ms;

        // Update weighted moving average of local error. (3)
        // ei = es × ce × w + ei × (1 − ce × w)
        // self.error = (es * C_ERROR * w + self.error * (1.0 - C_ERROR * w)).max(MIN_ERROR);
        // NOTE: using `mul_add()` which is a little safer (avoid overflows)
        self.error = (es * C_ERROR)
            .mul_add(w, self.error * C_ERROR.mul_add(-w, 1.0))
            .max(MIN_ERROR);

        // Update local coordinates. (4)
        // δ = cc × w
        let delta = C_DELTA * w;
        // xi = xi + δ × (rtt − ‖xi − xj ‖) × u(xi − xj)
        self.heightvec =
            self.heightvec + (self.heightvec - rhs.heightvec).normalized() * delta * error;

        // if we ended up with an invalid coordinate, return a new random coordinate with default
        // error
        if self.heightvec.is_invalid() {
            *self = Self::new();

            // We should never get here because the call to `normalized()` above should catch any
            // invalid `heightvec`
            unreachable!();
        }

        // return reference to updated self
        self
    }

    /// getter for error value - useful for consumers to understand the estimated accuracty of this
    /// `NetworkCoordinate`
    #[must_use]
    pub const fn error(&self) -> FloatType {
        self.error
    }
}

//
// **** Trait Implementations ****
//

impl<const N: usize> Default for NetworkCoordinate<N> {
    /// A default `NetworkCoordinate` has a random position and `DEFAULT_ERROR`
    fn default() -> Self {
        Self {
            heightvec: HeightVector::<N>::random(),
            error: DEFAULT_ERROR,
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
    fn test_convergence() {
        let mut a = NetworkCoordinate::<3>::new();
        let mut b = NetworkCoordinate::<3>::new();
        let t = Duration::from_millis(250);
        (0..20).for_each(|_| {
            a.update(&b, t);
            b.update(&a, t);
        });
        let rtt = a.estimated_rtt(&b);
        assert_approx_eq!(rtt.as_secs_f32() * 1000.0, 250.0, 1.0);
    }

    #[test]
    fn test_mini_network() {
        // define a little network with these nodes:
        //
        // slc has 80ms stem time to core entry Seattle
        // nyc has 30ms stem time to core entry Virginia
        // lax has 15ms stem time to core entry Los Angeles
        // mad has 60ms stem time to core entry London
        //
        // we'll assume that traffic in the core moves at 50% the speed of light
        //
        // that gives us this grid of RTTs (in ms) for the core:
        //
        // |             | Seattle | Virgina | Los Angeles | London |
        // |-------------|---------|---------|-------------|--------|
        // | Seattle     |       - |      52 |          20 |    102 |
        // | Virginia    |         |       - |          50 |     78 |
        // | Los Angeles |         |         |           - |    116 |
        // | London      |         |         |             |      - |
        //
        // Which gives us these routes (plus their reverse) and times (ms):
        //
        // SLC -> Seattle -> Virginia -> NYC = 80 + 52 + 30 = 162
        // SLC -> Seattle -> Los Angeles -> LAX = 80 + 20 + 15 = 115
        // SLC -> Seattle -> Londong -> MAD = 80 + 102 + 60 = 242
        // NYC -> Virginia -> Los Angeles -> LAX = 30 + 50 + 15 = 95
        // NYC -> Virginia -> London -> MAD = 30 + 78 + 60 = 168
        // LAX -> Los Angeles -> London -> MAD = 15 + 116 + 60 = 192

        // create the NCs for each endpoint
        let mut slc = NetworkCoordinate::<2>::new();
        let mut nyc = NetworkCoordinate::<2>::new();
        let mut lax = NetworkCoordinate::<2>::new();
        let mut mad = NetworkCoordinate::<2>::new();

        // verify the initial error
        let error = slc.error.hypot(nyc.error.hypot(lax.error.hypot(mad.error)));
        assert_approx_eq!(error, 400.0);

        // iterate plenty of times to converge and minimize error
        (0..20).for_each(|_| {
            slc.update(&nyc, Duration::from_millis(162));
            nyc.update(&slc, Duration::from_millis(162));

            slc.update(&lax, Duration::from_millis(115));
            lax.update(&slc, Duration::from_millis(115));

            slc.update(&mad, Duration::from_millis(242));
            mad.update(&slc, Duration::from_millis(242));

            nyc.update(&lax, Duration::from_millis(95));
            lax.update(&nyc, Duration::from_millis(95));

            nyc.update(&mad, Duration::from_millis(168));
            mad.update(&nyc, Duration::from_millis(168));

            lax.update(&mad, Duration::from_millis(192));
            mad.update(&lax, Duration::from_millis(192));
        });

        // compute and test the root mean squared error
        let error = slc.error + nyc.error + lax.error + mad.error;
        assert!(error < 5.0);
    }

    #[test]
    fn test_serde() {
        // start with JSON, deserialize it
        let s = "{\"position\":[1.5,0.5,2.0],\"height\":0.1,\"error\":1.0}";
        let a: NetworkCoordinate<3> =
            serde_json::from_str(s).expect("deserialization failed during test");

        // make sure it's the right length and works like we expect a normal NC
        assert_approx_eq!(a.heightvec.len(), 2.649_509, 0.001);
        assert_approx_eq!(a.error, 1.0);
        assert_eq!(a.estimated_rtt(&a).as_millis(), 0);

        // serialize it into a new JSON string and make sure it matches the original
        let t = serde_json::to_string(&a);
        assert_eq!(t.as_ref().expect("serialization failed during test"), s);
    }

    #[test]
    fn test_estimated_rtt() {
        // start with JSON, deserialize it
        let s = "{\"position\":[1.5,0.5,2.0],\"height\":25.0,\"error\":1.0}";
        let a: NetworkCoordinate<3> =
            serde_json::from_str(s).expect("deserialization failed during test");
        let s = "{\"position\":[-1.5,-0.5,-2.0],\"height\":50.0,\"error\":1.0}";
        let b: NetworkCoordinate<3> =
            serde_json::from_str(s).expect("deserialization failed during test");

        let estimate = a.estimated_rtt(&b);
        assert_approx_eq!(estimate.as_secs_f32(), 0.080_099);
    }

    #[test]
    fn test_error_getter() {
        let s = "{\"position\":[1.5,0.5,2.0],\"height\":25.0,\"error\":1.0}";
        let a: NetworkCoordinate<3> =
            serde_json::from_str(s).expect("deserialization failed during test");
        assert_approx_eq!(a.error(), 1.0);
    }
}
