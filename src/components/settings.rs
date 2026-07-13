pub struct SettingsComponent {
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

impl Default for SettingsComponent {
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

pub fn render(ui: &mut egui::Ui, frame: &mut eframe::Frame, settings: &mut SettingsComponent) {
    egui::CollapsingHeader::new("TSS Settings")
        .default_open(true)
        .show(ui, |ui| {
            ui.add(
                egui::Slider::new(&mut settings.m, 10.0..=100.0)
                    .suffix(" kg")
                    .text("Mass, m"),
            );
            ui.add(
                egui::Slider::new(&mut settings.f0, 0.0..=100.00)
                    .suffix(" N")
                    .text("Thrust force, f0"),
            );
            ui.add(
                egui::Slider::new(&mut settings.phi, 0.0..=359.00)
                    .suffix("°")
                    .text("Force direction angle, phi"),
            );

            // TODO: Implement fool resistance.
            ui.horizontal(|ui| {
                ui.add(egui::Label::new("t1:"));
                ui.add(egui::DragValue::new(&mut settings.t1));
                ui.add(egui::Label::new("t2:"));
                ui.add(egui::DragValue::new(&mut settings.t2));
                ui.add(egui::Label::new("t3:"));
                ui.add(egui::DragValue::new(&mut settings.t3));
            });
        });
    ui.separator();

    egui::CollapsingHeader::new("Control Law Settings")
        .default_open(true)
        .show(ui, |ui| {
            ui.add(
                egui::Slider::new(&mut settings.l_k, 10.0..=1500.0)
                    .suffix(" m")
                    .text("Target tethers length, l_k"),
            );
            ui.add(
                egui::Slider::new(&mut settings.k_l, 0.0..=10.0)
                    .text("Length regulation ratio, k_l"),
            );
            ui.add(
                egui::Slider::new(&mut settings.k_v, 0.0..=10.0)
                    .text("Velocity regulation ratio, k_v"),
            );
        });
    ui.separator();

    egui::CollapsingHeader::new("Initial State Settings")
        .default_open(true)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.add(egui::Label::new("Length rate of change, v:"));
                ui.add(egui::DragValue::new(&mut settings.init_v).suffix(" m/s"));
            });
            ui.horizontal(|ui| {
                ui.add(egui::Label::new("Tethers length, l:"));
                ui.add(egui::DragValue::new(&mut settings.init_l).suffix(" m"));
            });
            ui.horizontal(|ui| {
                ui.add(egui::Label::new("Angle velocity, omega:"));
                ui.add(egui::DragValue::new(&mut settings.init_omega).suffix(" rad/s"));
            });
            ui.horizontal(|ui| {
                ui.add(egui::Label::new("Orientation angle, theta:"));
                ui.add(egui::DragValue::new(&mut settings.init_theta).suffix(" rad"));
            });
        });
    ui.separator();

    egui::CollapsingHeader::new("RKF45 Settings").show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.add(egui::Label::new("Absolute tolerance:"));
            ui.add(egui::DragValue::new(&mut settings.tol_abs).min_decimals(6));
        });
        ui.horizontal(|ui| {
            ui.add(egui::Label::new("Relative tolerance:"));
            ui.add(egui::DragValue::new(&mut settings.tol_rel).min_decimals(6));
        });
        ui.horizontal(|ui| {
            ui.add(egui::Label::new("Minimum step size:"));
            ui.add(egui::DragValue::new(&mut settings.h_min).min_decimals(4));
        });
        ui.horizontal(|ui| {
            ui.add(egui::Label::new("Maximum step size:"));
            ui.add(egui::DragValue::new(&mut settings.h_max).min_decimals(4));
        });
    });
}
