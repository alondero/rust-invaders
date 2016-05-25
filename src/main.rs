extern crate sdl2;
extern crate sdl2_image;

mod engine;
mod views;

fn main() {
    ::engine::spawn("Rust Invaders", |engine| {Box::new(::views::ShipView::new(engine))});
}