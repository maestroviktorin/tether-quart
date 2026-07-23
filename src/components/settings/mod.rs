mod boundary_values;
mod settings_config;

use std::path::Path;

use egui_file_dialog::FileDialog;

use crate::components::{
    common::process_error_window,
    settings::{boundary_values::BoundaryValues, settings_config::SettingsConfig},
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum DialogAction {
    Save,
    Load,
}

pub struct SettingsComponent {
    pub config: SettingsConfig,
    file_dialog: FileDialog,
    error_message: Option<String>,
}

impl Default for SettingsComponent {
    fn default() -> Self {
        Self {
            config: SettingsConfig::default(),
            file_dialog: FileDialog::new()
                .default_file_name("config.json")
                .add_file_filter(
                    "JSON Config",
                    egui_file_dialog::Filter::new(|path: &Path| {
                        path.extension().unwrap_or_default() == "json"
                    }),
                )
                .default_file_filter("JSON Config"),
            error_message: None,
        }
    }
}

pub fn render(ui: &mut egui::Ui, frame: &mut eframe::Frame, settings: &mut SettingsComponent) {
    egui::CollapsingHeader::new("TSS Settings")
        .default_open(true)
        .show(ui, |ui| {
            render_tss_settings(ui, &mut settings.config);
        });
    ui.separator();

    egui::CollapsingHeader::new("Control Law Settings")
        .default_open(true)
        .show(ui, |ui| {
            render_control_law_settings(ui, &mut settings.config);
        });
    ui.separator();

    egui::CollapsingHeader::new("Initial State Settings")
        .default_open(true)
        .show(ui, |ui| {
            render_initial_state_settings(ui, &mut settings.config);
        });
    ui.separator();

    egui::CollapsingHeader::new("RKF45 Settings").show(ui, |ui| {
        render_rkf45_settings(ui, &mut settings.config);
    });

    ui.separator();

    egui::CollapsingHeader::new("Save & Load Configuration")
        .default_open(true)
        .show(ui, |ui| {
            render_save_load_buttons(ui, &mut settings.file_dialog);
        });

    settings.file_dialog.update(ui.ctx());

    process_save_load(settings);

    process_error_window(ui, "Load Config Error", &mut settings.error_message);
}

fn render_tss_settings(ui: &mut egui::Ui, config: &mut SettingsConfig) {
    ui.add(
        egui::Slider::new(&mut config.m, BoundaryValues::MIN_M..=BoundaryValues::MAX_M)
            .suffix(" kg")
            .text("Mass, m"),
    );
    ui.add(
        egui::Slider::new(
            &mut config.f0,
            BoundaryValues::MIN_F0..=BoundaryValues::MAX_F0,
        )
        .suffix(" N")
        .text("Thrust force, f0"),
    );
    ui.add(
        egui::Slider::new(
            &mut config.phi,
            BoundaryValues::MIN_PHI..=BoundaryValues::MAX_PHI,
        )
        .suffix("°")
        .text("Force direction angle, phi"),
    );

    render_time_drag_values(ui, config);
}

fn render_time_drag_values(ui: &mut egui::Ui, config: &mut SettingsConfig) {
    ui.horizontal(|ui| {
        ui.add(egui::Label::new("t1:"));
        let dv1 = ui.add(egui::DragValue::new(&mut config.t1));

        ui.add(egui::Label::new("t2:"));
        let dv2 = ui.add(egui::DragValue::new(&mut config.t2));

        ui.add(egui::Label::new("t3:"));
        let dv3 = ui.add(egui::DragValue::new(&mut config.t3));

        if dv1.changed() {
            config.t1 = config.t1.max(0.0);
            config.t2 = config.t2.max(config.t1 + 1.0);
            config.t3 = config.t3.max(config.t2 + 1.0);
        }

        if dv2.changed() {
            config.t2 = config.t2.max(1.0);
            config.t1 = config.t1.min(config.t2 - 1.0);
            config.t3 = config.t3.max(config.t2 + 1.0);
        }

        if dv3.changed() {
            config.t3 = config.t3.max(2.0);
            config.t2 = config.t2.min(config.t3 - 1.0);
            config.t1 = config.t1.min(config.t2 - 1.0);
        }
    });
}

fn render_control_law_settings(ui: &mut egui::Ui, config: &mut SettingsConfig) {
    ui.add(
        egui::Slider::new(
            &mut config.l_k,
            BoundaryValues::MIN_L_K..=BoundaryValues::MAX_L_K,
        )
        .suffix(" m")
        .text("Target tethers length, l_k"),
    );
    ui.add(
        egui::Slider::new(
            &mut config.k_l,
            BoundaryValues::MIN_K_L..=BoundaryValues::MAX_K_L,
        )
        .text("Length regulation ratio, k_l"),
    );
    ui.add(
        egui::Slider::new(
            &mut config.k_v,
            BoundaryValues::MIN_K_V..=BoundaryValues::MAX_K_V,
        )
        .text("Velocity regulation ratio, k_v"),
    );
}

fn render_initial_state_settings(ui: &mut egui::Ui, config: &mut SettingsConfig) {
    ui.horizontal(|ui| {
        ui.add(egui::Label::new("Length rate of change, v:"));
        let dv_v = ui.add(egui::DragValue::new(&mut config.init_v).suffix(" m/s"));

        if dv_v.changed() {
            config.init_v = config.init_v.max(0.0);
        }
    });
    ui.horizontal(|ui| {
        ui.add(egui::Label::new("Tethers length, l:"));
        let dv_l = ui.add(egui::DragValue::new(&mut config.init_l).suffix(" m"));

        if dv_l.changed() {
            config.init_l = config.init_l.max(0.0);
        }
    });
    ui.horizontal(|ui| {
        ui.add(egui::Label::new("Angle velocity, omega:"));
        let dv_omega = ui.add(egui::DragValue::new(&mut config.init_omega).suffix(" rad/s"));

        if dv_omega.changed() {
            config.init_omega = config.init_omega.max(0.0);
        }
    });
    ui.horizontal(|ui| {
        ui.add(egui::Label::new("Orientation angle, theta:"));
        let dv_theta = ui.add(egui::DragValue::new(&mut config.init_theta).suffix(" rad"));

        if dv_theta.changed() {
            config.init_theta = config.init_theta.max(0.0);
        }
    });
}

fn render_rkf45_settings(ui: &mut egui::Ui, config: &mut SettingsConfig) {
    let min_h_min = 10.0_f64.powi(-(BoundaryValues::MAX_H_DECIMALS as i32));
    let min_h_max = 2.0 * min_h_min;

    ui.horizontal(|ui| {
        ui.add(egui::Label::new("Absolute tolerance:"));
        let dv_ta = ui.add(
            egui::DragValue::new(&mut config.tol_abs)
                .min_decimals(BoundaryValues::MIN_TOL_DECIMALS)
                .max_decimals(BoundaryValues::MAX_TOL_DECIMALS),
        );

        if dv_ta.changed() {
            config.tol_abs = config.tol_abs.max(0.0);
        }
    });
    ui.horizontal(|ui| {
        ui.add(egui::Label::new("Relative tolerance:"));
        let dv_tr = ui.add(
            egui::DragValue::new(&mut config.tol_rel)
                .min_decimals(BoundaryValues::MIN_TOL_DECIMALS)
                .max_decimals(BoundaryValues::MAX_TOL_DECIMALS),
        );

        if dv_tr.changed() {
            config.tol_rel = config.tol_rel.max(0.0);
        }
    });
    ui.horizontal(|ui| {
        ui.add(egui::Label::new("Minimum step size:"));
        let dv_h_min = ui.add(
            egui::DragValue::new(&mut config.h_min)
                .min_decimals(BoundaryValues::MIN_H_DECIMALS)
                .max_decimals(BoundaryValues::MAX_H_DECIMALS),
        );

        if dv_h_min.changed() {
            config.h_min = config.h_min.max(min_h_min);
            config.h_max = config.h_max.max(config.h_min + min_h_min);
        }
    });
    ui.horizontal(|ui| {
        ui.add(egui::Label::new("Maximum step size:"));
        let dv_h_max = ui.add(
            egui::DragValue::new(&mut config.h_max)
                .min_decimals(BoundaryValues::MIN_H_DECIMALS)
                .max_decimals(BoundaryValues::MAX_H_DECIMALS),
        );

        if dv_h_max.changed() {
            config.h_max = config.h_max.max(min_h_max);
            config.h_min = config.h_min.min(config.h_max - min_h_min).max(min_h_min);
        }
    });
}

fn render_save_load_buttons(ui: &mut egui::Ui, file_dialog: &mut FileDialog) {
    ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
        if ui.button("Load").clicked() {
            file_dialog.set_user_data(DialogAction::Load);
            file_dialog.pick_file();
        }

        if ui.button("Save").clicked() {
            file_dialog.set_user_data(DialogAction::Save);
            file_dialog.save_file();
        }
    });
}

fn process_save_load(settings: &mut SettingsComponent) {
    if let Some(path) = settings.file_dialog.take_picked() {
        if let Some(action) = settings.file_dialog.user_data::<DialogAction>() {
            match action {
                DialogAction::Save => {
                    if let Ok(file) = std::fs::File::create(&path) {
                        let _ = serde_json::to_writer_pretty(file, &settings.config);
                    }
                }
                DialogAction::Load => {
                    if let Ok(file) = std::fs::File::open(&path) {
                        match serde_json::from_reader::<_, SettingsConfig>(file) {
                            Ok(new_config) => match new_config.validate() {
                                Ok(()) => {
                                    settings.config = new_config;
                                }
                                Err(validation_err) => {
                                    settings.error_message =
                                        Some(format!("Incorrect config:\n{:?}", validation_err));
                                }
                            },
                            Err(serde_err) => {
                                settings.error_message =
                                    Some(format!("Failed to read config:\n{:?}", serde_err));
                            }
                        }
                    }
                }
            }
        }
    }
}
