use gl;
use glutin::{self, GlContext};
use super::{Result, Error};

/// Represents a window
pub struct Window {
    pub width: u32,
    pub height: u32,
    pub should_exit: bool,

    evt_loop: glutin::EventsLoop,
    gl_win: glutin::GlWindow
}

impl Window {
    /// Create a new window with the specified title, width
    /// and height
    pub fn new<S: Into<String>>(title: S, width: u32, height: u32, vsync: bool) -> Result<Window> {
        // Prepare window creation
        let builder = glutin::WindowBuilder::new()
            .with_title(title)
            .with_dimensions(width, height)
            .with_min_dimensions(width, height)
            .with_max_dimensions(width, height);

        // Create OpenGL context
        let ctx = glutin::ContextBuilder::new()
            .with_gl_profile(glutin::GlProfile::Compatibility)
            .with_vsync(vsync);

        // Create event loop & window
        let evt = glutin::EventsLoop::new();
        let win = match glutin::GlWindow::new(builder, ctx, &evt) {
            Ok(win) => win,
            Err(err) => return Err(Error::WindowCreation(err))
        };

        unsafe {
            // Try to use the created OpenGL context
            if let Err(err) = win.make_current() {
                return Err(Error::GlContext(err))
            }

            // Load OpenGL symbols
            gl::load_with(|symbol| win.get_proc_address(symbol) as *const _);
        }

        // Success, return the window representation to the caller
        Ok(Window {
            width: width,
            height: height,
            should_exit: false,

            evt_loop: evt,
            gl_win: win
        })
    }

    /// Handle the events related to the window, using the specified
    /// closure as a callback that will be invoked for each event
    pub fn handle_events<F: FnMut(glutin::WindowEvent)>(&mut self, mut callback: F) {
        let mut should_exit = false;

        self.evt_loop.poll_events(|evt| match evt {
            glutin::Event::WindowEvent{ event, .. } => match event {
                glutin::WindowEvent::Closed => should_exit = true,
                _ => callback(event)
            },
            _ => ()
        });

        self.should_exit = should_exit;
    }

    /// Swap the two rendering buffers, present the renderer
    /// frame to the screen
    pub fn swap_buffers(&self) {
        self.gl_win.swap_buffers().unwrap();
    }
}
