use std::collections::HashMap;

use eframe::egui;
use views::DevTodos;

mod views;

use crate::views::ItemSelectionView;

trait View{
  fn render(&mut self,ui: &mut egui::Ui);
}


struct AppRoot{
  current_view: Box<dyn View>,
  selected_button:usize,
}

impl Default for AppRoot{
  fn default() -> Self {
    Self { current_view: Box::new(ItemSelectionView::new()), selected_button: 0 }
  }
}

impl eframe::App for AppRoot{
  fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
    egui::TopBottomPanel::top("menu").show(ctx,|ui|{
      let mut table:Vec<(String,fn()->Box<dyn View>)> = vec![];
      table.push((
        "Item Selection".into(),
        ||{Box::new(ItemSelectionView::new())}
      ));
      table.push((
        "Dev Todos".into(),
        ||{Box::new(DevTodos::new())}
      ));
      ui.horizontal(|ui|{
        for i in 0..table.len(){
          if self.selected_button == i{
            ui.add_enabled(false, egui::Button::new(&table[i].0));
          }else{
            if ui.button(&table[i].0).clicked(){
              self.current_view = table[i].1();
              self.selected_button = i;
            }
          }
        }
      });
    });
    egui::CentralPanel::default().show(ctx,|ui|{
      self.current_view.render(ui);
    });
  }
}


fn main() -> Result<(),eframe::Error> {
  let native_options = eframe::NativeOptions{
    viewport: egui::ViewportBuilder::default().with_inner_size([400.,400.]),
    ..Default::default()
  };
  eframe::run_native("Foundry Manager", native_options, 
   Box::new(|_|{Box::<AppRoot>::default()}))
}
