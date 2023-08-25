use crate::{egui, MyApp};

use super::{EguiApp, View};

#[derive(Default)]
pub struct UserLogin {
    username: String,
    password: String,
}
#[derive(Default)]
pub struct UserRegister {
    username: String,
    email: String,
    password: String,
}

#[derive(Default)]
pub struct LeftView {
    user_login: UserLogin,
    user_register: UserRegister,
}
#[derive(Default)]
pub struct LeftViewImpl<'a> {
    // Parent APP
    app: Option<&'a mut MyApp>,
    width: f32,
}

impl<'a> EguiApp for LeftViewImpl<'a> {
    fn name(&self) -> &'static str {
        "LeftView"
    }

    fn show(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left(self.name()).show(ctx, |ui| self.ui(ui));
    }
}
impl<'a> View for LeftViewImpl<'a> {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.set_min_width(self.width);
        // load login ui.
        self.login_ui(ui);

        // load register ui.
        self.register_ui();
    }
}
impl<'a> LeftViewImpl<'a> {
    pub fn width(&mut self, width: f32) -> &mut Self {
        self.width = width;
        self
    }
    pub fn set_app(&mut self, app: &'a mut MyApp) -> &mut Self {
        self.app = Some(app);
        self
    }
    fn app_leftview_ref(&self) -> &LeftView {
        &self.app.as_ref().unwrap().left_side
    }
    fn app_leftview_mut(&mut self) -> &mut LeftView {
        &mut self.app.as_mut().unwrap().left_side
    }
    fn app_userlogin(&mut self) -> &mut UserLogin {
        &mut self.app_leftview_mut().user_login
    }
    pub fn login_ui(&mut self, ui: &mut egui::Ui) {
        ui.label("username: ");
        ui.text_edit_singleline(&mut self.app_userlogin().username);
        ui.label(format!("{}", self.app_userlogin().username));

        ui.label("password: ");
        ui.text_edit_singleline(&mut self.app_userlogin().password);
        ui.label(format!("{}", self.app_userlogin().password));
    }
    // todo
    pub fn register_ui(&mut self) {}
}
