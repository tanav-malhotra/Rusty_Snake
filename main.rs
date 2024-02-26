#![allow(non_snake_case)]
#![windows_subsystem = "windows"] // Prevent the console from popping up
extern crate rand;
extern crate piston_window;
mod draw;
mod snake;
mod game;

use piston_window::*;
use piston_window::types::{Color};

use crate::game::Game;
use crate::snake::Direction;
use crate::draw::to_coord_u32;

const BACKGROUND_COLOR: Color = [0.368627, 0.839216, 0.337255, 1.0]; // green


fn main() {
    let (width, height) = (30, 30);

    let mut window = create_window("Rusty_Snake - Tanav Malhotra", width, height);

    let mut game = Game::new(width, height);

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }
        window.draw_2d(&event, |c, g, _| {
            clear(BACKGROUND_COLOR, g);
            game.draw(&c, g);
        });

        event.update(|arg| {
            game.update(arg.dt);
            if game.get_snake_dir() != Direction::Waiting {
                window.set_title(format!("Score: {}", game.get_score()));
            } else {
                if game.get_high_score() == 0 {
                    window.set_title(String::from("Rusty_Snake - Tanav Malhotra"));
                } else {
                    window.set_title(format!("High Score: {}", game.get_high_score()));
                }
            }
            window.set_size([to_coord_u32(width), to_coord_u32(height)]);
        });
    }
}

fn create_window(title: &str, width: i32, height: i32) -> PistonWindow {
    return WindowSettings::new(
        title, [to_coord_u32(width), to_coord_u32(height)])
        .exit_on_esc(true)
        .fullscreen(false)
        .resizable(false)
        .decorated(true)
        .build()
        .unwrap();
}