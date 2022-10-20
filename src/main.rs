pub mod vendor;
use vendor::foundation::{http, Application};

fn main() {
    let app = Application::make("halo");
    let kernel = http::Kernel::make(app);
    println!("{:?}", kernel.get_app());
}
