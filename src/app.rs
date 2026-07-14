use std::{
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
    sync::{
        Arc,
        mpsc::{Receiver, Sender},
    },
};

use egui::Rangef;
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};
use egui_plot::{Line, Plot, PlotPoints};

use crate::{
    components::{self, settings::SettingsComponent},
    model::{State, SystemParameters},
    rkf45::Rkf45Solver,
};

const ABOUT: &str = include_str!("assets/markdown/about.md");

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
    common_mark_cache: CommonMarkCache,
    math_cache: Rc<RefCell<HashMap<String, Arc<[u8]>>>>,
    settings: SettingsComponent,
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
            common_mark_cache: CommonMarkCache::default(),
            math_cache: Rc::new(RefCell::new(HashMap::new())),
            settings: SettingsComponent::default(),
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

    egui::Panel::left("params_panel")
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

    egui::Panel::top("scheme_panel")
        .resizable(true)
        .default_size(300.0)
        .show(ui, |ui| {
            ui.take_available_space();
            ui.heading("TSS Scheme");
            let last_state = app.history.last().map(|u| u.state);

            let size = egui::vec2(ui.available_width(), ui.available_height());
            let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());
            let painter = ui.painter_at(rect);
            let center = rect.center();

            if let Some(state) = last_state {
                let r = state.l / f64::sqrt(2.0);
                let scale = 0.4 * f64::from(rect.width()) / app.settings.l_k;
                let angles =
                    [0.0, 1.0, 2.0, 3.0].map(|i| state.theta + i * std::f64::consts::FRAC_PI_2);
                let points: Vec<egui::Pos2> = angles
                    .iter()
                    .map(|&a| {
                        center
                            + egui::vec2((r * a.cos() * scale) as f32, (r * a.sin() * scale) as f32)
                    })
                    .collect();
                let tension = app.history.last().map(|u| u.tension).unwrap_or(0.0);
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
        ui.heading("Plot: l(t)");
        let points: PlotPoints = app.history.iter().map(|u| [u.t, u.state.l]).collect();
        let line = Line::new("l(t)", points);
        Plot::new("len_plot")
            .height(200.0)
            .show(ui, |plot_ui| plot_ui.line(line));
    });

    if app.simulation_state == SimulationState::Running {
        ui.request_repaint();
    }
}

fn about(app: &mut App, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
    let math_cache = Rc::clone(&app.math_cache);

    egui::CentralPanel::default().show(ui, |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.take_available_space();
            CommonMarkViewer::new()
                .render_math_fn(Some(&move |ui, math_str, inline| {
                    let math_string = math_str.to_string();

                    let id = egui::Id::from(math_string.clone());
                    let uri = format!("bytes://math_{}.svg", id.value());

                    let svg_bytes = {
                        let mut cache = math_cache.borrow_mut();

                        cache.entry(math_string.clone())
                            .or_insert_with(move || {
                                compile_latex_to_svg(&math_string, inline)
                                .unwrap_or_else(|err| {
                                    Arc::from(format!(
                                        r#"<svg xmlns="http://www.w3.org/2000/svg"><text y="15" fill="red">Error: {}</text></svg>"#,
                                        err
                                    ).into_bytes())
                                })
                            })
                            .clone()
                            
                    };

                    ui.add(
                        egui::Image::new(egui::ImageSource::Bytes {
                            uri: uri.into(),
                            bytes: egui::load::Bytes::Shared(svg_bytes),
                        })
                        .fit_to_original_size(0.33),
                    );
                }))
                .show(ui, &mut app.common_mark_cache, ABOUT);
        });
    });
}

// TODO: Use `anyhow`.
fn compile_latex_to_svg(latex: &str, inline: bool) -> Result<Arc<[u8]>, String> {
    let parsed = ratex_parser::parse(latex).map_err(|e| format!("LaTeX parse error: {:?}", e))?;

    let layout_options = ratex_layout::LayoutOptions::default();
    let layout_box = ratex_layout::layout(&parsed, &layout_options);
    let display_list = ratex_layout::to_display_list(&layout_box);

    let mut svg_options = ratex_svg::SvgOptions::default();
    svg_options.embed_glyphs = true;

    let svg_string = ratex_svg::render_to_svg(&display_list, &svg_options);

    Ok(Arc::from(svg_string.into_bytes()))
}
