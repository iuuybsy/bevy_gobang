use bevy::prelude::*;
use std::collections::HashSet;

#[derive(Resource, Default)]
pub struct GameState {
    num: u32,
    move_stack: Vec<(u32, u32)>,
    pub red_set: HashSet<(u32, u32)>,
    pub yellow_set: HashSet<(u32, u32)>,
}

impl GameState {
    pub fn is_red_turn(&self) -> bool {
        self.num % 2 == 0
    }

    pub fn take_a_move(&mut self, new_move_cord: (u32, u32)) {
        if self.red_set.contains(&new_move_cord) || self.yellow_set.contains(&new_move_cord) {
            return;
        }

        self.move_stack.push(new_move_cord.clone());
        if self.is_red_turn() {
            self.red_set.insert(new_move_cord);
        } else {
            self.yellow_set.insert(new_move_cord);
        }
        self.num += 1;
    }

    pub fn regret(&mut self) {
        if self.num > 0 {
            self.num -= 1;
            let regreted_move = self.move_stack.pop().unwrap();
            self.red_set.remove(&regreted_move);
            self.yellow_set.remove(&regreted_move);
        }
    }
}

#[derive(Resource, Default)]
pub struct PlotInfo {
    pub outter_radius: f32,
    pub center: f32,
    pub window_size: Vec2,
}
