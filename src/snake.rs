use std::collections::VecDeque;

use crate::point::IPoint2;

pub struct Snake {
    pub body: VecDeque<IPoint2>,
}

impl Snake {
    pub fn new(x: i32, y: i32) -> Snake {
        let mut body: VecDeque<IPoint2> = VecDeque::new();
        let init_position = IPoint2::new(x, y);
        body.push_back(init_position);
        Snake {
            body
        }
    }

    pub fn grow_in_direction(&mut self, direction: &IPoint2) {
        let head = self.body.back().expect("The snake had no body. This should not occur.");
        let new_head = IPoint2::sum(*head, *direction);
        self.body.push_back(new_head);
    }

    pub fn move_in_direction(&mut self, direction: &IPoint2) {
        self.grow_in_direction(direction);
        self.body.pop_front().expect("The snake had no body. This should not occur.");
    }

    pub fn warp(&mut self, w: i32, h: i32) {
        for item in self.body.iter_mut() {
            item.warp(w, h);
        }
    }
}