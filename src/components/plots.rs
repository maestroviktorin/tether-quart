use crate::{app::SimulationUpdate, macros::plot};

pub fn render(ui: &mut egui::Ui, frame: &mut eframe::Frame, history: &[SimulationUpdate]) {
    plot!(
        ui = ui,
        history = history,
        name = "l(t)",
        id_source = "l_t_plot",
        height = 200.0,
        x = t,
        y = state.l
    );
}
