use anyhow::{Result, anyhow};
use egui_file_dialog::FileDialog;
use serde::{Deserialize, Serialize};

struct BoundaryValues;

impl BoundaryValues {
    pub const MIN_M: f64 = 10.0;
    pub const MAX_M: f64 = 100.0;
    pub const MIN_F0: f64 = 0.0;
    pub const MAX_F0: f64 = 100.0;
    pub const MIN_PHI: f64 = 0.0;
    pub const MAX_PHI: f64 = 359.0;
    pub const MIN_L_K: f64 = 10.0;
    pub const MAX_L_K: f64 = 1500.0;
    pub const MIN_K_L: f64 = 0.0;
    pub const MAX_K_L: f64 = 10.0;
    pub const MIN_K_V: f64 = 0.0;
    pub const MAX_K_V: f64 = 10.0;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum DialogAction {
    Save,
    Load,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsConfig {
    pub m: f64,
    pub f0: f64,
    pub phi: f64,
    pub l_k: f64,
    pub k_l: f64,
    pub k_v: f64,
    pub t1: f64,
    pub t2: f64,
    pub t3: f64,
    pub init_v: f64,
    pub init_l: f64,
    pub init_omega: f64,
    pub init_theta: f64,
    pub tol_abs: f64,
    pub tol_rel: f64,
    pub h_min: f64,
    pub h_max: f64,
}

impl Default for SettingsConfig {
    fn default() -> Self {
        Self {
            m: 25.0,
            f0: 5.0,
            phi: 0.0,
            l_k: 500.0,
            k_l: 1.5,
            k_v: 3.2,
            t1: 30.0,
            t2: 60.0,
            t3: 90.0,
            init_v: 0.1,
            init_l: 5.0,
            init_omega: 0.02,
            init_theta: 0.0,
            tol_abs: 1e-6,
            tol_rel: 1e-6,
            h_min: 1e-4,
            h_max: 1.0,
        }
    }
}

impl SettingsConfig {
    pub fn validate(&self) -> Result<()> {
        if self.m < BoundaryValues::MIN_M || self.m > BoundaryValues::MAX_M {
            return Err(anyhow!(
                "Condition failed: {} <= m <= {}\n(received m = {})",
                BoundaryValues::MIN_M,
                BoundaryValues::MAX_M,
                self.m
            ));
        }
        if self.f0 < BoundaryValues::MIN_F0 || self.f0 > BoundaryValues::MAX_F0 {
            return Err(anyhow!(
                "Condition failed: {} <= f0 <= {}\n(received f0 = {})",
                BoundaryValues::MIN_F0,
                BoundaryValues::MAX_F0,
                self.f0
            ));
        }
        if self.phi < BoundaryValues::MIN_PHI || self.phi > BoundaryValues::MAX_PHI {
            return Err(anyhow!(
                "Condition failed: {} <= phi <= {}\n(received phi = {})",
                BoundaryValues::MIN_PHI,
                BoundaryValues::MAX_PHI,
                self.phi
            ));
        }
        if self.l_k < BoundaryValues::MIN_L_K || self.l_k > BoundaryValues::MAX_L_K {
            return Err(anyhow!(
                "Condition failed: {} <= l_k <= {}\n(received l_k = {})",
                BoundaryValues::MIN_L_K,
                BoundaryValues::MAX_L_K,
                self.l_k
            ));
        }
        if self.k_l < BoundaryValues::MIN_K_L || self.k_l > BoundaryValues::MAX_K_L {
            return Err(anyhow!(
                "Condition failed: {} <= k_l <= {}\n(received k_l = {})",
                BoundaryValues::MIN_K_L,
                BoundaryValues::MAX_K_L,
                self.k_l
            ));
        }
        if self.k_v < BoundaryValues::MIN_K_V || self.k_v > BoundaryValues::MAX_K_V {
            return Err(anyhow!(
                "Condition failed: {} <= k_v <= {}\n(received k_v = {})",
                BoundaryValues::MIN_K_V,
                BoundaryValues::MAX_K_V,
                self.k_v
            ));
        }
        if self.t1 < 0.0 {
            return Err(anyhow!("Parameter t1 cannot be negative."));
        }
        if self.t2 < self.t1 + 1.0 {
            return Err(anyhow!(
                "Condition failed: t1 < t2. Difference must be at least 1.0\n(received t1 = {}, t2 = {}, t2 >= {} required)",
                self.t1,
                self.t2,
                self.t1 + 1.0
            ));
        }
        if self.t3 < self.t2 + 1.0 {
            return Err(anyhow!(
                "Condition failed: t2 < t3. Difference must be at least 1.0\n(received t2 = {}, t3 = {}, t3 >= {} required)",
                self.t2,
                self.t3,
                self.t2 + 1.0
            ));
        }
        if self.init_v < 0.0 {
            return Err(anyhow!("Parameter init_v cannot be negative."));
        }
        if self.init_l < 0.0 {
            return Err(anyhow!("Parameter init_l cannot be negative."));
        }
        if self.init_omega < 0.0 {
            return Err(anyhow!("Parameter init_omega cannot be negative."));
        }
        if self.init_theta < 0.0 {
            return Err(anyhow!("Parameter init_theta cannot be negative."));
        }
        if self.tol_abs < 0.0 {
            return Err(anyhow!("Parameter tol_abs cannot be negative."));
        }
        if self.tol_rel < 0.0 {
            return Err(anyhow!("Parameter tol_rel cannot be negative."));
        }
        if self.h_min < 0.0 {
            return Err(anyhow!("Parameter h_min cannot be negative."));
        }
        if self.h_max < 0.0 {
            return Err(anyhow!("Parameter h_max cannot be negative."));
        }
        anyhow::Ok(())
    }
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
            file_dialog: FileDialog::new(),
            error_message: None,
        }
    }
}

pub fn render(ui: &mut egui::Ui, frame: &mut eframe::Frame, settings: &mut SettingsComponent) {
    egui::CollapsingHeader::new("TSS Settings")
        .default_open(true)
        .show(ui, |ui| {
            ui.add(
                egui::Slider::new(
                    &mut settings.config.m,
                    BoundaryValues::MIN_M..=BoundaryValues::MAX_M,
                )
                .suffix(" kg")
                .text("Mass, m"),
            );
            ui.add(
                egui::Slider::new(
                    &mut settings.config.f0,
                    BoundaryValues::MIN_F0..=BoundaryValues::MAX_F0,
                )
                .suffix(" N")
                .text("Thrust force, f0"),
            );
            ui.add(
                egui::Slider::new(
                    &mut settings.config.phi,
                    BoundaryValues::MIN_PHI..=BoundaryValues::MAX_PHI,
                )
                .suffix("°")
                .text("Force direction angle, phi"),
            );

            time_drag_values(ui, settings);
        });
    ui.separator();

    egui::CollapsingHeader::new("Control Law Settings")
        .default_open(true)
        .show(ui, |ui| {
            ui.add(
                egui::Slider::new(
                    &mut settings.config.l_k,
                    BoundaryValues::MIN_L_K..=BoundaryValues::MAX_L_K,
                )
                .suffix(" m")
                .text("Target tethers length, l_k"),
            );
            ui.add(
                egui::Slider::new(
                    &mut settings.config.k_l,
                    BoundaryValues::MIN_K_L..=BoundaryValues::MAX_K_L,
                )
                .text("Length regulation ratio, k_l"),
            );
            ui.add(
                egui::Slider::new(
                    &mut settings.config.k_v,
                    BoundaryValues::MIN_K_V..=BoundaryValues::MAX_K_V,
                )
                .text("Velocity regulation ratio, k_v"),
            );
        });
    ui.separator();

    egui::CollapsingHeader::new("Initial State Settings")
        .default_open(true)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.add(egui::Label::new("Length rate of change, v:"));
                ui.add(egui::DragValue::new(&mut settings.config.init_v).suffix(" m/s"));
            });
            ui.horizontal(|ui| {
                ui.add(egui::Label::new("Tethers length, l:"));
                ui.add(egui::DragValue::new(&mut settings.config.init_l).suffix(" m"));
            });
            ui.horizontal(|ui| {
                ui.add(egui::Label::new("Angle velocity, omega:"));
                ui.add(egui::DragValue::new(&mut settings.config.init_omega).suffix(" rad/s"));
            });
            ui.horizontal(|ui| {
                ui.add(egui::Label::new("Orientation angle, theta:"));
                ui.add(egui::DragValue::new(&mut settings.config.init_theta).suffix(" rad"));
            });
        });
    ui.separator();

    egui::CollapsingHeader::new("RKF45 Settings").show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.add(egui::Label::new("Absolute tolerance:"));
            ui.add(egui::DragValue::new(&mut settings.config.tol_abs).min_decimals(6));
        });
        ui.horizontal(|ui| {
            ui.add(egui::Label::new("Relative tolerance:"));
            ui.add(egui::DragValue::new(&mut settings.config.tol_rel).min_decimals(6));
        });
        ui.horizontal(|ui| {
            ui.add(egui::Label::new("Minimum step size:"));
            ui.add(egui::DragValue::new(&mut settings.config.h_min).min_decimals(4));
        });
        ui.horizontal(|ui| {
            ui.add(egui::Label::new("Maximum step size:"));
            ui.add(egui::DragValue::new(&mut settings.config.h_max).min_decimals(4));
        });
    });

    ui.separator();

    egui::CollapsingHeader::new("Save & Load Configuration")
        .default_open(true)
        .show(ui, |ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                if ui.button("Load").clicked() {
                    settings.file_dialog.set_user_data(DialogAction::Load);
                    settings.file_dialog.pick_file();
                }

                if ui.button("Save").clicked() {
                    settings.file_dialog.set_user_data(DialogAction::Save);
                    settings.file_dialog.save_file();
                }
            });
        });

    settings.file_dialog.update(ui.ctx());

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

    if let Some(err_msg) = &settings.error_message {
        let mut open = true;

        egui::Window::new("Load Config Error")
            .open(&mut open)
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ui.ctx(), |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(5.0);
                    ui.label(err_msg);
                    ui.add_space(10.0);
                });
            });

        if !open {
            settings.error_message = None;
        }
    }
}

fn time_drag_values(ui: &mut egui::Ui, settings: &mut SettingsComponent) {
    ui.horizontal(|ui| {
        ui.add(egui::Label::new("t1:"));
        let dv1 = ui.add(egui::DragValue::new(&mut settings.config.t1));

        ui.add(egui::Label::new("t2:"));
        let dv2 = ui.add(egui::DragValue::new(&mut settings.config.t2));

        ui.add(egui::Label::new("t3:"));
        let dv3 = ui.add(egui::DragValue::new(&mut settings.config.t3));

        if dv1.changed() {
            settings.config.t1 = settings.config.t1.max(0.0);
            settings.config.t2 = settings.config.t2.max(settings.config.t1 + 1.0);
            settings.config.t3 = settings.config.t3.max(settings.config.t2 + 1.0);
        }

        if dv2.changed() {
            settings.config.t2 = settings.config.t2.max(1.0);
            settings.config.t1 = settings.config.t1.min(settings.config.t2 - 1.0);
            settings.config.t3 = settings.config.t3.max(settings.config.t2 + 1.0);
        }

        if dv3.changed() {
            settings.config.t3 = settings.config.t3.max(2.0);
            settings.config.t2 = settings.config.t2.min(settings.config.t3 - 1.0);
            settings.config.t1 = settings.config.t1.min(settings.config.t2 - 1.0);
        }
    });
}
