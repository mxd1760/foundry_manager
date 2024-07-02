use eframe::egui;

use crate::View;

pub struct DevTodos{
}

impl View for DevTodos{
  fn render(&mut self, ui: &mut egui::Ui){
    ui.label("Dev Todos");
  }
}

impl DevTodos{
  pub fn new() -> Self{
    Self { }
  }
}