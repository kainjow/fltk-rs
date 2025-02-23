// Slightly modified translation of https://github.com/fltk/fltk/blob/master/examples/table-simple.cxx

use fltk::*;

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut wind = window::Window::default().with_size(800, 600);
    let mut table = table::Table::default()
        .with_size(800 - 10, 600 - 10)
        .center_of(&wind);

    table.set_rows(30);
    table.set_row_header(true);
    table.set_row_resize(true);
    table.set_cols(26);
    table.set_col_header(true);
    table.set_col_width_all(80);
    table.set_col_resize(true);
    table.end();

    wind.make_resizable(true);
    wind.end();
    wind.show();

    // Called when the table is drawn then when it's redrawn due to events
    table.draw_cell2(move |t, ctx, row, col, x, y, w, h| match ctx {
        table::TableContext::StartPage => draw::set_font(Font::Helvetica, 14),
        table::TableContext::ColHeader => {
            draw_header(&format!("{}", (col + 65) as u8 as char), x, y, w, h)
        } // Column titles
        table::TableContext::RowHeader => draw_header(&format!("{}", row + 1), x, y, w, h), // Row titles
        table::TableContext::Cell => draw_data(
            &format!("{}", row + col),
            x,
            y,
            w,
            h,
            t.is_selected(row, col),
        ), // Data in cells
        _ => (),
    });

    app.run().unwrap();
}

fn draw_header(txt: &str, x: i32, y: i32, w: i32, h: i32) {
    draw::push_clip(x, y, w, h);
    draw::draw_box(FrameType::ThinUpBox, x, y, w, h, Color::FrameDefault);
    draw::set_draw_color(Color::Black);
    draw::draw_text2(txt, x, y, w, h, Align::Center);
    draw::pop_clip();
}

// The selected flag sets the color of the cell to a grayish color, otherwise white
fn draw_data(txt: &str, x: i32, y: i32, w: i32, h: i32, selected: bool) {
    draw::push_clip(x, y, w, h);
    if selected {
        draw::set_draw_color(Color::from_u32(0xD3D3D3));
    } else {
        draw::set_draw_color(Color::White);
    }
    draw::draw_rectf(x, y, w, h);
    draw::set_draw_color(Color::Gray0);
    draw::draw_text2(txt, x, y, w, h, Align::Center);
    draw::draw_rect(x, y, w, h);
    draw::pop_clip();
}
