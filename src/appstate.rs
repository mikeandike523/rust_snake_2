use crate::directions::*;
use crate::grid::Grid;
use crate::point::{IPoint2, Point2};
use crate::snake::Snake;

pub struct AppState {
    pub w: i32,
    pub h: i32,
    pub grid: Grid,
    pub travel_direction: IPoint2,
    pub snake: Snake,
    pub food: IPoint2,
    pub score: i32,
    pub update_tick: i32,
    pub last_travel_direction: IPoint2,
}

impl AppState {
    pub fn new(init_w: i32, init_h: i32, grid_rows: i32, grid_cols: i32, init_travel_direction: IPoint2, init_position: (i32, i32)) -> AppState {
        AppState {
            w: init_w,
            h: init_h,
            grid: Grid::new(grid_rows, grid_cols),
            travel_direction: init_travel_direction,
            snake: Snake::new(init_position.0, init_position.1),
            food: IPoint2::new(init_position.0, init_position.1),
            score: 0,
            update_tick: 0,
            last_travel_direction: init_travel_direction,
        }
    }
}