use egui::{Color32, RichText};
use egui_file_dialog::FileDialog;
use egui_plot::{Line, Plot, PlotPoints};
use std::path::Path;

use crate::{app::SimulationUpdate, components::common::process_error_window};

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
    pub file_dialog: FileDialog,
    pub export_error_message: Option<String>,
    pub add_error_message: Option<String>,
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
            file_dialog: FileDialog::new(),
            export_error_message: None,
            add_error_message: None,
        }
    }
}

pub fn render(
    ui: &mut egui::Ui,
    _frame: &mut eframe::Frame,
    history: &[SimulationUpdate],
    plots: &mut PlotsComponent,
) {
    enum Action {
        MoveUp(usize),
        MoveDown(usize),
        Delete(usize),
        Export(usize),
    }

    let mut action: Option<Action> = None;

    ui.vertical(|ui| {
        let len = plots.plots.len();
        plots.plots.iter().enumerate().for_each(|(i, plot)| {
            ui.horizontal(|ui| {
                egui::CollapsingHeader::new(RichText::new(plot.title()).heading())
                    .default_open(true)
                    .show(ui, |ui| {
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

                            if ui.button("📸").clicked() {
                                action = Some(Action::Export(i));
                            }
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
            });
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
                if i < plots.plots.len() - 1 {
                    plots.plots.swap(i, i + 1);
                }
            }
            Action::Delete(i) => {
                if i < plots.plots.len() {
                    plots.plots.remove(i);
                }
            }
            Action::Export(i) => {
                let suggested_name = format!("{}.png", plots.plots[i].title());
                plots.file_dialog = FileDialog::new()
                    .default_file_name(&suggested_name)
                    .add_file_filter(
                        "PNG Image",
                        egui_file_dialog::Filter::new(|path: &Path| {
                            path.extension().unwrap_or_default() == "png"
                        }),
                    )
                    .default_file_filter("PNG Image");

                plots.file_dialog.set_user_data(i);
                plots.file_dialog.save_file();
            }
        }
    }

    plots.file_dialog.update(ui.ctx());

    if let Some(path) = plots.file_dialog.take_picked() {
        if let Some(&plot_idx) = plots.file_dialog.user_data::<usize>() {
            if plot_idx < plots.plots.len() {
                let plot = &plots.plots[plot_idx];

                let points_data: Vec<[f64; 2]> = history
                    .iter()
                    .map(|u| [plot.x_var.get_value(u), plot.y_var.get_value(u)])
                    .collect();

                if let Err(err) = save_plot_to_png(
                    &path,
                    &plot.title(),
                    plot.x_var.as_str(),
                    plot.y_var.as_str(),
                    &points_data,
                ) {
                    plots.export_error_message =
                        Some(format!("Failed to export the plot: {:?}", err));
                }
            }
        }
    }

    process_error_window(ui, "Plot Export Error", &mut plots.export_error_message);
    process_error_window(ui, "Plot Addition Error", &mut plots.add_error_message);

    render_add_modal(plots, ui.ctx());
}

fn save_plot_to_png(
    path: &Path,
    title: &str,
    x_label: &str,
    y_label: &str,
    points: &[[f64; 2]],
) -> anyhow::Result<()> {
    use plotters::prelude::*;

    if points.is_empty() {
        return Err(anyhow::anyhow!("No points to plot"));
    }

    let root = BitMapBackend::new(path, (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut min_x = points[0][0];
    let mut max_x = points[0][0];
    let mut min_y = points[0][1];
    let mut max_y = points[0][1];

    for p in points.iter().skip(1) {
        min_x = min_x.min(p[0]);
        max_x = max_x.max(p[0]);
        min_y = min_y.min(p[1]);
        max_y = max_y.max(p[1]);
    }

    let x_margin = if max_x == min_x {
        1.0
    } else {
        (max_x - min_x) * 0.05
    };
    let y_margin = if max_y == min_y {
        1.0
    } else {
        (max_y - min_y) * 0.05
    };

    let x_range = (min_x - x_margin)..(max_x + x_margin);
    let y_range = (min_y - y_margin)..(max_y + y_margin);

    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("monospace", 35).into_font())
        .margin(20)
        .x_label_area_size(50)
        .y_label_area_size(60)
        .build_cartesian_2d(x_range, y_range)?;

    chart
        .configure_mesh()
        .x_desc(x_label)
        .y_desc(y_label)
        .axis_desc_style(("monospace", 20).into_font())
        .draw()?;

    chart.draw_series(LineSeries::new(
        points.iter().map(|p| (p[0], p[1])),
        &RGBColor(255, 0, 0),
    ))?;

    root.present()?;
    Ok(())
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
                        let is_duplicate = plots
                            .plots
                            .iter()
                            .any(|p| p.x_var == plots.new_plot_x && p.y_var == plots.new_plot_y);

                        if is_duplicate {
                            plots.add_error_message = Some(format!(
                                "Plot {}({}) already exists.",
                                plots.new_plot_y.as_str(),
                                plots.new_plot_x.as_str()
                            ));
                        } else {
                            plots.plot_counter += 1;
                            plots.plots.push(PlotComponent {
                                id_source: format!("plot_{:?}", plots.plot_counter),
                                x_var: plots.new_plot_x,
                                y_var: plots.new_plot_y,
                            });
                            plots.show_add_modal = false;
                        }
                    }
                });
            });
        });
    if !open {
        plots.show_add_modal = false;
    }
}
