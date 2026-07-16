use std::sync::mpsc::channel;

mod app;
mod components;
mod macros;
mod model;
mod rkf45;
mod simulation_worker;

fn main() -> eframe::Result<()> {
    let (tx_cmd, rx_cmd) = channel();
    let (tx_update, rx_update) = channel();

    simulation_worker::spawn_worker(rx_cmd, tx_update);

    let icon = eframe::icon_data::from_png_bytes(include_bytes!("assets/png/icon.png"))
        .expect("Failed to load icon.");
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_icon(icon)
            .with_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Tether Quart",
        native_options,
        Box::new(|cc| Ok(Box::new(app::App::new(cc, tx_cmd, rx_update)))),
    )
}
