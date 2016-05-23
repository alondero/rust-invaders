extern crate sdl2;

mod engine;
mod views;

use sdl2::pixels::Color;
use engine::{Events, Engine, View, ViewAction};

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();

    let window = video.window("Rust Invaders", 1280, 720)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut context = Engine {
    	events: Events::new(sdl_context.event_pump().unwrap()),
    	renderer: window.renderer().accelerated().build().unwrap()
    };

    let mut current_view: Box<View> = Box::new(::views::DefaultView);

    loop {
    	context.events.pump();

    	match current_view.render(&mut context, 0.01) {
    		ViewAction::None => context.renderer.present(),
    		ViewAction::Quit => break
    	}
    }
}