pub mod http;

#[derive(Debug)]
pub struct Application {
    base_path: String,
}

impl Application {
    pub fn make(base_path: &str) -> Self {
        Application {base_path: base_path.to_string()}
    }
    pub fn get_base_path(&self)->String{
        self.base_path.to_string()
    }
    // add code here
}
