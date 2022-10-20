use crate::vendor::foundation::Application;

#[derive(Debug)]
pub struct Kernel {
    app: Application,
}

impl Kernel {
    pub fn make(app: Application)-> Self{
        Kernel {app}
    }
    pub fn get_app(&self)->&Application{
        &self.app
    }
}
