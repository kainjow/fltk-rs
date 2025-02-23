// FLTK offers 16 fonts by default. However, it's possible to load all system fonts to be able to choose from them:
// The following are the default FLTK fonts:
// - Helvetica,
// - HelveticaBold,
// - HelveticaItalic,
// - HelveticaBoldItalic,
// - Courier,
// - CourierBold,
// - CourierItalic,
// - CourierBoldItalic,
// - Times,
// - TimesBold,
// - TimesItalic,
// - TimesBoldItalic,
// - Symbol,
// - Screen,
// - ScreenBold,
// - Zapfdingbats,
//
// The system fonts depend on the system, and are not loaded by default.
// These can be loaded using the App::load_system_fonts() method.
// The fonts can then be aquired using the app::fonts() function
// or be queried using the app::font_count(), app::font_name() and app::font_index() functions.
// And the can be used using the Font::by_index() or Font::by_name() methods.

use fltk::*;

fn main() {
    let app = app::App::default().load_system_fonts();
    // To load a font by path, check the app::load_font() function
    let fonts = app::fonts();
    // println!("{:?}", fonts);
    let mut wind = window::Window::default().with_size(400, 300);
    let mut frame = frame::Frame::default().size_of(&wind);
    frame.set_label_size(30);
    wind.set_color(Color::White);
    wind.end();
    wind.show();
    println!("The system has {} fonts!\nStarting slideshow!", fonts.len());
    let mut i = 0;
    while app.wait() {
        if i == fonts.len() {
            i = 0;
        }
        frame.set_label(&format!("[{}]", fonts[i]));
        frame.set_label_font(Font::by_index(i));
        app::sleep(0.5);
        i += 1;
    }
}
