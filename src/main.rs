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
            CommonMarkViewer::new().show(ui, &mut self.cache, &self.text);
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
        Box::new(move |_cc| Ok(Box::new(MdLife::new(path)))),
    )
}
