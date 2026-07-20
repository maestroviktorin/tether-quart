use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};

use crate::components::settings::boundary_values::BoundaryValues;

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsConfig {
    pub m: f64,
    pub f0: f64,
    pub phi: f64,
    pub l_k: f64,
    pub k_l: f64,
    pub k_v: f64,
    pub t1: f64,
    pub t2: f64,
    pub t3: f64,
    pub init_v: f64,
    pub init_l: f64,
    pub init_omega: f64,
    pub init_theta: f64,
    pub tol_abs: f64,
    pub tol_rel: f64,
    pub h_min: f64,
    pub h_max: f64,
}

impl Default for SettingsConfig {
    fn default() -> Self {
        Self {
            m: 25.0,
            f0: 5.0,
            phi: 0.0,
            l_k: 500.0,
            k_l: 1.5,
            k_v: 3.2,
            t1: 30.0,
            t2: 60.0,
            t3: 90.0,
            init_v: 0.1,
            init_l: 5.0,
            init_omega: 0.02,
            init_theta: 0.0,
            tol_abs: 1e-6,
            tol_rel: 1e-6,
            h_min: 1e-4,
            h_max: 1.0,
        }
    }
}

impl SettingsConfig {
    pub fn validate(&self) -> Result<()> {
        if self.m < BoundaryValues::MIN_M || self.m > BoundaryValues::MAX_M {
            return Err(anyhow!(
                "Condition failed: {} <= m <= {}\n(received m = {})",
                BoundaryValues::MIN_M,
                BoundaryValues::MAX_M,
                self.m
            ));
        }
        if self.f0 < BoundaryValues::MIN_F0 || self.f0 > BoundaryValues::MAX_F0 {
            return Err(anyhow!(
                "Condition failed: {} <= f0 <= {}\n(received f0 = {})",
                BoundaryValues::MIN_F0,
                BoundaryValues::MAX_F0,
                self.f0
            ));
        }
        if self.phi < BoundaryValues::MIN_PHI || self.phi > BoundaryValues::MAX_PHI {
            return Err(anyhow!(
                "Condition failed: {} <= phi <= {}\n(received phi = {})",
                BoundaryValues::MIN_PHI,
                BoundaryValues::MAX_PHI,
                self.phi
            ));
        }
        if self.l_k < BoundaryValues::MIN_L_K || self.l_k > BoundaryValues::MAX_L_K {
            return Err(anyhow!(
                "Condition failed: {} <= l_k <= {}\n(received l_k = {})",
                BoundaryValues::MIN_L_K,
                BoundaryValues::MAX_L_K,
                self.l_k
            ));
        }
        if self.k_l < BoundaryValues::MIN_K_L || self.k_l > BoundaryValues::MAX_K_L {
            return Err(anyhow!(
                "Condition failed: {} <= k_l <= {}\n(received k_l = {})",
                BoundaryValues::MIN_K_L,
                BoundaryValues::MAX_K_L,
                self.k_l
            ));
        }
        if self.k_v < BoundaryValues::MIN_K_V || self.k_v > BoundaryValues::MAX_K_V {
            return Err(anyhow!(
                "Condition failed: {} <= k_v <= {}\n(received k_v = {})",
                BoundaryValues::MIN_K_V,
                BoundaryValues::MAX_K_V,
                self.k_v
            ));
        }
        if self.t1 < 0.0 {
            return Err(anyhow!("Parameter t1 cannot be negative."));
        }
        if self.t2 < self.t1 + 1.0 {
            return Err(anyhow!(
                "Condition failed: t1 < t2. Difference must be at least 1.0\n(received t1 = {}, t2 = {}, t2 >= {} required)",
                self.t1,
                self.t2,
                self.t1 + 1.0
            ));
        }
        if self.t3 < self.t2 + 1.0 {
            return Err(anyhow!(
                "Condition failed: t2 < t3. Difference must be at least 1.0\n(received t2 = {}, t3 = {}, t3 >= {} required)",
                self.t2,
                self.t3,
                self.t2 + 1.0
            ));
        }
        if self.init_v < 0.0 {
            return Err(anyhow!("Parameter init_v cannot be negative."));
        }
        if self.init_l < 0.0 {
            return Err(anyhow!("Parameter init_l cannot be negative."));
        }
        if self.init_omega < 0.0 {
            return Err(anyhow!("Parameter init_omega cannot be negative."));
        }
        if self.init_theta < 0.0 {
            return Err(anyhow!("Parameter init_theta cannot be negative."));
        }
        if self.tol_abs < 0.0 {
            return Err(anyhow!("Parameter tol_abs cannot be negative."));
        }
        if self.tol_rel < 0.0 {
            return Err(anyhow!("Parameter tol_rel cannot be negative."));
        }
        if self.h_min < 0.0 {
            return Err(anyhow!("Parameter h_min cannot be negative."));
        }
        if self.h_max < 0.0 {
            return Err(anyhow!("Parameter h_max cannot be negative."));
        }
        anyhow::Ok(())
    }
}
