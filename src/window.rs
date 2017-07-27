use gl;
use glutin::{self, GlContext};
use super::{Result, Error};

/// Represents a window
pub struct Window {
    pub width: u32,
    pub height: u32,

    evt_loop: glutin::EventsLoop,
    gl_win: glutin::GlWindow
}

impl Window {
    /// Create a new window with the specified title, width
    /// and height
    pub fn new<S: Into<String>>(title: S, width: u32, height: u32) -> Result<Window> {
        // Prepare window creation
        let builder = glutin::WindowBuilder::new()
            .with_title(title)
            .with_dimensions(width, height);

        let evt = glutin::EventsLoop::new();
        let ctx = glutin::ContextBuilder::new();
        let win = match glutin::GlWindow::new(builder, ctx, &evt) {
            Ok(win) => win,
            Err(err) => return Err(Error::WindowCreationError(err))
        };

        unsafe {
            // Try to use the created OpenGL context
            if let Err(err) = win.make_current() {
                return Err(Error::GlContextError(err))
            }

            // Load OpenGL symbols
            gl::load_with(|symbol| win.get_proc_address(symbol) as *const _);
        }

        // Success, return the window representation to the caller
        Ok(Window {
            width: width,
            height: height,

            evt_loop: evt,
            gl_win: win
        })
    }
}
