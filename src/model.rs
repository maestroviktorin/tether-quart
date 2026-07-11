use std::ops::Add;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct SystemParameters {
    /// Microsatellite mass (*kg*)
    pub m: f64,

    /// Thrust force (*N*)
    pub f0: f64,

    /// Force direction angle (*rad*)
    pub phi: f64,

    /// Target tethers length (*m*)
    pub l_k: f64,

    /// Length regulation ratio (*N/m*)
    pub k_l: f64,

    /// Velocity regulation ratio (*N\*s/m*)
    pub k_v: f64,

    /// Time point #1.
    pub t1: f64,

    /// Time point #2.
    pub t2: f64,

    /// Time point #3.
    pub t3: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct State {
    /// Length rate of change (*m/s*)
    pub v: f64,

    /// Tethers length (*m*)
    pub l: f64,

    /// Angle velocity (*rad/s*)
    pub omega: f64,

    /// Orientation angle (*rad*)
    pub theta: f64,
}

impl State {
    pub fn new(v: f64, l: f64, omega: f64, theta: f64) -> Self {
        Self { v, l, omega, theta }
    }
}

impl std::ops::Add<State> for State {
    type Output = Self;

    fn add(self, rhs: State) -> Self::Output {
        Self {
            v: self.v + rhs.v,
            l: self.l + rhs.l,
            omega: self.omega + rhs.omega,
            theta: self.theta + rhs.theta,
        }
    }
}

impl std::ops::Mul<f64> for State {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            v: rhs * self.v,
            l: rhs * self.l,
            omega: rhs * self.omega,
            theta: rhs * self.theta,
        }
    }
}

impl std::ops::Sub<State> for State {
    type Output = Self;

    fn sub(self, rhs: State) -> Self::Output {
        self.add(rhs * -1.0f64)
    }
}

pub struct TetheredSystem {
    pub params: SystemParameters,
}

impl TetheredSystem {
    pub fn new(params: SystemParameters) -> Self {
        Self { params }
    }

    #[inline]
    pub fn thrust_force(&self, t: f64) -> f64 {
        if t <= self.params.t1 {
            self.params.f0
        } else if t <= self.params.t2 {
            0.0
        } else if t <= self.params.t3 {
            self.params.f0
        } else {
            0.0
        }
    }

    #[inline]
    pub fn tether_tension(&self, state: &State) -> f64 {
        let tension = 0.5
            * (self.params.m * state.l * state.omega.powi(2)
                + self.params.k_l * (state.l - self.params.l_k)
                + self.params.k_v * state.v);

        if tension < 0.0 { 0.0 } else { tension }
    }

    // TODO: Use `anyhow`.
    pub fn right_hand_side(&self, state: &State, t: f64) -> Result<State, &'static str> {
        if state.l <= 0.0 {
            return Err("Tether length must be positive.");
        }

        let f = self.thrust_force(t);
        let tension = self.tether_tension(state);
        let m = self.params.m;
        let phi_offset = self.params.phi + std::f64::consts::FRAC_PI_4;

        // dV/dt
        let dv_dt = state.l * state.omega.powi(2)
            - f64::sqrt(2.0) * (f / m) * f64::cos(phi_offset)
            - 2.0 * tension / m;

        // dl/dt
        let dl_dt = state.v;

        // d_omega/dt
        let domega_dt = f64::sqrt(2.0) * (f / (state.l * m)) * f64::sin(phi_offset)
            - 2.0 * (state.v / state.l) * state.omega;

        // d_theta/dt
        let dtheta_dt = state.omega;

        Ok(State::new(dv_dt, dl_dt, domega_dt, dtheta_dt))
    }
}
