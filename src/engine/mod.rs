#[macro_use]
mod events;

use sdl2::render::Renderer;
use sdl2::pixels::Color;

struct_events! {
    keyboard: {
        key_escape: Escape,
        key_up: Up,
        key_down: Down,
        key_space: Space
    },
    else: {
    	quit: Quit { .. }
    }
}

pub struct Engine<'window> {
	pub events: Events,
	pub renderer: Renderer<'window>
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

    let window = video.window(title, 1280, 720)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut context = Engine {
    	events: Events::new(sdl_context.event_pump().unwrap()),
    	renderer: window.renderer().accelerated().build().unwrap()
    };

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
            println!("FPS: {}", fps);
            last_second = now;
            fps = 0;
        }

		context.events.pump();

        match current_view.render(&mut context, elapsed) {
            ViewAction::None => context.renderer.present(),
            ViewAction::Quit => break,
            ViewAction::ChangeView(new_view) => current_view = new_view
        }
    }
}