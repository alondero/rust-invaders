extern crate sdl2;

mod engine;
mod views;

fn main() {
    ::engine::spawn("Rust Invaders", |engine| {Box::new(::views::ShipView::new(engine))});
}