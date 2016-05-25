use engine::{Engine, View, ViewAction};
use engine::data::Rectangle;
use engine::graphics::{Sprite, CopySprite};
use sdl2::pixels::Color;
use std::path::Path;
use sdl2::render::{Texture, TextureQuery};
use sdl2_image::LoadTexture;

const PLAYER_SPEED: f64 = 360.0;
const SHIP_W: f64 = 43.0;
const SHIP_H: f64 = 39.0;

#[derive(Clone, Copy)]
enum ShipFrame {
    UpNorm   = 0,
    UpFast   = 1,
    UpSlow   = 2,
    MidNorm  = 3,
    MidFast  = 4,
    MidSlow  = 5,
    DownNorm = 6,
    DownFast = 7,
    DownSlow = 8
}

pub struct ShipView {
    player: Ship
}

impl ShipView {
    pub fn new(engine: &mut Engine) -> ShipView {
        let spritesheet = Sprite::load(&mut engine.renderer, "assets/spaceship.png").unwrap();
        let mut sprites = Vec::with_capacity(9);

        for y in 0..3 {
            for x in 0..3 {
                sprites.push(spritesheet.region(Rectangle {
                    x: SHIP_W * x as f64,
                    y: SHIP_H * y as f64,
                    w: SHIP_W,
                    h: SHIP_H
                }).unwrap());
            }
        }

        ShipView {            
            player: Ship {
                rect: Rectangle {
                    x: 64.0,
                    y: 64.0,
                    w: SHIP_W,
                    h: SHIP_H
                },
                sprites: sprites,
                current: ShipFrame::MidNorm
            }
        }
    }
}

impl View for ShipView {
    fn render(&mut self, context: &mut Engine, elapsed: f64) -> ViewAction {
        if context.events.now.quit || context.events.now.key_escape == Some(true) {
            return ViewAction::Quit;
        }

        let diagonal =
            (context.events.key_up ^ context.events.key_down) &&
            (context.events.key_left ^ context.events.key_right);

        let moved =
            if diagonal { 1.0 / 2.0f64.sqrt() }
            else { 1.0 } * PLAYER_SPEED * elapsed;

        let dx = match (context.events.key_left, context.events.key_right) {
            (true, true) | (false, false) => 0.0,
            (true, false) => -moved,
            (false, true) => moved,
        };

        let dy = match (context.events.key_up, context.events.key_down) {
            (true, true) | (false, false) => 0.0,
            (true, false) => -moved,
            (false, true) => moved,
        };

        self.player.rect.x += dx;
        self.player.rect.y += dy;

        let movable_region = Rectangle {
            x: 0.0,
            y: 0.0,
            w: context.output_size().0 * 0.70,
            h: context.output_size().1,
        };

        self.player.rect = self.player.rect.move_inside(movable_region).unwrap();

        self.player.current =
            if dx == 0.0 && dy < 0.0       { ShipFrame::UpNorm }
            else if dx > 0.0 && dy < 0.0   { ShipFrame::UpFast }
            else if dx < 0.0 && dy < 0.0   { ShipFrame::UpSlow }
            else if dx == 0.0 && dy == 0.0 { ShipFrame::MidNorm }
            else if dx > 0.0 && dy == 0.0  { ShipFrame::MidFast }
            else if dx < 0.0 && dy == 0.0  { ShipFrame::MidSlow }
            else if dx == 0.0 && dy > 0.0  { ShipFrame::DownNorm }
            else if dx > 0.0 && dy > 0.0   { ShipFrame::DownFast }
            else if dx < 0.0 && dy > 0.0   { ShipFrame::DownSlow }
            else { unreachable!() };

        context.renderer.set_draw_color(Color::RGB(0, 0, 0));
        context.renderer.clear();

        context.renderer.set_draw_color(Color::RGB(200, 200, 50));
        context.renderer.fill_rect(self.player.rect.to_sdl());

        context.renderer.copy_sprite(&mut self.player.sprites[self.player.current as usize], self.player.rect);

        ViewAction::None
    }
}

struct Ship {
    rect: Rectangle,
    sprites: Vec<Sprite>,
    current: ShipFrame
}