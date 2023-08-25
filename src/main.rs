#![allow(unused)]
use eframe::{egui, run_native, App, NativeOptions};

mod components;
use components::{
    left_side::{LeftView, LeftViewImpl},
    EguiApp,
};
#[derive(Default)]
pub struct MyApp {
    left_side: LeftView,
}
impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        LeftViewImpl::default().width(280.).set_app(self).show(ctx);
    }
}

fn main() {
    let mut win_option = NativeOptions::default();
    win_option.min_window_size = Some(egui::vec2(1600., 809.));

    run_native(
        "egui_app",
        win_option,
        Box::new(|_cc| {
            _cc.egui_ctx.set_visuals(egui::Visuals::dark());
            Box::new(MyApp::default())
        }),
    );
}
