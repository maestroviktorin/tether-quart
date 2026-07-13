use std::sync::mpsc::{Receiver, Sender};

use crate::{
    app::{SimulationCmd, SimulationUpdate},
    model::{State, TetheredSystem},
    rkf45::Rkf45Solver,
};

pub fn spawn_worker(rx_cmd: Receiver<SimulationCmd>, tx_update: Sender<SimulationUpdate>) {
    std::thread::spawn(move || {
        let mut active_sim: Option<(TetheredSystem, State, Rkf45Solver, f64, f64)> = None; // t: f64, h: f64
        let mut paused = false;

        loop {
            if let Ok(cmd) = rx_cmd.try_recv() {
                match cmd {
                    SimulationCmd::Start(params, init_state, solver) => {
                        let system = TetheredSystem::new(params);
                        active_sim = Some((system, init_state, solver, 0.0, 0.01));

                        paused = false;
                    }
                    SimulationCmd::Pause => paused = true,
                    SimulationCmd::Resume => paused = false,
                    SimulationCmd::Reset => active_sim = None,
                }
            }

            if let Some((ref system, ref mut state, ref solver, ref mut t, ref mut h)) = active_sim
            {
                if !paused {
                    match solver.adaptive_step(system, state, *t, *h) {
                        Ok((next_state, h_used, h_next)) => {
                            *state = next_state;
                            *t += h_used;
                            *h = h_next;

                            let tension = system.tether_tension(state);

                            let _ = tx_update.send(SimulationUpdate {
                                t: *t,
                                state: *state,
                                tension,
                                h_used,
                            });
                        }
                        Err(_) => active_sim = None,
                    }
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    });
}
