#[macro_use]
extern crate vulkano;
extern crate vulkano_shaders;
extern crate vulkano_win;
extern crate winit;

mod app;
use app::App;

mod queue_family;

fn main() {
    let app = App::new();
    app.run();
}
