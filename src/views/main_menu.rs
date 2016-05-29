use engine::{Engine, View, ViewAction};
use engine::data::Rectangle;
use engine::graphics::{Sprite, CopySprite};
use sdl2::pixels::Color;

pub struct MainMenuView {
    actions: Vec<Action>,
    selected: i8,
}

impl MainMenuView {
    pub fn new(engine: &mut Engine) -> MainMenuView {
        MainMenuView {
            actions: vec![
                Action::new(engine, "New Game", Box::new(|engine| {ViewAction::ChangeView(Box::new(::views::game::ShipView::new(engine)))})),
                Action::new(engine, "Quit to Desktop", Box::new(|_| {ViewAction::Quit}))
            ],
            selected: 0
        }
    }
}

impl View for MainMenuView {
    fn render(&mut self, engine: &mut Engine, elapsed: f64) -> ViewAction {
        if engine.events.now.quit || engine.events.now.key_escape == Some(true) {
            return ViewAction::Quit;
        }

        if engine.events.now.key_space == Some(true) {
            return (self.actions[self.selected as usize].func)(engine);
        }

        if engine.events.now.key_up == Some(true) {
            self.selected -= 1;
            if self.selected < 0 {
                self.selected = self.actions.len() as i8 - 1;
            }
        } else if engine.events.now.key_down == Some(true) {
            self.selected += 1;
            if self.selected >= self.actions.len() as i8 {
                self.selected = 0;
            }
        }

        engine.renderer.set_draw_color(Color::RGB(0, 0, 0));
        engine.renderer.clear();

        let (win_w, win_h) = engine.output_size();
        let label_h = 50.0;
        let border_width = 3.0;
        let box_w = 360.0;
        let box_h = self.actions.len() as f64 * label_h;
        let margin_h = 10.0;

        engine.renderer.set_draw_color(Color::RGB(70, 15, 70));
        engine.renderer.fill_rect(Rectangle {
            w: box_w + border_width * 2.0,
            h: box_h + border_width * 2.0 + margin_h * 2.0,
            x: (win_w - box_w) / 2.0 - border_width,
            y: (win_h - box_h) / 2.0 - margin_h - border_width,
        }.to_sdl());

        // Render the colored box which holds the labels
        engine.renderer.set_draw_color(Color::RGB(140, 30, 140));
        engine.renderer.fill_rect(Rectangle {
            w: box_w,
            h: box_h + margin_h * 2.0,
            x: (win_w - box_w) / 2.0,
            y: (win_h - box_h) / 2.0 - margin_h,
        }.to_sdl());

        for (i, action) in self.actions.iter().enumerate() {
            if self.selected as usize == i {
                let (w, h) = action.hover_sprite.size();
                engine.renderer.copy_sprite(&action.idle_sprite, Rectangle {
                    w: w,
                    h: h,
                    x: (win_w - w) / 2.0,
                    y: (win_h - box_h + label_h - h) / 2.0 + label_h * i as f64,
                });
            } else {
                let (w, h) = action.idle_sprite.size();
                engine.renderer.copy_sprite(&action.idle_sprite, Rectangle {
                    w: w,
                    h: h,
                    x: (win_w - w) / 2.0,
                    y: (win_h - box_h + label_h - h) / 2.0 + label_h * i as f64,
                });
            }
        }

        ViewAction::None
    }
}

struct Action {
    func: Box<Fn(&mut Engine) -> ViewAction>,

    idle_sprite: Sprite,
    hover_sprite: Sprite,
}

impl Action {
    fn new(engine: &mut Engine, label: &'static str, func: Box<Fn(&mut Engine) -> ViewAction>) -> Action {
        Action {
            func: func,
            idle_sprite: engine.ttf_str_sprite(label, "assets/belligerent.ttf", 32, Color::RGB(220, 220, 220)).unwrap(),
            hover_sprite: engine.ttf_str_sprite(label, "assets/belligerent.ttf", 38, Color::RGB(255, 255, 255)).unwrap(),
        }
    }
}