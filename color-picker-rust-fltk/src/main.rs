use fltk::{app, prelude::*, window::Window};

fn main() {
    let app = app::App::default();
    let mut wind = Window::new(0, 0, 400, 400, "Color picker").center_screen();
    wind.end();
    wind.show();
    app.run().unwrap();
}
