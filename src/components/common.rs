pub fn process_error_window(ui: &mut egui::Ui, title: &str, error_message: &mut Option<String>) {
    if let Some(err_msg) = error_message {
        let mut open = true;

        egui::Window::new(title)
            .open(&mut open)
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ui.ctx(), |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(5.0);
                    ui.label(err_msg.to_owned());
                    ui.add_space(10.0);
                });
            });

        if !open {
            *error_message = None;
        }
    }
}
