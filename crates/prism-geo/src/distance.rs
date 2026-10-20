// © 2026 aiaiaiai · aiaiaiai.org
// SPDX-License-Identifier: Apache-2.0

/// Mean Earth radius in km.
pub const EARTH_RADIUS_KM: f64 = 6371.0088;

/// Great-circle distance between two points in km.
pub fn haversine_km(lat_a: f64, lon_a: f64, lat_b: f64, lon_b: f64) -> f64 {
    let (phi_a, phi_b) = (lat_a.to_radians(), lat_b.to_radians());
    let d_phi = phi_b - phi_a;
    let d_lambda = (lon_b - lon_a).to_radians();
    let a =
        (d_phi / 2.0).sin().powi(2) + phi_a.cos() * phi_b.cos() * (d_lambda / 2.0).sin().powi(2);
    2.0 * EARTH_RADIUS_KM * a.sqrt().asin()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn haversine_matches_a_known_distance() {
        // Kyiv to Lviv is about 470 km.
        let d = haversine_km(50.4501, 30.5234, 49.8397, 24.0297);
        assert!((465.0..480.0).contains(&d), "{d}");
        assert!(haversine_km(1.0, 2.0, 1.0, 2.0).abs() < 1e-9);
    }

    #[test]
    fn distance_is_symmetric() {
        let (a, b) = ((50.4501, 30.5234), (49.8397, 24.0297));
        assert_eq!(
            haversine_km(a.0, a.1, b.0, b.1),
            haversine_km(b.0, b.1, a.0, a.1)
        );
    }
}
