#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod app;
mod models;

fn main() -> eframe::Result {
    env_logger::init();
    app::run_app()
}
