extern crate mu_list;
use mu_list::{ListApp, run_gui};
use eframe::NativeOptions;
use egui::Vec2;

#[tokio::main]
async fn main() {
    let options = NativeOptions {
        initial_window_size: Some(Vec2::new(1080.0, 700.0)),
        ..Default::default()
    };

    let app = ListApp::new();  // Now calling the public new() method

    eframe::run_native(
        "MuList",
        options,
        Box::new(|_cc| Box::new(MyApp { app })),
    );
}

struct MyApp {
    app: ListApp,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            run_gui(ui, &mut self.app);
        });
    }
}