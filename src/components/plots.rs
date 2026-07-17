use egui::{Color32, RichText};
use egui_plot::{Line, Plot, PlotPoints};

use crate::app::SimulationUpdate;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlotVariable {
    T,
    V,
    L,
    Omega,
    Theta,
    Tension,
    HUsed,
}

impl PlotVariable {
    pub const ALL: &[Self] = &[
        Self::T,
        Self::V,
        Self::L,
        Self::Omega,
        Self::Theta,
        Self::Tension,
        Self::HUsed,
    ];

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::T => "t",
            Self::V => "v",
            Self::L => "l",
            Self::Omega => "omega",
            Self::Theta => "theta",
            Self::Tension => "tension",
            Self::HUsed => "h_used",
        }
    }

    pub fn get_value(&self, u: &SimulationUpdate) -> f64 {
        match self {
            Self::T => u.t,
            Self::V => u.state.v,
            Self::L => u.state.l,
            Self::Omega => u.state.omega,
            Self::Theta => u.state.theta,
            Self::Tension => u.tension,
            Self::HUsed => u.h_used,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlotComponent {
    pub id_source: String,
    pub x_var: PlotVariable,
    pub y_var: PlotVariable,
}

impl PlotComponent {
    pub fn title(&self) -> String {
        format!("{}({})", self.y_var.as_str(), self.x_var.as_str())
    }
}

pub struct PlotsComponent {
    pub plots: Vec<PlotComponent>,
    pub show_add_modal: bool,
    pub new_plot_x: PlotVariable,
    pub new_plot_y: PlotVariable,
    plot_counter: usize,
}

impl Default for PlotsComponent {
    fn default() -> Self {
        Self {
            plots: vec![PlotComponent {
                id_source: "plot_1".to_string(),
                x_var: PlotVariable::T,
                y_var: PlotVariable::L,
            }],
            show_add_modal: Default::default(),
            new_plot_x: PlotVariable::T,
            new_plot_y: PlotVariable::Tension,
            plot_counter: 1,
        }
    }
}

pub fn render(
    ui: &mut egui::Ui,
    frame: &mut eframe::Frame,
    history: &[SimulationUpdate],
    plots: &mut PlotsComponent,
) {
    enum Action {
        MoveUp(usize),
        MoveDown(usize),
        Delete(usize),
    }

    let mut action: Option<Action> = None;

    ui.vertical(|ui| {
        let len = plots.plots.len();
        plots.plots.iter().enumerate().for_each(|(i, plot)| {
            ui.horizontal(|ui| {
                ui.heading(plot.title());

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("❌").clicked() {
                        action = Some(Action::Delete(i));
                    }

                    if i < len - 1 {
                        if ui.button("🔽").clicked() {
                            action = Some(Action::MoveDown(i));
                        }
                    }

                    if i > 0 {
                        if ui.button("🔼").clicked() {
                            action = Some(Action::MoveUp(i));
                        }
                    }
                });
            });

            let points: PlotPoints = history
                .iter()
                .map(|u| [plot.x_var.get_value(u), plot.y_var.get_value(u)])
                .collect();

            let line = Line::new(plot.title(), points);

            Plot::new(&plot.id_source)
                .height(200.0)
                .show(ui, |plot_ui| plot_ui.line(line));

            ui.add_space(10.0);
        });

        if len > 0 {
            ui.separator();
        }

        if ui.button("Add Plot").clicked() {
            plots.show_add_modal = true;
        }
    });

    if let Some(act) = action {
        match act {
            Action::MoveUp(i) => {
                if i > 0 {
                    plots.plots.swap(i, i - 1);
                }
            }
            Action::MoveDown(i) => {
                if i < plots.plots.len() {
                    plots.plots.swap(i, i + 1);
                }
            }
            Action::Delete(i) => {
                if i < plots.plots.len() {
                    plots.plots.remove(i);
                }
            }
        }
    }

    render_add_modal(plots, ui.ctx());
}

fn render_add_modal(plots: &mut PlotsComponent, ctx: &egui::Context) {
    if !plots.show_add_modal {
        return;
    }

    let mut open = true;
    egui::Window::new("Add Plot")
        .open(&mut open)
        .collapsible(false)
        .resizable(false)
        .movable(true)
        .drag_area(egui::WindowDrag::TitleBar)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label("X-axis:");
                    egui::ComboBox::from_id_salt("new_plot_x")
                        .selected_text(plots.new_plot_x.as_str())
                        .show_ui(ui, |ui| {
                            PlotVariable::ALL.iter().for_each(|var| {
                                ui.selectable_value(&mut plots.new_plot_x, *var, var.as_str());
                            });
                        });
                });

                ui.horizontal(|ui| {
                    ui.label("Y-axis:");
                    egui::ComboBox::from_id_salt("new_plot_y")
                        .selected_text(plots.new_plot_y.as_str())
                        .show_ui(ui, |ui| {
                            PlotVariable::ALL.iter().for_each(|var| {
                                ui.selectable_value(&mut plots.new_plot_y, *var, var.as_str());
                            });
                        });
                });

                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    if ui
                        .button(RichText::new("Cancel").color(Color32::RED))
                        .clicked()
                    {
                        plots.show_add_modal = false;
                    }

                    if ui.button("Confirm").clicked() {
                        plots.plot_counter += 1;
                        plots.plots.push(PlotComponent {
                            id_source: format!("plot_{:?}", plots.plot_counter),
                            x_var: plots.new_plot_x,
                            y_var: plots.new_plot_y,
                        });
                        plots.show_add_modal = false;
                    }
                });
            });
        });
    if !open {
        plots.show_add_modal = false;
    }
}
