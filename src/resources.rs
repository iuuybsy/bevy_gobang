use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct ClickNum {
    pub num: u32,
}

#[derive(Resource, Default)]
pub struct HoverCell(pub Option<(f32, f32)>);
