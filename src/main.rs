extern crate piston_window;

use piston_window::*;

struct Rectangle {
    x: u64,
    y: u64,
    width: u64,
    height: u64,
}

struct Player {
    rectangle: Rectangle,
}

struct World {
    objects: Vec<Rectangle>,
}

struct State {
    player: Player,
    world: World,
}

enum GameState {
    LoadingScreen,
    Playing(State),
    Finished,
}

fn make_initial_state() -> GameState {
    GameState::Playing(State {
        player: Player {
            rectangle: Rectangle {
                x: 0,
                y: 0,
                width: 5,
                height: 20,
            },
        },
        world: World { objects: vec![] },
    })
}

fn tick(state: GameState) -> GameState {
    state
}

fn draw_state(state: GameState) {}

fn main() {
    let mut window: PistonWindow =
    WindowSettings::new("Hello World!", [512; 2])
            .build().unwrap();
    // let dim = WindowDim::Windowed {
    //     width: 960,
    //     height: 540,
    // };

    // let state = make_initial_state();
    // draw_state(state);

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _| {
            clear([0.5, 0.5, 0.5, 1.0], g);
            rectangle([1.0, 0.0, 0.0, 1.0], // red
                      [0.0, 0.0, 100.0, 100.0], // rectangle
                      c.transform, g);
        });
    }
}

