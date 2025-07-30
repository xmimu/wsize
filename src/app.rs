use std::path::PathBuf;

use eframe::egui;
use egui::{
    epaint::text::{FontInsert, InsertFontFamily},
    ComboBox,
};
use rfd::FileDialog;

use crate::models::{
    filter_type::{LanguageFilter, MatchBank, NameFilter},
    match_data::MatchData,
};

pub fn run_app() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 400.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Wwise分包预览",
        options,
        Box::new(|cc| Ok(Box::new(LibApp::new(cc)))),
    )
}

fn add_font(ctx: &egui::Context) {
    ctx.add_font(FontInsert::new(
        "my_font",
        egui::FontData::from_static(include_bytes!("./assets/SIMHEI.TTF")),
        vec![
            InsertFontFamily {
                family: egui::FontFamily::Proportional,
                priority: egui::epaint::text::FontPriority::Highest,
            },
            InsertFontFamily {
                family: egui::FontFamily::Monospace,
                priority: egui::epaint::text::FontPriority::Lowest,
            },
        ],
    ));
}

pub struct LibApp {
    json_path: PathBuf,
    json_path_str: String,
    filters: Vec<Box<dyn MatchBank>>,
    results: Vec<MatchData>,
    log_text: String,
}

impl Default for LibApp {
    fn default() -> Self {
        Self {
            json_path: PathBuf::new(),
            json_path_str: String::new(),
            filters: vec![],
            results: vec![],
            log_text: String::new(),
        }
    }
}

impl LibApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        add_font(&cc.egui_ctx);
        LibApp::default()
    }

    fn log(&mut self, msg: &str) {
        self.log_text.push_str(msg);
        self.log_text.push('\n');
    }
}

impl LibApp {
    fn ui_path_picker(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Bank目录设置：");

            if ui.text_edit_singleline(&mut self.json_path_str).changed() {
                self.update_path_from_input();
            }

            if ui.button("...").clicked() {
                if let Some(p) = FileDialog::new().set_title("选择 Bank 目录").pick_file() {
                    self.set_json_path(p);
                }
            }
        });
    }

    fn update_path_from_input(&mut self) {
        let p = PathBuf::from(&self.json_path_str);
        if !Self::is_valid_json_path(&p) {
            self.log("请选择 SoundbanksInfo.json 文件！");
            return;
        }
        self.set_json_path(p);
    }

    fn set_json_path(&mut self, p: PathBuf) {
        self.json_path = p.clone();
        self.json_path_str = p.to_string_lossy().to_string();
        self.log(&format!("设置目录：{}", self.json_path.display()));
    }

    fn is_valid_json_path(path: &PathBuf) -> bool {
        path.is_file()
            && path.file_name().map_or(false, |f| {
                f.to_string_lossy().contains("SoundbanksInfo.json")
            })
    }
}

impl LibApp {
    fn ui_filters(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Filters");
            if ui.button("添加 Name 过滤器").clicked() {
                self.filters.push(Box::new(NameFilter::new()));
            }
            if ui.button("添加 Language 过滤器").clicked() {
                self.filters.push(Box::new(LanguageFilter::new()));
            }
            if ui.button("全部移除").clicked() {}
        });

        for f in self.filters.iter() {
            self.draw_filter_ui(ui, f);
        }
    }

    fn draw_filter_ui(&self, ui: &mut egui::Ui, filter: &Box<dyn MatchBank>) {
        if let Some(f) = filter.as_any().downcast_ref::<NameFilter>() {
            self.ui_name_filter(ui, f);
        } else if let Some(f) = filter.as_any().downcast_ref::<LanguageFilter>() {
            self.ui_language_filter(ui, f);
        }
    }

    fn ui_name_filter(&self, ui: &mut egui::Ui, f: &NameFilter) {
        ui.label("Namefilter");
    }

    fn ui_language_filter(&self, ui: &mut egui::Ui, f: &LanguageFilter) {
        for i in f.languages.iter() {
            ui.label(i);
        }
    }
}

impl eframe::App for LibApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // 路径设置
            self.ui_path_picker(ui);
            // 过滤器
            self.ui_filters(ui);

            // 结果展示

            // 日志
            if ui.button("清空").clicked() {
                self.log_text.clear();
            }
            ui.text_edit_multiline(&mut self.log_text);
        });
    }
}
