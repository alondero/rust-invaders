#[macro_use]
mod events;
pub mod data;
pub mod graphics;

use self::graphics::Sprite;
use sdl2::render::Renderer;
use sdl2::pixels::Color;
use sdl2_ttf::Sdl2TtfContext;
use std::collections::HashMap;
use std::path::Path;

struct_events! {
    keyboard: {
        key_escape: Escape,
        key_up: Up,
        key_down: Down,
        key_left: Left,
        key_right: Right,
        key_space: Space
    },
    else: {
    	quit: Quit { .. }
    }
}

pub struct Engine<'window> {
	pub events: Events,
	pub renderer: Renderer<'window>,
	pub ttf_context: Sdl2TtfContext,

	cached_fonts: HashMap<(&'static str, i32), ::sdl2_ttf::Font>
}

impl<'window> Engine<'window> {
    pub fn output_size(&self) -> (f64, f64) {
        let (w, h) = self.renderer.output_size().unwrap();
        (w as f64, h as f64)
    }

    fn new(events: Events, renderer: Renderer<'window>, ttf_context: Sdl2TtfContext) -> Engine<'window> {
    	Engine {
    		events: events,
    		renderer: renderer,
    		ttf_context: ttf_context,

    		cached_fonts: HashMap::new()
    	}
    }

    pub fn ttf_str_sprite(&mut self, text: &str, font_path: &'static str, size: i32, colour: Color) -> Option<Sprite> {
    	if let Some(font) = self.cached_fonts.get(&(font_path, size)) {
            return font.render(text)
						.blended(colour).ok()
	    				.and_then(|surface| self.renderer.create_texture_from_surface(&surface).ok())
	    				.map(Sprite::new)
        }

    	self.ttf_context.load_font(Path::new(font_path), size as u16).ok()
    		.and_then(|font| {
    			self.cached_fonts.insert((font_path, size), font);
    			self.ttf_str_sprite(text, font_path, size, colour)
    		})
    }
}

pub enum ViewAction {
	None,
	Quit,
	ChangeView(Box<View>)
}

pub trait View {
	fn render(&mut self, context: &mut Engine, elapsed: f64) -> ViewAction;
}

/// Create a window with name `title`, initialize the underlying libraries and
/// start the game with the `View` returned by `init()`.
///
/// # Examples
///
/// Here, we simply show a window with color #ffff00 and exit when escape is
/// pressed or when the window is closed.
///
/// ```
/// struct MyView;
///
/// impl View for MyView {
///     fn render(&mut self, context: &mut Phi, _: f64) -> ViewAction {
///         if context.events.now.quit {
///             return ViewAction::Quit;
///         }
///
///         context.renderer.set_draw_color(Color::RGB(255, 255, 0));
///         context.renderer.clear();
///         ViewAction::None
///     }
/// }
///
/// spawn("Example", |_| {
///     Box::new(MyView)
/// });
/// ```
pub fn spawn<F>(title: &str, init: F) where F: Fn(&mut Engine) -> Box<View> {
    
    let sdl_context = ::sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();
    let mut timer = sdl_context.timer().unwrap();
	let _image_context = ::sdl2_image::init(::sdl2_image::INIT_PNG).unwrap();
	let _ttf_context = ::sdl2_ttf::init().unwrap();

    let window = video.window(title, 1280, 720)
        .position_centered()
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let mut context = Engine::new(
    	Events::new(sdl_context.event_pump().unwrap()),
    	window.renderer().accelerated().build().unwrap(),
    	_ttf_context
    );

    let mut current_view = init(&mut context);

    let interval = 1_000 / 60;
    let mut before = timer.ticks();
    let mut last_second = timer.ticks();
    let mut fps = 0u16;

    loop {
    	let now = timer.ticks();
        let dt = now - before;
        let elapsed = dt as f64 / 1_000.0;

        if dt < interval {
            timer.delay(interval - dt);
            continue;
        }

        before = now;
        fps += 1;

        if now - last_second > 1_000 {
            last_second = now;
            fps = 0;
        }

		context.events.pump(&mut context.renderer);

        match current_view.render(&mut context, elapsed) {
            ViewAction::None => context.renderer.present(),
            ViewAction::Quit => break,
            ViewAction::ChangeView(new_view) => current_view = new_view
        }
    }
}