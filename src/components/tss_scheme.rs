use egui::Scene;

use crate::{app::SimulationUpdate, components::settings::SettingsComponent};

pub fn render(
    ui: &mut egui::Ui,
    frame: &mut eframe::Frame,
    history: &[SimulationUpdate],
    settings: &SettingsComponent,
) {
    ui.heading("TSS Scheme");
    let size = ui.available_size();
    let scene_rect_id = ui.id().with("tss_scene_rect");

    let mut scene_rect = ui.data_mut(|d| {
        *d.get_temp_mut_or_insert_with(scene_rect_id, || {
            egui::Rect::from_center_size(egui::Pos2::ZERO, size)
        })
    });

    /*
        Add `.scroll_zooms(true)` if https://github.com/emilk/egui/pull/7891 gets approved someday.

        This method brings in better UX: user can scale a `Scene` without holding `Ctrl`.
        So far, `Scene`-scaling can only be done when holding `Ctrl`.
    */
    Scene::new()
        .zoom_range(0.1..=2.0)
        .sense(egui::Sense::hover())
        .max_inner_size(size)
        .show(ui, &mut scene_rect, |ui| {
            let painter = ui.painter();
            let last_state = history.last().map(|u| u.state);

            if let Some(state) = last_state {
                let r = state.l / f64::sqrt(2.0);

                let base_virtual_width = size.x as f64;
                let scale = 0.4 * base_virtual_width / settings.config.l_k;

                let angles =
                    [0.0, 1.0, 2.0, 3.0].map(|i| state.theta + i * std::f64::consts::FRAC_PI_2);
                let points: Vec<egui::Pos2> = angles
                    .iter()
                    .map(|&a| {
                        egui::Pos2::new((r * a.cos() * scale) as f32, (r * a.sin() * scale) as f32)
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
        });

    let centered_scene_rect = egui::Rect::from_center_size(egui::Pos2::ZERO, scene_rect.size());

    ui.data_mut(|d| d.insert_temp(scene_rect_id, centered_scene_rect));
}
