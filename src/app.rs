use std::{
    rc::Rc,
    sync::mpsc::{Receiver, Sender},
};

use egui::Rangef;

use crate::{
    components::{self, about::AboutComponent, plots::PlotsComponent, settings::SettingsComponent},
    model::{State, SystemParameters},
    rkf45::Rkf45Solver,
};

#[derive(Debug)]
pub enum SimulationCmd {
    Start(SystemParameters, State, Rkf45Solver),
    Pause,
    Resume,
    Reset,
}

#[derive(Default, PartialEq)]
pub enum SimulationState {
    #[default]
    Stopped,
    Running,
    Paused,
}

#[derive(Debug)]
pub struct SimulationUpdate {
    pub t: f64,
    pub state: State,
    pub tension: f64,
    pub h_used: f64,
}

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub enum Tab {
    #[default]
    Dashboard,
    About,
}

pub struct App {
    active_tab: Tab,
    about: AboutComponent,
    settings: SettingsComponent,
    plots: PlotsComponent,
    simulation_state: SimulationState,
    tx_cmd: Sender<SimulationCmd>,
    rx_update: Receiver<SimulationUpdate>,
    pub history: Vec<SimulationUpdate>,
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
            active_tab: Tab::default(),
            about: AboutComponent::default(),
            settings: SettingsComponent::default(),
            plots: PlotsComponent::default(),
            simulation_state: SimulationState::default(),
            tx_cmd,
            rx_update,
            history: Vec::new(),
        }
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        // egui_extras::install_image_loaders(ui);

        egui::Panel::top("top_navigation_bar").show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.active_tab, Tab::Dashboard, "Dashboard");
                ui.selectable_value(&mut self.active_tab, Tab::About, "About");
            });
        });

        egui::CentralPanel::default().show(ui, |ui| match self.active_tab {
            Tab::Dashboard => dashboard(self, ui, frame),
            Tab::About => about(self, ui, frame),
        });
    }
}

fn dashboard(app: &mut App, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
    while let Ok(update) = app.rx_update.try_recv() {
        app.history.push(update);
    }

    egui::Panel::left("settings_panel")
        .size_range(Rangef::new(150.0, 450.0))
        .show(ui, |ui| {
            ui.take_available_space();

            components::settings::render(ui, frame, &mut app.settings);
            ui.separator();

            components::buttons::render(
                ui,
                frame,
                &app.settings,
                &mut app.simulation_state,
                &mut app.tx_cmd,
                &mut app.history,
            );
        });

    egui::Panel::top("tss_scheme_panel")
        .resizable(true)
        .default_size(300.0)
        .show(ui, |ui| {
            ui.take_available_space();
            components::tss_scheme::render(ui, frame, &app.history, &app.settings);
        });

    egui::CentralPanel::default().show(ui, |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.take_available_space();
            components::plots::render(ui, frame, &app.history, &mut app.plots);
        });
    });

    if app.simulation_state == SimulationState::Running {
        ui.request_repaint();
    }
}

fn about(app: &mut App, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
    let math_cache = Rc::clone(&app.about.math_cache);

    egui::CentralPanel::default().show(ui, |ui| {
        components::about::render(ui, frame, &mut app.about.common_mark_cache, math_cache);
    });
}
