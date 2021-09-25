use std::collections::HashMap;

pub struct SystemState {
  pub exit: bool,
  pub departments: HashMap<String, Vec<String>>,
}

impl SystemState {
  pub fn build() -> SystemState {
    SystemState {
      exit: false,
      departments: HashMap::new(),
    }
  }
}
