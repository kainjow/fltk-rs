use crate::image::RgbImage;
pub use crate::prelude::*;
use fltk_sys::draw::*;
use std::ffi::{CStr, CString};
use std::mem;
use std::os::raw;

/// Defines a coordinate of x and y
#[derive(Copy, Clone, Debug)]
pub struct Coord<T: Copy>(pub T, pub T);

bitflags! {
    /// Defines the line styles supported by fltk
    pub struct LineStyle: i32 {
        /// Solid line
        const Solid = 0;
        /// Dash
        const Dash = 1;
        /// Dot
        const Dot =2;
        /// Dash dot
        const DashDot = 3;
        /// Dash dot dot
        const DashDotDot =4;
        /// Cap flat
        const CapFlat = 100;
        /// Cap round
        const CapRound = 200;
        /// Cap square
        const CapSquare = 300;
        /// Join miter
        const JoinMiter = 1000;
        /// Join round
        const JoinRound = 2000;
        /// Join bevel
        const JoinBevel = 3000;
    }
}

/// Opaque type around Fl_Region
pub type Region = *mut raw::c_void;

/// Opaque type around Fl_Offscreen
#[derive(Debug)]
pub struct Offscreen {
    _inner: *mut raw::c_void,
}

unsafe impl Sync for Offscreen {}

unsafe impl Send for Offscreen {}

impl Offscreen {
    /// Creates a new offscreen type
    pub fn new(w: i32, h: i32) -> Option<Offscreen> {
        unsafe {
            let x = Fl_create_offscreen(w, h);
            if x.is_null() {
                None
            } else {
                Some(Offscreen { _inner: x })
            }
        }
    }

    /// Creates an uninitialized offscreen type
    /// # Safety
    /// Leaves the offscreen in an uninitialized state
    pub unsafe fn uninit() -> Offscreen {
        Offscreen {
            _inner: std::ptr::null_mut(),
        }
    }

    /// Begins drawing in the offscreen
    pub fn begin(&self) {
        assert!(!self._inner.is_null());
        unsafe { Fl_begin_offscreen(self._inner) }
    }

    /// Ends drawing in the offscreen
    pub fn end(&self) {
        assert!(!self._inner.is_null());
        unsafe { Fl_end_offscreen() }
    }

    /// Copies the offscreen
    pub fn copy(&self, x: i32, y: i32, w: i32, h: i32, srcx: i32, srcy: i32) {
        assert!(!self._inner.is_null());
        unsafe { Fl_copy_offscreen(x, y, w, h, self._inner, srcx, srcy) }
    }

    /// Rescales the offscreen
    pub fn rescale(&mut self) {
        assert!(!self._inner.is_null());
        unsafe { Fl_rescale_offscreen(self._inner) }
    }

    /// Checks the validity of the offscreen
    pub fn is_valid(&self) -> bool {
        assert!(!self._inner.is_null());
        !self._inner.is_null()
    }

    /// Performs a shallow copy of the offscreen
    /// # Safety
    /// This can lead to multiple mutable references to the same offscreen
    pub unsafe fn shallow_copy(&self) -> Offscreen {
        assert!(!self._inner.is_null());
        Offscreen {
            _inner: self._inner,
        }
    }
}

impl Drop for Offscreen {
    fn drop(&mut self) {
        unsafe { Fl_delete_offscreen(self._inner) }
    }
}

/// Shows a color map
pub fn show_colormap(old_color: Color) -> Color {
    unsafe { mem::transmute(Fl_show_colormap(old_color.bits() as u32)) }
}

/// Sets the color using rgb values
pub fn set_color_rgb(r: u8, g: u8, b: u8) {
    unsafe { Fl_set_color_rgb(r, g, b) }
}

/// Gets the last used color
pub fn get_color() -> Color {
    unsafe { mem::transmute(Fl_get_color()) }
}

/// Draws a line
pub fn draw_line(x1: i32, y1: i32, x2: i32, y2: i32) {
    unsafe {
        Fl_line(x1, y1, x2, y2);
    }
}

/// Draws a line from (x,y) to (x1,y1) and another from (x1,y1) to (x2,y2)
pub fn draw_line2(pos1: Coord<i32>, pos2: Coord<i32>, pos3: Coord<i32>) {
    unsafe { Fl_line2(pos1.0, pos1.1, pos2.0, pos2.1, pos3.0, pos3.1) }
}

/// Draws a point
pub fn draw_point(x: i32, y: i32) {
    unsafe { Fl_point(x, y) }
}

/// Draws a point
pub fn draw_point2(pos: Coord<i32>) {
    unsafe { Fl_point(pos.0, pos.1) }
}

/// Draws a rectangle
pub fn draw_rect(x: i32, y: i32, w: i32, h: i32) {
    unsafe { Fl_rect(x, y, w, h) }
}

/// Draws a rectangle with border color
pub fn draw_rect_with_color(x: i32, y: i32, w: i32, h: i32, color: Color) {
    unsafe { Fl_rect_with_color(x, y, w, h, color.bits() as u32) }
}

/// Draws a non-filled 3-sided polygon
pub fn draw_loop(x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32) {
    unsafe {
        Fl_loop(x1, y1, x2, y2, x3, y3);
    }
}

/// Draws a non-filled 3-sided polygon
pub fn draw_loop2(pos1: Coord<i32>, pos2: Coord<i32>, pos3: Coord<i32>) {
    unsafe { Fl_loop(pos1.0, pos1.1, pos2.0, pos2.1, pos3.0, pos3.1) }
}

/// Draws a non-filled 4-sided polygon
pub fn draw_loop3(pos1: Coord<i32>, pos2: Coord<i32>, pos3: Coord<i32>, pos4: Coord<i32>) {
    unsafe {
        Fl_loop2(
            pos1.0, pos1.1, pos2.0, pos2.1, pos3.0, pos3.1, pos4.0, pos4.1,
        )
    }
}

/// Draws a filled rectangle
pub fn draw_rect_fill(x: i32, y: i32, w: i32, h: i32, color: Color) {
    unsafe { Fl_rectf_with_color(x, y, w, h, color.bits() as u32) }
}

/// Draws a focus rectangle
pub fn draw_focus_rect(x: i32, y: i32, w: i32, h: i32) {
    unsafe { Fl_focus_rect(x, y, w, h) }
}

/// Sets the drawing color
pub fn set_draw_hex_color(color: u32) {
    let (r, g, b) = crate::utils::hex2rgb(color);
    unsafe { Fl_set_color_rgb(r, g, b) }
}

/// Sets the drawing color
pub fn set_draw_rgb_color(r: u8, g: u8, b: u8) {
    unsafe { Fl_set_color_rgb(r, g, b) }
}

/// Sets the drawing color
pub fn set_draw_color(color: Color) {
    unsafe { Fl_set_color_int(color.bits() as u32) }
}

/// Draws a circle
pub fn draw_circle(x: f64, y: f64, r: f64) {
    unsafe {
        Fl_circle(x, y, r);
    }
}

/// Draws an arc
pub fn draw_arc(x: i32, y: i32, width: i32, height: i32, a: f64, b: f64) {
    unsafe {
        Fl_arc(x, y, width, height, a, b);
    }
}

/// Draws an arc
pub fn draw_arc2(x: f64, y: f64, r: f64, start: f64, end: f64) {
    unsafe { Fl_arc2(x, y, r, start, end) }
}

/// Draws a filled pie
pub fn draw_pie(x: i32, y: i32, width: i32, height: i32, a: f64, b: f64) {
    unsafe {
        Fl_pie(x, y, width, height, a, b);
    }
}

/// Sets the line style
pub fn set_line_style(style: LineStyle, width: i32) {
    unsafe {
        Fl_line_style(
            style.bits(),
            width,
            std::ptr::null_mut() as *mut std::os::raw::c_char,
        );
    }
}

/// Limits drawing to a region
pub fn push_clip(x: i32, y: i32, w: i32, h: i32) {
    unsafe {
        Fl_push_clip(x, y, w, h);
    }
}

/// Puts the drawing back
pub fn pop_clip() {
    unsafe {
        Fl_pop_clip();
    }
}

/// Sets the clip region
pub fn set_clip_region(r: Region) {
    assert!(!r.is_null());
    unsafe { Fl_set_clip_region(r) }
}

/// Gets the clip region
pub fn clip_region() -> Region {
    unsafe {
        let ptr = Fl_clip_region();
        assert!(!ptr.is_null());
        ptr
    }
}

/// Pushes an empty clip region onto the stack so nothing will be clipped
pub fn push_no_clip() {
    unsafe { Fl_push_no_clip() }
}

/// Returns whether the rectangle intersect with the current clip region
pub fn not_clipped(x: i32, y: i32, w: i32, h: i32) -> bool {
    unsafe { Fl_not_clipped(x, y, w, h) != 0 }
}

/// Restores the clip region
pub fn restore_clip() {
    unsafe { Fl_restore_clip() }
}

/// Transforms coordinate using the current transformation matrix
pub fn transform_x(x: f64, y: f64) -> f64 {
    unsafe { Fl_transform_x(x, y) }
}

/// Transforms coordinate using the current transformation matrix
pub fn transform_y(x: f64, y: f64) -> f64 {
    unsafe { Fl_transform_y(x, y) }
}

/// Transforms distance using current transformation matrix
pub fn transform_dx(x: f64, y: f64) -> f64 {
    unsafe { Fl_transform_dx(x, y) }
}

/// Transforms distance using current transformation matrix
pub fn transform_dy(x: f64, y: f64) -> f64 {
    unsafe { Fl_transform_dy(x, y) }
}

/// Adds coordinate pair to the vertex list without further transformations
pub fn transformed_vertex(xf: f64, yf: f64) {
    unsafe { Fl_transformed_vertex(xf, yf) }
}

/// Draws a filled rectangle
pub fn draw_rectf(x: i32, y: i32, w: i32, h: i32) {
    unsafe { Fl_rectf(x, y, w, h) }
}

/// Draws a filled rectangle with specified RGB color
pub fn draw_rectf_with_rgb(
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    color_r: u8,
    color_g: u8,
    color_b: u8,
) {
    unsafe { Fl_rectf_with_rgb(x, y, width, height, color_r, color_g, color_b) }
}

/// Fills a 3-sided polygon. The polygon must be convex
pub fn draw_polygon(x: i32, y: i32, x1: i32, y1: i32, x2: i32, y2: i32) {
    unsafe { Fl_polygon(x, y, x1, y1, x2, y2) }
}

/// Fills a 3-sided polygon. The polygon must be convex
pub fn draw_polygon2(pos1: Coord<i32>, pos2: Coord<i32>, pos3: Coord<i32>) {
    unsafe { Fl_polygon(pos1.0, pos1.1, pos2.0, pos2.1, pos3.0, pos3.1) }
}

/// Fills a 4-sided polygon. The polygon must be convex
pub fn draw_polygon3(pos1: Coord<i32>, pos2: Coord<i32>, pos3: Coord<i32>, pos4: Coord<i32>) {
    unsafe {
        Fl_polygon2(
            pos1.0, pos1.1, pos2.0, pos2.1, pos3.0, pos3.1, pos4.0, pos4.1,
        )
    }
}

/// Adds a series of points on a Bezier curve to the path
pub fn draw_curve(pos1: Coord<f64>, pos2: Coord<f64>, pos3: Coord<f64>, pos4: Coord<f64>) {
    unsafe {
        Fl_curve(
            pos1.0, pos1.1, pos2.0, pos2.1, pos3.0, pos3.1, pos4.0, pos4.1,
        )
    }
}

/// Draws a horizontal line from (x,y) to (x1,y)
pub fn draw_xyline(x: i32, y: i32, x1: i32) {
    unsafe { Fl_xyline(x, y, x1) }
}

/// Draws a horizontal line from (x,y) to (x1,y), then vertical from (x1,y) to (x1,y2)
pub fn draw_xyline2(x: i32, y: i32, x1: i32, y2: i32) {
    unsafe { Fl_xyline2(x, y, x1, y2) }
}

/// Draws a horizontal line from (x,y) to (x1,y), then a vertical from (x1,y) to (x1,y2)
/// and then another horizontal from (x1,y2) to (x3,y2)
pub fn draw_xyline3(x: i32, y: i32, x1: i32, y2: i32, x3: i32) {
    unsafe { Fl_xyline3(x, y, x1, y2, x3) }
}

/// Draws a vertical line from (x,y) to (x,y1)
pub fn draw_yxline(x: i32, y: i32, y1: i32) {
    unsafe { Fl_yxline(x, y, y1) }
}

/// Draws a vertical line from (x,y) to (x,y1), then a horizontal from (x,y1) to (x2,y1)
pub fn draw_yxline2(x: i32, y: i32, y1: i32, x2: i32) {
    unsafe { Fl_yxline2(x, y, y1, x2) }
}

///  Draws a vertical line from (x,y) to (x,y1) then a horizontal from (x,y1)
/// to (x2,y1), then another vertical from (x2,y1) to (x2,y3)
pub fn draw_yxline3(x: i32, y: i32, y1: i32, x2: i32, y3: i32) {
    unsafe { Fl_yxline3(x, y, y1, x2, y3) }
}

/// Saves the current transformation matrix on the stack
pub fn push_matrix() {
    unsafe { Fl_push_matrix() }
}

/// Pops the current transformation matrix from the stack
pub fn pop_matrix() {
    unsafe { Fl_pop_matrix() }
}

/// Concatenates scaling transformation onto the current one
pub fn scale_xy(x: f64, y: f64) {
    unsafe { Fl_scale(x, y) }
}

/// Concatenates scaling transformation onto the current one
pub fn scale_x(x: f64) {
    unsafe { Fl_scale2(x) }
}

/// Concatenates translation transformation onto the current one
pub fn translate(x: f64, y: f64) {
    unsafe { Fl_translate(x, y) }
}

/// Concatenates rotation transformation onto the current one
pub fn rotate(d: f64) {
    unsafe { Fl_rotate(d) }
}

/// Concatenates another transformation onto the current one
pub fn mult_matrix(val_a: f64, val_b: f64, val_c: f64, val_d: f64, x: f64, y: f64) {
    unsafe { Fl_mult_matrix(val_a, val_b, val_c, val_d, x, y) }
}

/// Starts drawing a list of points. Points are added to the list with fl_vertex()
pub fn begin_points() {
    unsafe { Fl_begin_points() }
}

/// Starts drawing a list of lines
pub fn begin_line() {
    unsafe { Fl_begin_line() }
}

/// Starts drawing a closed sequence of lines
pub fn begin_loop() {
    unsafe { Fl_begin_loop() }
}

/// Starts drawing a convex filled polygon
pub fn begin_polygon() {
    unsafe { Fl_begin_polygon() }
}

/// Adds a single vertex to the current path
pub fn vertex(x: f64, y: f64) {
    unsafe { Fl_vertex(x, y) }
}

/// Ends list of points, and draws
pub fn end_points() {
    unsafe { Fl_end_points() }
}

/// Ends list of lines, and draws
pub fn end_line() {
    unsafe { Fl_end_line() }
}

/// Ends closed sequence of lines, and draws
pub fn end_loop() {
    unsafe { Fl_end_loop() }
}

/// Ends closed sequence of lines, and draws
pub fn end_polygon() {
    unsafe { Fl_end_polygon() }
}

/// Starts drawing a complex filled polygon
pub fn begin_complex_polygon() {
    unsafe { Fl_begin_complex_polygon() }
}

/// Call gap() to separate loops of the path
pub fn gap() {
    unsafe { Fl_gap() }
}

/// Ends complex filled polygon, and draws
pub fn end_complex_polygon() {
    unsafe { Fl_end_complex_polygon() }
}

/// Sets the current font, which is then used in various drawing routines
pub fn set_font(face: Font, fsize: u32) {
    unsafe { Fl_set_draw_font(face.bits() as i32, fsize as i32) }
}

/// Gets the current font, which is used in various drawing routines
pub fn font() -> Font {
    unsafe { mem::transmute(Fl_font()) }
}

/// Gets the current font size, which is used in various drawing routines
pub fn size() -> u32 {
    unsafe { Fl_size() as u32 }
}

/// Returns the recommended minimum line spacing for the current font
pub fn height() -> i32 {
    unsafe { Fl_height() }
}

/// Sets the line spacing for the current font
pub fn set_height(font: Font, size: u32) {
    unsafe {
        Fl_set_height(font.bits() as i32, size as i32);
    }
}

/// Returns the recommended distance above the bottom of a height() tall box to
/// draw the text at so it looks centered vertically in that box
pub fn descent() -> i32 {
    unsafe { Fl_descent() }
}

/// Returns the typographical width of a string
pub fn width(txt: &str) -> f64 {
    let txt = CString::safe_new(txt);
    unsafe { Fl_width(txt.as_ptr()) }
}

/// Returns the typographical width of a sequence of n characters
pub fn width2(txt: &str, n: i32) -> f64 {
    let txt = CString::safe_new(txt);
    unsafe { Fl_width2(txt.as_ptr(), n) }
}

/// Measure the width and height of a text
pub fn measure(txt: &str, draw_symbols: bool) -> (i32, i32) {
    let txt = CString::safe_new(txt);
    let mut x = 0;
    let mut y = 0;
    unsafe {
        Fl_measure(txt.as_ptr(), &mut x, &mut y, draw_symbols as i32);
    }
    (x, y)
}

/// Returns the typographical width of a single character
pub fn char_width(c: char) -> f64 {
    unsafe { Fl_width3(c as u32) }
}

/// Converts text from Windows/X11 latin1 character set to local encoding
pub fn latin1_to_local(txt: &str, n: i32) -> String {
    let txt = CString::safe_new(txt);
    unsafe {
        let x = Fl_latin1_to_local(txt.as_ptr(), n);
        assert!(!x.is_null());
        CStr::from_ptr(x as *mut raw::c_char)
            .to_string_lossy()
            .to_string()
    }
}

/// Converts text from local encoding to Windowx/X11 latin1 character set
pub fn local_to_latin1(txt: &str, n: i32) -> String {
    let txt = CString::safe_new(txt);
    unsafe {
        let x = Fl_local_to_latin1(txt.as_ptr(), n);
        assert!(!x.is_null());
        CStr::from_ptr(x as *mut raw::c_char)
            .to_string_lossy()
            .to_string()
    }
}

/// Draws a string starting at the given x, y location
pub fn draw_text(txt: &str, x: i32, y: i32) {
    let txt = CString::safe_new(txt);
    unsafe { Fl_draw(txt.as_ptr(), x, y) }
}

/// Draws a string starting at the given x, y location with width and height and alignment
pub fn draw_text2(string: &str, x: i32, y: i32, width: i32, height: i32, align: Align) {
    let s = CString::safe_new(string);
    unsafe { Fl_draw_text2(s.as_ptr(), x, y, width, height, align.bits() as i32) }
}

/// Draws a string starting at the given x, y location, rotated to an angle
pub fn draw_text_angled(angle: i32, txt: &str, x: i32, y: i32) {
    let txt = CString::safe_new(txt);
    unsafe { Fl_draw2(angle, txt.as_ptr(), x, y) }
}

/// Draws a UTF-8 string right to left starting at the given x, y location
pub fn rtl_draw(txt: &str, x: i32, y: i32) {
    let len = txt.len() as i32;
    let txt = CString::safe_new(txt);
    unsafe { Fl_rtl_draw(txt.as_ptr(), len, x, y) }
}

/// Draws a frame with text
pub fn draw_frame(string: &str, x: i32, y: i32, width: i32, height: i32) {
    let s = CString::safe_new(string);
    unsafe { Fl_frame(s.as_ptr(), x, y, width, height) }
}

/// Draws a frame with text.
/// Differs from frame() by the order of the line segments
pub fn draw_frame2(string: &str, x: i32, y: i32, width: i32, height: i32) {
    let s = CString::safe_new(string);
    unsafe { Fl_frame2(s.as_ptr(), x, y, width, height) }
}

/// Draws a box given the box type, size, position and color
pub fn draw_box(box_type: FrameType, x: i32, y: i32, w: i32, h: i32, color: Color) {
    unsafe { Fl_draw_box(box_type as i32, x, y, w, h, color.bits() as u32) }
}

/// Checks whether platform supports true alpha blending for RGBA images
pub fn can_do_alpha_blending() -> bool {
    unsafe { Fl_can_do_alpha_blending() != 0 }
}

/// Get a human-readable string from a shortcut value
pub fn shortcut_label(shortcut: Shortcut) -> String {
    unsafe {
        let x = Fl_shortcut_label(shortcut.bits() as u32);
        assert!(!x.is_null());
        CStr::from_ptr(x as *mut raw::c_char)
            .to_string_lossy()
            .to_string()
    }
}

/// Draws a selection rectangle, erasing a previous one by XOR'ing it first.
pub fn overlay_rect(x: i32, y: i32, w: i32, h: i32) {
    unsafe { Fl_overlay_rect(x, y, w, h) }
}

/// Erase a selection rectangle without drawing a new one
pub fn overlay_clear() {
    unsafe { Fl_overlay_clear() }
}

/// Sets the cursor style
pub fn set_cursor(cursor: Cursor) {
    unsafe { Fl_set_cursor(cursor as i32) }
}

/// Sets the cursor style
pub fn set_cursor_with_color(cursor: Cursor, fg: Color, bg: Color) {
    unsafe { Fl_set_cursor2(cursor as i32, fg.bits() as i32, bg.bits() as i32) }
}

/// Sets the status
pub fn set_status(x: i32, y: i32, w: i32, h: i32) {
    unsafe { Fl_set_status(x, y, w, h) }
}

/// Sets spot within the window
pub fn set_spot<Win: WindowExt>(font: Font, size: u32, x: i32, y: i32, w: i32, h: i32, win: &Win) {
    unsafe {
        assert!(!win.was_deleted());
        Fl_set_spot(
            font.bits() as i32,
            size as i32,
            x,
            y,
            w,
            h,
            win.as_widget_ptr() as *mut raw::c_void,
        )
    }
}

/// Resets the spot within the window
pub fn reset_spot() {
    unsafe { Fl_reset_spot() }
}

/// Captures part of the window and returns raw data.
/// Example usage:
/// ```no_run
/// use fltk::*;
/// let mut win = window::Window::default();
/// let image = draw::capture_window(&mut win).unwrap().into_jpeg().unwrap();
/// image
///    .write_to_file(&std::path::PathBuf::from("test.jpg"))
///    .unwrap();
/// ```
pub fn capture_window<Win: WindowExt>(win: &mut Win) -> Result<RgbImage, FltkError> {
    assert!(!win.was_deleted());
    let cp = win.width() as u32 * win.height() as u32 * 3;
    win.show();
    unsafe {
        let x = Fl_read_image(std::ptr::null_mut(), 0, 0, win.width(), win.height(), 0);
        if x.is_null() {
            Err(FltkError::Internal(FltkErrorKind::FailedOperation))
        } else {
            let x = std::slice::from_raw_parts(x, cp as usize);
            Ok(RgbImage::new(
                x,
                win.width() as u32,
                win.height() as u32,
                ColorDepth::Rgb8,
            )?)
        }
    }
}

/// Draw a framebuffer (rgba) into a widget
pub fn draw_rgba<'a, T: WidgetBase>(wid: &'a mut T, fb: &'a [u8]) -> Result<(), FltkError> {
    let width = wid.width() as u32;
    let height = wid.height() as u32;
    let mut img = crate::image::RgbImage::new(fb, width, height, ColorDepth::Rgba8)?;
    wid.draw2(move |s| {
        let x = s.x();
        let y = s.y();
        let w = s.width();
        let h = s.height();
        img.scale(w, h, false, true);
        img.draw(x, y, w, h);
    });
    Ok(())
}

/// Draw a framebuffer (rgba) into a widget
/// # Safety
/// The data passed should be valid and outlive the widget
pub unsafe fn draw_rgba_nocopy<T: WidgetBase>(wid: &mut T, fb: &[u8]) {
    let ptr = fb.as_ptr();
    let len = fb.len();
    let width = wid.width() as u32;
    let height = wid.height() as u32;
    wid.draw2(move |s| {
        let x = s.x();
        let y = s.y();
        let w = s.width();
        let h = s.height();
        if let Ok(mut img) = crate::image::RgbImage::from_data(
            std::slice::from_raw_parts(ptr, len),
            width,
            height,
            ColorDepth::Rgba8,
        ) {
            img.scale(w, h, false, true);
            img.draw(x, y, w, h);
        }
    });
}

/// Draw a framebuffer (rgba) into a widget
pub fn draw_rgb<'a, T: WidgetBase>(wid: &'a mut T, fb: &'a [u8]) -> Result<(), FltkError> {
    let width = wid.width() as u32;
    let height = wid.height() as u32;
    let mut img = crate::image::RgbImage::new(fb, width, height, ColorDepth::Rgb8)?;
    wid.draw2(move |s| {
        let x = s.x();
        let y = s.y();
        let w = s.width();
        let h = s.height();
        img.scale(w, h, false, true);
        img.draw(x, y, w, h);
    });
    Ok(())
}

/// Draw a framebuffer (rgba) into a widget
/// # Safety
/// The data passed should be valid and outlive the widget
pub unsafe fn draw_rgb_nocopy<T: WidgetBase>(wid: &mut T, fb: &[u8]) {
    let ptr = fb.as_ptr();
    let len = fb.len();
    let width = wid.width() as u32;
    let height = wid.height() as u32;
    wid.draw2(move |s| {
        let x = s.x();
        let y = s.y();
        let w = s.width();
        let h = s.height();
        if let Ok(mut img) = crate::image::RgbImage::from_data(
            std::slice::from_raw_parts(ptr, len),
            width,
            height,
            ColorDepth::Rgb8,
        ) {
            img.scale(w, h, false, true);
            img.draw(x, y, w, h);
        }
    });
}

/// Draw an image into a widget.
/// Requires a call to app::set_visual(Mode::Rgb8).unwrap()
pub fn draw_image(
    data: &[u8],
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    depth: ColorDepth,
) -> Result<(), FltkError> {
    let sz = (w * h * depth as i32) as usize;
    if sz > data.len() {
        return Err(FltkError::Internal(FltkErrorKind::ImageFormatError));
    }
    unsafe {
        Fl_draw_image(data.as_ptr(), x, y, w, h, depth as i32, 0);
    }
    Ok(())
}

/// Transforms raw data to png file
pub fn write_to_png_file<I: ImageExt, P: AsRef<std::path::Path>>(
    image: &I,
    path: P,
) -> Result<(), FltkError> {
    write_to_png_file_(image, path.as_ref())
}

fn write_to_png_file_<I: ImageExt>(image: &I, path: &std::path::Path) -> Result<(), FltkError> {
    assert!(
        std::any::type_name::<I>() != std::any::type_name::<crate::image::SvgImage>(),
        "SVG images are not supported!"
    );
    let path = path.to_str();
    if path.is_none() {
        return Err(FltkError::IoError(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Could not convert path to string!",
        )));
    }
    let path = std::ffi::CString::new(path.unwrap())?;
    unsafe {
        match Fl_raw_image_to_png(
            *image.to_raw_data() as *mut u8,
            path.as_ptr(),
            image.data_w() as i32,
            image.data_h() as i32,
        ) {
            -1 => Err(FltkError::IoError(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Could not write image!",
            ))),
            _ => Ok(()),
        }
    }
}

/// Transforms raw data to jpg file
pub fn write_to_jpg_file<I: ImageExt, P: AsRef<std::path::Path>>(
    image: &I,
    path: P,
) -> Result<(), FltkError> {
    write_to_jpg_file_(image, path.as_ref())
}

fn write_to_jpg_file_<I: ImageExt>(image: &I, path: &std::path::Path) -> Result<(), FltkError> {
    assert!(
        std::any::type_name::<I>() != std::any::type_name::<crate::image::SvgImage>(),
        "SVG images are not supported!"
    );
    let path = path.to_str();
    if path.is_none() {
        return Err(FltkError::IoError(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Could not convert path to string!",
        )));
    }
    let path = std::ffi::CString::new(path.unwrap())?;
    unsafe {
        match Fl_raw_image_to_jpg(
            *image.to_raw_data() as *mut u8,
            path.as_ptr(),
            image.data_w() as i32,
            image.data_h() as i32,
        ) {
            -1 => Err(FltkError::IoError(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Could not write image!",
            ))),
            _ => Ok(()),
        }
    }
}

/// Transforms raw data to bmp file
pub fn write_to_bmp_file<I: ImageExt, P: AsRef<std::path::Path>>(
    image: &I,
    path: P,
) -> Result<(), FltkError> {
    write_to_bmp_file_(image, path.as_ref())
}

fn write_to_bmp_file_<I: ImageExt>(image: &I, path: &std::path::Path) -> Result<(), FltkError> {
    assert!(
        std::any::type_name::<I>() != std::any::type_name::<crate::image::SvgImage>(),
        "SVG images are not supported!"
    );
    let path = path.to_str();
    if path.is_none() {
        return Err(FltkError::IoError(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Could not convert path to string!",
        )));
    }
    let path = std::ffi::CString::new(path.unwrap())?;
    unsafe {
        match Fl_raw_image_to_bmp(
            *image.to_raw_data() as *mut u8,
            path.as_ptr(),
            image.data_w() as i32,
            image.data_h() as i32,
        ) {
            -1 => Err(FltkError::IoError(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Could not write image!",
            ))),
            _ => Ok(()),
        }
    }
}
