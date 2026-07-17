use std::sync::mpsc::Sender;

use crate::{
    app::{SimulationCmd, SimulationState, SimulationUpdate},
    components::settings::SettingsComponent,
    model::{State, SystemParameters},
    rkf45::Rkf45Solver,
};

pub fn render(
    ui: &mut egui::Ui,
    frame: &mut eframe::Frame,
    settings: &SettingsComponent,
    simulation_state: &mut SimulationState,
    tx_cmd: &mut Sender<SimulationCmd>,
    history: &mut Vec<SimulationUpdate>,
) {
    let can_start = *simulation_state == SimulationState::Stopped;
    let can_pause = *simulation_state == SimulationState::Running;
    let can_resume = *simulation_state == SimulationState::Paused;
    let can_reset =  true /* *simulation_state == SimulationState::Paused
        || *simulation_state == SimulationState::Running */;

    ui.horizontal(|ui| {
        if ui
            .add_enabled(can_start, egui::Button::new("Start"))
            .clicked()
        {
            let params = SystemParameters {
                m: settings.config.m,
                f0: settings.config.f0,
                phi: settings.config.phi,
                l_k: settings.config.l_k,
                k_l: settings.config.k_l,
                k_v: settings.config.k_v,
                t1: settings.config.t1,
                t2: settings.config.t2,
                t3: settings.config.t3,
            };
            let init_state = State::new(
                settings.config.init_v,
                settings.config.init_l,
                settings.config.init_omega,
                settings.config.init_theta,
            );
            let solver = Rkf45Solver::new(
                settings.config.tol_abs,
                settings.config.tol_rel,
                settings.config.h_min,
                settings.config.h_max,
            );
            let _ = tx_cmd.send(SimulationCmd::Start(params, init_state, solver));
            *simulation_state = SimulationState::Running;
        }
        if ui
            .add_enabled(can_pause, egui::Button::new("Pause"))
            .clicked()
        {
            let _ = tx_cmd.send(SimulationCmd::Pause);
            *simulation_state = SimulationState::Paused;
        }
        if ui
            .add_enabled(can_resume, egui::Button::new("Resume"))
            .clicked()
        {
            let _ = tx_cmd.send(SimulationCmd::Resume);
            *simulation_state = SimulationState::Running;
        }
        if ui
            .add_enabled(can_reset, egui::Button::new("Reset"))
            .clicked()
        {
            let _ = tx_cmd.send(SimulationCmd::Reset);
            history.clear();
            *simulation_state = SimulationState::Stopped;
        }
    });
}
