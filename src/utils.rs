use crate::consts::*;
use bevy::prelude::*;

pub fn color_rgb(u8_rgb: (u8, u8, u8)) -> Color {
    let r_f32 = (u8_rgb.0 as f32) / RGB_NORMALIZE;
    let g_f32 = (u8_rgb.1 as f32) / RGB_NORMALIZE;
    let b_f32 = (u8_rgb.2 as f32) / RGB_NORMALIZE;
    Color::srgb(r_f32, g_f32, b_f32)
}
