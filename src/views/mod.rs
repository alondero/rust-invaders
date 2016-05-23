use engine::{Engine, View, ViewAction};
use engine::data::Rectangle;
use sdl2::pixels::Color;

const PLAYER_SPEED: f64 = 360.0;

pub struct ShipView {
    player: Ship
}

impl ShipView {
    pub fn new(engine: &mut Engine) -> ShipView {
        ShipView {
            player: Ship {
                rect: Rectangle {
                    x: 64.0,
                    y: 64.0,
                    w: 32.0,
                    h: 32.0,
                }
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

        context.renderer.set_draw_color(Color::RGB(0, 0, 0));
        context.renderer.clear();

        context.renderer.set_draw_color(Color::RGB(200, 200, 50));
        context.renderer.fill_rect(self.player.rect.to_sdl().unwrap());

        ViewAction::None
    }
}

struct Ship {
    rect: Rectangle
}