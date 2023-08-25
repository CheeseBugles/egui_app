pub mod left_side;
use crate::egui;

pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui);
}

pub trait EguiApp {
    fn name(&self) -> &'static str;
    fn show(&mut self, ctx: &egui::Context);
}
