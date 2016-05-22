#[macro_use]
mod events;

use sdl2::render::Renderer;

struct_events! {
    keyboard: {
        key_escape: Escape,
        key_up: Up,
        key_down: Down
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
	Quit
}

pub trait View {
	fn render(&mut self, context: &mut Engine, elapsed: f64) -> ViewAction;
}