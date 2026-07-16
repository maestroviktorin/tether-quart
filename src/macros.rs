macro_rules! plot {
    (
        ui = $ui:expr,
        history = $history:expr,
        name = $name:expr,
        id_source = $id_source:expr,
        height = $height:expr,
        x = $($abscissa:ident).+,
        y = $($ordinate:ident).+
    ) => {{
        use egui_plot::{Line, Plot, PlotPoints};

        $ui.heading($name);
        let points: PlotPoints = $history
            .iter()
            .map(|u| [(u $(.$abscissa)+) as f64, (u $(.$ordinate)+) as f64])
            .collect();

        let line = Line::new($name, points);

        Plot::new($id_source)
            .height($height)
            .show($ui, |plot_ui| plot_ui.line(line));
    }};
}

pub(crate) use plot;