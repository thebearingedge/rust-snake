mod draw;
mod game;
mod snake;

use crate::draw::to_coord_u32;
use crate::game::Game;
use piston_window::types::Color;
use piston_window::*;

const BACKGROUND: Color = [1.0, 1.0, 1.0, 1.0];

fn main() {
    let (width, height) = (20, 20);

    let mut window: PistonWindow =
        WindowSettings::new("Snake", [to_coord_u32(width), to_coord_u32(height)])
            .exit_on_esc(true)
            .build()
            .unwrap();

    let mut game = Game::new(width, height);

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }
        window.draw_2d(&event, |ctx, buffer| {
            clear(BACKGROUND, buffer);
            game.draw(&ctx, buffer);
        });
        event.update(|arg| game.update(arg.dt));
    }
}
