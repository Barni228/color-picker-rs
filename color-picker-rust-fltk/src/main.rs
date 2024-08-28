use color_picker_rust_fltk::Color;
use fltk::{app, dialog, enums, frame, input, prelude::*, window::Window};

fn main() {
    let app = app::App::default();

    // create window at the center of the screen
    let mut win = Window::new(0, 0, 400, 400, "Color picker").center_screen();

    // box with color preview
    let mut color_preview = frame::Frame::new(100, 30, 200, 50, "");
    color_preview.set_color(enums::Color::Red);
    color_preview.set_frame(enums::FrameType::FlatBox); // Set the frame type to fill the background

    // text area with warnings
    let mut warnings = frame::Frame::new(100, 162, 200, 50, "");
    warnings.set_label_color(enums::Color::Red);

    // input field
    let mut hex_field = input::Input::new(100, 150, 200, 30, "Hex:");
    hex_field.set_maximum_size(7);
    hex_field.handle(move |field, ev| {
        // if user pressed key
        if ev == enums::Event::KeyDown {
            match app::event_key() {
                // if user pressed enter, try to convert hex to color
                enums::Key::Enter => match Color::try_from(field.value()) {
                    // if it is valid, set color preview to that color
                    Ok(color) => {
                        color_preview.set_color(enums::Color::from_hex_str(&color.hex()).unwrap());
                        color_preview.redraw();
                        true
                    }

                    // if it is not valid, set warning and play beep sound
                    Err(e) => {
                        warnings.set_label(&e.to_string());
                        dialog::beep(dialog::BeepType::Error);
                        true
                    }
                },

                // if user pressed another key, update warnings
                _ => {
                    // if field is empty, remove warnings
                    if field.value().is_empty() {
                        warnings.set_label("");
                    } else {
                        warnings.set_label(&match Color::try_from(field.value()) {
                            Ok(_) => String::from(""), // if color is Ok, clear warnings
                            Err(e) => e.to_string(),   // if color is not Ok, display warnings
                        });
                    }
                    false // we didn't fully handle the event, so return false
                }
            }

        // if user did something else, do nothing
        } else {
            false
        }
    });

    win.end();
    win.show();
    app.run().unwrap();
}
