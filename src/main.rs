use eframe::egui;
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};
use std::{
    env, fs,
    path::{Path, PathBuf},
};

const APOLOGY_LETTER: &str = "# Pardon me fellow human. I failed. \n\nI couldn't read the file.";

fn load(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| APOLOGY_LETTER.to_string())
}

fn visuals(t: &fenth::Theme) -> egui::Visuals {
    let c = |role: &str| {
        let c = t[role];
        egui::Color32::from_rgb(c.r, c.g, c.b)
    };

    // myeah, picking a visual without if will be hard... unless...
    let base = [egui::Visuals::light, egui::Visuals::dark]
        [(t.variant() == fenth::Variant::Dark) as usize]();
    egui::Visuals {
        panel_fill: c("background"),
        override_text_color: Some(c("foreground")),
        hyperlink_color: c("link"),
        code_bg_color: c("surface"),
        ..base
    }
}

struct MdLife {
    cache: CommonMarkCache,
    path: PathBuf,
    text: String,
}

impl MdLife {
    fn new(path: PathBuf) -> Self {
        let text = load(&path);
        Self {
            cache: CommonMarkCache::default(),
            path,
            text,
        }
    }
}

impl eframe::App for MdLife {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Frame::new()
                .inner_margin(egui::Margin::same(32))
                .show(ui, |ui| {
                    CommonMarkViewer::new().show(ui, &mut self.cache, &self.text);
                });
        });
    }
}

fn main() -> eframe::Result<()> {
    let path = env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("sample.md"));

    eframe::run_native(
        "mdlife",
        eframe::NativeOptions::default(),
        Box::new(move |cc| {
            let theme = fenth::get("catppuccin-latte").unwrap();
            cc.egui_ctx.set_visuals(visuals(theme));
            Ok(Box::new(MdLife::new(path)))
        }),
    )
}
