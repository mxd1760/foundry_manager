use std::collections::HashMap;

use eframe::egui::{self, ahash::HashSet};

use crate::View;

pub struct ItemSelectionView{
  list: HashMap<String,(i32,i32)>,
}

impl View for ItemSelectionView{
  fn render(&mut self, ui: &mut egui::Ui){
    ui.label(format!("Total cost {}",self.get_sum()));
    for (key,value) in &mut self.list{
      ui.horizontal(|ui|{
        ui.label(format!("{} - {}KW",key,value.0));
        ui.add(egui::DragValue::new(&mut value.1))
      });
    }
  }
}

impl ItemSelectionView{
  pub fn new() -> Self{
    Self { 
      list: make_empty_list(),
    }
  }
  fn get_sum(&self) -> i32{
    let mut sum = 0;
    for (_,value) in &self.list{
      sum += value.0*value.1;
    }
    sum
  }
}

fn make_empty_list() -> HashMap<String,(i32,i32)>{
  let mut list = HashMap::<String,(i32,i32)>::new();
  list.insert(
    "Advanced Smelter".into(),
    (150,0)
  );
  list.insert(
    "Assembler I".into(),
    (50,0)
  );
  list.insert(
    "Drone Miner I".into(),
    (200,0)
  );
  list.insert(
    "Loader".into(),
    (2,0)
  );
  list.insert(
    "Crusher".into(),
    (75,0)
  );
  list
}