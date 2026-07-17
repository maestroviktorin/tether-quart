use crate::{app::SimulationUpdate, components::settings::SettingsComponent};

pub fn render(
    ui: &mut egui::Ui,
    frame: &mut eframe::Frame,
    history: &[SimulationUpdate],
    settings: &SettingsComponent,
) {
    ui.heading("TSS Scheme");
    let last_state = history.last().map(|u| u.state);

    let size = egui::vec2(ui.available_width(), ui.available_height());
    let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());
    let painter = ui.painter_at(rect);
    let center = rect.center();

    if let Some(state) = last_state {
        let r = state.l / f64::sqrt(2.0);
        let scale = 0.4 * f64::from(rect.width()) / settings.config.l_k;
        let angles = [0.0, 1.0, 2.0, 3.0].map(|i| state.theta + i * std::f64::consts::FRAC_PI_2);
        let points: Vec<egui::Pos2> = angles
            .iter()
            .map(|&a| {
                center + egui::vec2((r * a.cos() * scale) as f32, (r * a.sin() * scale) as f32)
            })
            .collect();
        let tension = history.last().map(|u| u.tension).unwrap_or(0.0);
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
}
