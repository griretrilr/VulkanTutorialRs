mod app;
mod log;
mod vulkano_ext;

extern crate num;
extern crate vulkano;
extern crate vulkano_shaders;
extern crate vulkano_win;
extern crate winit;

use app::App;

fn main() {
    let app = App::new();
    app.run();
}
