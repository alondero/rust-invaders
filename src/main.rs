extern crate sdl2;

mod engine;

use sdl2::pixels::Color;
use engine::Events;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();

    let window = video.window("Rust Invaders", 1280, 720)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.renderer()
        .accelerated()
        .build()
        .unwrap();

    let mut events = Events::new(sdl_context.event_pump().unwrap());

    loop {
    	events.pump();

    	if events.now.quit || events.now.key_escape == Some(true) {
    		break;
    	}
    
    	renderer.set_draw_color(Color::RGB(0, 0, 0));
    	renderer.clear();
    	renderer.present();
    }
}