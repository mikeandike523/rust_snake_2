use std::time::Duration;

use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::KeyboardState;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Scancode;
// Basic game loop template courtesy of https://sunjay.dev/learn-game-dev/refactor-traditional-game-loop.html
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

use rand::Rng;

use crate::appstate::AppState;
use crate::directions::*;
use crate::point::{IPoint2, Point2};

const FPS: u32 = 60;
const INIT_SIZE: (u32, u32) = (640, 480);
const GRID_SIZE: (i32, i32) = (41, 41);
const INIT_TRAVEL_DIRECTION: IPoint2 = EAST;
const SNAKE_INIT_POSITION: (i32, i32) = (20, 20);
const FRAMES_PER_UPDATE: i32 = 4;

fn isKeyDown(event_pump: &EventPump, code: Scancode) -> bool {
    KeyboardState::new(event_pump).is_scancode_pressed(code)
}

fn update<R: Rng>(event_pump: &EventPump, appstate: &mut AppState, rng: &mut R) -> bool {
    if isKeyDown(event_pump, Scancode::Left) || isKeyDown(event_pump, Scancode::A) {
        if appstate.last_travel_direction != EAST {
            appstate.travel_direction = WEST;
        }
    }
    if isKeyDown(event_pump, Scancode::Right) || isKeyDown(event_pump, Scancode::D) {
        if appstate.last_travel_direction != WEST {
            appstate.travel_direction = EAST;
        }
    }
    if isKeyDown(event_pump, Scancode::Up) || isKeyDown(event_pump, Scancode::W) {
        if appstate.last_travel_direction != SOUTH {
            appstate.travel_direction = NORTH;
        }
    }
    if isKeyDown(event_pump, Scancode::Down) || isKeyDown(event_pump, Scancode::S) {
        if appstate.last_travel_direction != NORTH {
            appstate.travel_direction = SOUTH;
        }
    }

    if appstate.update_tick < FRAMES_PER_UPDATE {
        appstate.update_tick += 1;
        return true;
    }

    appstate.last_travel_direction = appstate.travel_direction;

    appstate.update_tick = 0;

    appstate.grid.clear();

    for point in appstate.snake.body.iter() {
        appstate.grid.set_pixel(point.x, point.y, true);
    }


    let head = *appstate.snake.body.back().expect("Snake has no body.");

    let mut next_head_position = IPoint2::sum(head, appstate.travel_direction);

    next_head_position.warp(appstate.grid.rows, appstate.grid.cols);

    for point in appstate.snake.body.iter() {
        if *point == next_head_position {
            return false;
        }
    }

    if *appstate.snake.body.back().expect("Snake has no body.") == appstate.food {
        appstate.food = IPoint2::new(rng.gen_range(0i32..appstate.grid.cols),
                                     rng.gen_range(0i32..appstate.grid.rows),
        );
        appstate.snake.grow_in_direction(&appstate.travel_direction);
        appstate.score += 1;
    } else {
        appstate.snake.move_in_direction(&appstate.travel_direction);
    }

    appstate.snake.warp(appstate.grid.rows, appstate.grid.cols);

    true
}

fn render(canvas: &mut WindowCanvas, appstate: &mut AppState) {
    canvas.set_draw_color(Color::RGB(0, 0, 255));
    canvas.clear();
    appstate.grid.render_to(canvas, appstate);
    appstate.grid.render_pixel(canvas, appstate, appstate.food.x, appstate.food.y, Color::RGB(0, 255, 0));
    canvas.present();
}

pub fn run() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("game tutorial", INIT_SIZE.0 as u32, INIT_SIZE.1 as u32)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");

    let mut appstate = AppState::new(
        INIT_SIZE.0 as i32,
        INIT_SIZE.1 as i32,
        GRID_SIZE.0,
        GRID_SIZE.1,
        INIT_TRAVEL_DIRECTION,
        SNAKE_INIT_POSITION,
    );

    let mut rng = rand::thread_rng();

    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;
    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                _ => {}
            }
        }

        let has_not_lost_yet = update(&event_pump, &mut appstate, &mut rng);

        if !has_not_lost_yet {
            println!("You lose. Score: {}", appstate.score);
            return Ok(());
        }

        // Render
        render(&mut canvas, &mut appstate);

        // Time management!
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FPS));
    }

    Ok(())
}