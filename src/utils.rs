use crate::consts::*;
use bevy::prelude::*;

pub fn color_rgb(u8_rgb: (u8, u8, u8), use_dark: bool) -> Color {
    let factor: f32 = if use_dark { 2.0 } else { 1.0 };
    let r_f32 = (u8_rgb.0 as f32) / RGB_NORMALIZE / factor;
    let g_f32 = (u8_rgb.1 as f32) / RGB_NORMALIZE / factor;
    let b_f32 = (u8_rgb.2 as f32) / RGB_NORMALIZE / factor;
    Color::srgb(r_f32, g_f32, b_f32)
}
