use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::Arc};

use anyhow::{Result, anyhow};
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};

pub struct AboutComponent {
    pub common_mark_cache: CommonMarkCache,
    pub math_cache: Rc<RefCell<HashMap<String, Arc<[u8]>>>>,
}

impl Default for AboutComponent {
    fn default() -> Self {
        Self {
            common_mark_cache: Default::default(),
            math_cache: Rc::new(RefCell::new(HashMap::new()))
        }
    }
}

const ABOUT: &str = include_str!("../assets/markdown/about.md");

pub fn render(
    ui: &mut egui::Ui,
    frame: &mut eframe::Frame,
    // `AboutComponent`'s fields passed separately.
    common_mark_cache: &mut CommonMarkCache,
    math_cache: Rc<RefCell<HashMap<String, Arc<[u8]>>>>
) {
    CommonMarkViewer::new()
        .render_math_fn(Some(&move |ui, math_str, inline| {
            let math_string = math_str.to_string();

            let id = egui::Id::from(math_string.clone());
            let uri = format!("bytes://math_{}.svg", id.value());

            let svg_bytes = {
                let mut cache = math_cache.borrow_mut();

                cache.entry(math_string.clone())
                    .or_insert_with(move || {
                        compile_latex_to_svg(&math_string, inline)
                        .unwrap_or_else(|err| {
                            Arc::from(format!(
                                r#"<svg xmlns="http://www.w3.org/2000/svg"><text y="15" fill="red">Error: {:?}</text></svg>"#,
                                err
                            ).into_bytes())
                        })
                    })
                    .clone()
                    
            };

            ui.add(
                egui::Image::new(egui::ImageSource::Bytes {
                    uri: uri.into(),
                    bytes: egui::load::Bytes::Shared(svg_bytes),
                })
                .fit_to_original_size(0.33),
            );
        }))
        .show(ui, common_mark_cache, ABOUT);
}

// TODO: Use `anyhow`.
fn compile_latex_to_svg(latex: &str, inline: bool) -> Result<Arc<[u8]>> {
    let parsed = ratex_parser::parse(latex).map_err(|e| anyhow!("LaTeX parse error: {:?}", e))?;

    let layout_options = ratex_layout::LayoutOptions::default();
    let layout_box = ratex_layout::layout(&parsed, &layout_options);
    let display_list = ratex_layout::to_display_list(&layout_box);

    let mut svg_options = ratex_svg::SvgOptions::default();
    svg_options.embed_glyphs = true;

    let svg_string = ratex_svg::render_to_svg(&display_list, &svg_options);

    Ok(Arc::from(svg_string.into_bytes()))
}