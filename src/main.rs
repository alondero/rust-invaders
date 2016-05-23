extern crate sdl2;

mod engine;
mod views;

fn main() {
    ::engine::spawn("Rust Invaders", |_| {Box::new(::views::ViewA)});
}