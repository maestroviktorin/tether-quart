use std::{
    f32::consts::FRAC_PI_2,
    sync::mpsc::{Receiver, Sender},
};

use egui::Rangef;
use egui_plot::{Line, Plot, PlotPoints};

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
        while let Ok(update) = self.rx_update.try_recv() {
            self.history.push(update);
        }

        egui::Panel::left("params_panel")
            .size_range(Rangef::new(150.0, 450.0))
            .show(ui, |ui| {
                ui.take_available_space();
                ui.heading("TSS Settings");
                ui.add(egui::Slider::new(&mut self.m, 10.0..=100.0).text("Mass m, kg"));
                // Possible TODO: Make all members of `SystemParameters` configurable parameters of `Self`.

                ui.separator();

                ui.heading("Control Law Settings");
                ui.add(
                    egui::Slider::new(&mut self.l_k, 10.0..=1500.0)
                        .text("Target tethers length l_k, m"),
                );
                ui.add(
                    egui::Slider::new(&mut self.k_l, 0.0..=10.0)
                        .text("Length regulation ratio, N/m"),
                );
                ui.add(
                    egui::Slider::new(&mut self.k_v, 0.0..=10.0)
                        .text("Velocity regulation ratio, N*s/m"),
                );

                ui.separator();

                ui.horizontal(|ui| {
                    if ui.button("Start").clicked() {
                        // TODO: Remove hard-coded values.
                        let params = SystemParameters {
                            m: self.m,
                            f0: 5.0,
                            phi: 0.0,
                            l_k: self.l_k,
                            k_l: self.k_l,
                            k_v: self.k_v,
                            t1: 12.0,
                            t2: 20.0,
                            t3: 29.0,
                        };
                        let init_state = State::new(0.1, 5.0, 0.02, 0.0);
                        let solver = Rkf45Solver::new(1e-6, 1e-6, 1e-4, 1.0);
                        let _ = self
                            .tx_cmd
                            .send(SimulationCmd::Start(params, init_state, solver));
                        self.is_running = true;
                    }
                    if ui.button("Pause").clicked() {
                        let _ = self.tx_cmd.send(SimulationCmd::Pause);
                        self.is_running = false;
                    }
                    if ui.button("Reset").clicked() {
                        let _ = self.tx_cmd.send(SimulationCmd::Reset);
                        self.history.clear();
                        self.is_running = false;
                    }
                })
            });

        egui::Panel::top("scheme_panel")
            .resizable(true)
            .default_size(300.0)
            .show(ui, |ui| {
                ui.take_available_space();
                ui.heading("TSS Scheme");
                let last_state = self.history.last().map(|u| u.state);

                let size = egui::vec2(ui.available_width(), ui.available_height());
                let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());
                let painter = ui.painter_at(rect);
                let center = rect.center();

                if let Some(state) = last_state {
                    let r = state.l / f64::sqrt(2.0);
                    let scale = 0.4 * f64::from(rect.width()) / self.l_k;
                    let angles =
                        [0.0, 1.0, 2.0, 3.0].map(|i| state.theta + i * std::f64::consts::FRAC_PI_2);
                    let points: Vec<egui::Pos2> = angles
                        .iter()
                        .map(|&a| {
                            center
                                + egui::vec2(
                                    (r * a.cos() * scale) as f32,
                                    (r * a.sin() * scale) as f32,
                                )
                        })
                        .collect();
                    let tension = self.history.last().map(|u| u.tension).unwrap_or(0.0);
                    let line_color = if tension == 0.0 {
                        egui::Color32::from_rgb(100, 149, 237)
                    } else if tension > 50.0 {
                        egui::Color32::from_rgb(220, 20, 60)
                    } else {
                        egui::Color32::from_rgb(50, 205, 50)
                    };

                    for i in 0..4 {
                        painter.line_segment([points[i], points[(i + 1) % 4]], (2.0, line_color));
                        painter.circle_filled(points[i], 8.0, egui::Color32::GRAY);
                    }
                }
            });

        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Graph: l(t)");
            let points: PlotPoints = self.history.iter().map(|u| [u.t, u.state.l]).collect();
            let line = Line::new("l(t)", points);
            Plot::new("len_plot")
                .height(200.0)
                .show(ui, |plot_ui| plot_ui.line(line));
        });

        if self.is_running {
            ui.request_repaint();
        }
    }
}
