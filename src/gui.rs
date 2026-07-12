use std::sync::mpsc::{Receiver, Sender};

use crate::{
    model::{State, SystemParameters},
    rkf45::Rkf45Solver,
};

#[derive(Debug)]
pub enum SimulationCmd {
    Start(SystemParameters, State, Rkf45Solver),
    Pause,
    Reset,
}

#[derive(Debug)]
pub struct SimulationUpdate {
    pub t: f64,
    pub state: State,
    pub tension: f64,
    pub h_used: f64,
}

pub struct App {
    m: f64,
    l_k: f64,
    k_l: f64,
    k_v: f64,
    tx_cmd: Sender<SimulationCmd>,
    rx_update: Receiver<SimulationUpdate>,
    pub history: Vec<SimulationUpdate>,
    is_running: bool,
}

impl App {
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        tx_cmd: Sender<SimulationCmd>,
        rx_update: Receiver<SimulationUpdate>,
    ) -> Self {
        let mut visuals = egui::Visuals::light();
        visuals.window_corner_radius = 4.0.into();
        cc.egui_ctx.set_visuals(visuals);

        Self {
            m: 120.0,
            l_k: 500.0,
            k_l: 1.5,
            k_v: 10.0,
            tx_cmd,
            rx_update,
            history: Vec::new(),
            is_running: false,
        }
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Not Yet Implemented");
        });
    }
}
