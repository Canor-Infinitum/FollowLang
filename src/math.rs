/*
 * Copyright (c) 2026 Canor Aether Infinitum
 *
 * SPDX-License-Identifier: GNU AGPL v3.0-or-later
 */

use std::f64::consts::PI;

pub const L_P: f64 = 0.07881256452824544;

/// Flows: Fundamental Flow-scales
/// r_s := 2 * l_p * sqrt(pi * (q + sqrt(q * (1 + q))))
pub fn calculate_flow_scale(q: f64) -> f64 {
    let inner = q + (q * (1.0 + q)).sqrt();
    2.0 * L_P * (PI * inner).sqrt()
}

/// Actions: Action-Symbols within arbitrary differentiations F
/// returns C_N1 * (dF/df)^2 - C_N2 * F * (d2F/df^2)
pub fn calculate_action_moduli(c_n1: f64, c_n2: f64, f: f64, df: f64, d2f: f64) -> f64 {
    c_n1 * df.powi(2) - c_n2 * f * d2f
}

/// Maneuvers-Group: delta area of black hole
/// DELTA(A) = 32 * pi^2 * l_p^2 + 64 * pi^3 * (l_p^4 / r_s^2)
pub fn calculate_maneuver_delta(r_s: f64) -> f64 {
    let t1 = 32.0 * PI.powi(2) * L_P.powi(2);
    let t2 = 64.0 * PI.powi(3) * (L_P.powi(4) / r_s.powi(2));
    t1 + t2
}

/// Elite Programming: n-spheres with fixed radius r_f over length scale L
/// r_f = L * sqrt(pi * (n + sqrt(n * (1 + n))))
pub fn calculate_elite_radius(n: f64, l: f64) -> f64 {
    let inner = n + (n * (1.0 + n)).sqrt();
    l * (PI * inner).sqrt()
}

/// Follower Language: complementary sphere structure
/// p = (l_p^2 / r_s) / N
pub fn calculate_follower_sphere(n: f64, r_s: f64) -> f64 {
    (L_P.powi(2) / r_s) / n
}

/// Ether Language: machine epsilon mitigation
/// eps := upd_rate / err_rate
pub fn calculate_ether_epsilon(upd_rate: f64, err_rate: f64) -> f64 {
    if err_rate == 0.0 {
        0.0
    } else {
        upd_rate / err_rate
    }
}
