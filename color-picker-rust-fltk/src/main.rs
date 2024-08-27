use fltk::{app, enums, frame, input, prelude::*, window::Window};

fn main() {
    let app = app::App::default();

    // create window at the center of the screen
    let mut win = Window::new(0, 0, 400, 400, "Color picker").center_screen();

    // box with color preview
    let mut color_preview = frame::Frame::new(100, 30, 200, 50, "");

    color_preview.set_color(enums::Color::Red);
    color_preview.set_frame(enums::FrameType::FlatBox); // Set the frame type to fill the background

    let mut hex_field = input::Input::new(100, 150, 200, 30, "Hex:");
    hex_field.set_maximum_size(7);
    hex_field.handle(move |field, ev| {
        if ev == enums::Event::KeyDown {
            match app::event_key() {
                enums::Key::Enter => {
                    let color = enums::Color::from_hex_str(&field.value()).unwrap();
                    color_preview.set_color(color);
                    color_preview.redraw();
                    true
                }
                _ => false,
            }
        } else {
            false
        }
    });

    win.end();
    win.show();
    app.run().unwrap();
}
