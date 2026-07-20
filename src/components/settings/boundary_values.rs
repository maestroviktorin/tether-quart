pub struct BoundaryValues;

impl BoundaryValues {
    pub const MIN_M: f64 = 10.0;
    pub const MAX_M: f64 = 100.0;
    pub const MIN_F0: f64 = 0.0;
    pub const MAX_F0: f64 = 100.0;
    pub const MIN_PHI: f64 = 0.0;
    pub const MAX_PHI: f64 = 359.0;
    pub const MIN_L_K: f64 = 10.0;
    pub const MAX_L_K: f64 = 1500.0;
    pub const MIN_K_L: f64 = 0.0;
    pub const MAX_K_L: f64 = 10.0;
    pub const MIN_K_V: f64 = 0.0;
    pub const MAX_K_V: f64 = 10.0;

    pub const MIN_TOL_DECIMALS: usize = 6;
    pub const MAX_TOL_DECIMALS: usize = 7;
    pub const MIN_H_DECIMALS: usize = 1;
    pub const MAX_H_DECIMALS: usize = 5;
}
