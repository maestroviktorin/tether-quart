use crate::model::{State, TetheredSystem};

pub struct Rkf45Solver {
    pub tol_abs: f64,
    pub tol_rel: f64,
    pub h_min: f64,
    pub h_max: f64,
}

impl Rkf45Solver {
    pub fn new(tol_abs: f64, tol_rel: f64, h_min: f64, h_max: f64) -> Self {
        Self {
            tol_abs,
            tol_rel,
            h_min,
            h_max,
        }
    }

    // TODO: Use `anyhow`.
    /// Returns: `(next_state: State, h_used: f64, h_next: f64)`.
    pub fn adaptive_step(
        &self,
        system: &TetheredSystem,
        state: &State,
        t: f64,
        h_current: f64,
    ) -> Result<(State, f64, f64), &'static str> {
        let mut h = h_current.clamp(self.h_min, self.h_max);

        loop {
            let k1 = system.right_hand_side(state, t)? * h;
            let k2 = system.right_hand_side(&(*state + k1 * 0.25), t + 0.25 * h)? * h;
            let k3 = system.right_hand_side(
                &(*state + k1 * (3.0 / 32.0) + k2 * (9.0 / 32.0)),
                t + (3.0 / 8.0) * h,
            )? * h;
            let k4 = system.right_hand_side(
                &(*state + k1 * (1932.0 / 2197.0) - k2 * (7200.0 / 2197.0)
                    + k3 * (7296.0 / 2197.0)),
                t + (12.0 / 13.0) * h,
            )? * h;
            let k5 = system.right_hand_side(
                &(*state + k1 * (439.0 / 216.0) - k2 * 8.0 + k3 * (3680.0 / 513.0)
                    - k4 * (845.0 / 4104.0)),
                t + h,
            )? * h;
            let k6 = system.right_hand_side(
                &(*state - k1 * (8.0 / 27.0) + k2 * 2.0 - k3 * (3544.0 / 2565.0)
                    + k4 * (1859.0 / 4104.0)
                    - k5 * (11.0 / 40.0)),
                t + 0.5 * h,
            )? * h;

            let y_next =
                *state + k1 * (25.0 / 216.0) + k3 * (1408.0 / 2565.0) + k4 * (2197.0 / 4101.0)
                    - k5 * 0.2;
            let z_next =
                *state + k1 * (16.0 / 135.0) + k3 * (6656.0 / 12825.0) + k4 * (28561.0 / 56430.0)
                    - k5 * (9.0 / 50.0)
                    + k6 * (2.0 / 55.0);

            let calc_error = |val_z: f64, val_y: f64, val_init: f64| {
                let tol = self.tol_abs + self.tol_rel * val_init.abs();
                (val_z - val_y).abs() / tol
            };

            let e = calc_error(z_next.v, y_next.v, state.v)
                .max(calc_error(z_next.l, y_next.l, state.l))
                .max(calc_error(z_next.omega, y_next.omega, state.omega))
                .max(calc_error(z_next.theta, y_next.theta, state.theta));

            let s = (if e > 0.0 {
                0.84 * (1.0 / e).powf(0.25)
            } else {
                1.5
            })
            .clamp(0.1, 1.5);

            let h_next = (s * h).clamp(self.h_min, self.h_max);

            if e <= 1.0 {
                return Ok((z_next, h, h_next));
            }

            h = h_next;
            if h < self.h_min {
                return Err("Integration step became extremely small.");
            }
        }
    }
}
