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
    let stroke = |role: &str| egui::Stroke::new(1.0, c(role));
    let widget = |old: egui::style::WidgetVisuals, fill: &str, fg: &str, border: &str| {
        egui::style::WidgetVisuals {
            bg_fill: c(fill),
            weak_bg_fill: c(fill),
            bg_stroke: stroke(border),
            fg_stroke: stroke(fg),
            ..old
        }
    };

    // myeah, picking a visual without if/else will be hard... unless...
    let base = [egui::Visuals::light, egui::Visuals::dark]
        [(t.variant() == fenth::Variant::Dark) as usize]();
    let w = base.widgets.clone();

    // uuuugh
    egui::Visuals {
        panel_fill: c("background"),
        window_fill: c("surface"),
        window_stroke: stroke("border"),
        extreme_bg_color: c("background.alt"),
        faint_bg_color: c("surface"),
        code_bg_color: c("surface"),
        hyperlink_color: c("link"),
        warn_fg_color: c("warning"),
        error_fg_color: c("error"),
        selection: egui::style::Selection {
            bg_fill: c("selection"),
            stroke: stroke("accent.primary"),
        },
        widgets: egui::style::Widgets {
            noninteractive: widget(w.noninteractive, "background", "foreground", "border.muted"),
            inactive: widget(w.inactive, "surface", "foreground", "border.muted"),
            hovered: widget(w.hovered, "highlight", "foreground", "border"),
            active: widget(w.active, "active", "accent.primary", "border.focus"),
            open: widget(w.open, "surface.alt", "foreground", "border"),
        },
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
        egui::CentralPanel::default().show(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                egui::Frame::new()
                    .inner_margin(egui::Margin::same(32))
                    .show(ui, |ui| {
                        CommonMarkViewer::new().show(ui, &mut self.cache, &self.text);
                    });
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
            let theme = fenth::get("catppuccin-mocha").unwrap();
            cc.egui_ctx.set_visuals(visuals(theme));
            Ok(Box::new(MdLife::new(path)))
        }),
    )
}
