use std::sync::mpsc::channel;

mod gui;
mod model;
mod rkf45;
mod simulation_worker;

fn main() -> eframe::Result<()> {
    let (tx_cmd, rx_cmd) = channel();
    let (tx_update, rx_update) = channel();

    simulation_worker::spawn_worker(rx_cmd, tx_update);

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Tether Quart",
        native_options,
        Box::new(|cc| Ok(Box::new(gui::App::new(cc, tx_cmd, rx_update)))),
    )
}
