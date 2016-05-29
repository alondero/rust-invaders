extern crate sdl2;
extern crate sdl2_image;
extern crate sdl2_ttf;

mod engine;
mod views;

fn main() {
    ::engine::spawn("Rust Invaders", |engine| {Box::new(::views::main_menu::MainMenuView::new(engine))});
}