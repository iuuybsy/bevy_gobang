use crate::{consts::*, resources::*};
use bevy::prelude::*;

pub fn color_rgb(u8_rgb: (u8, u8, u8), use_dark: bool) -> Color {
    let factor: f32 = if use_dark { 4.0 } else { 1.0 };
    let r_f32 = (u8_rgb.0 as f32) / RGB_NORMALIZE / factor;
    let g_f32 = (u8_rgb.1 as f32) / RGB_NORMALIZE / factor;
    let b_f32 = (u8_rgb.2 as f32) / RGB_NORMALIZE / factor;
    Color::srgb(r_f32, g_f32, b_f32)
}

pub fn cursor_to_board(cursor: Vec2, plot_info: &Res<PlotInfo>) -> Option<(f32, f32)> {
    let ori_x = 0.5 * plot_info.window_size.x;
    let ori_y = 0.5 * plot_info.window_size.y;

    let delta_x = cursor.x - ori_x;
    let delta_y = cursor.y - ori_y;

    let num_x =
        (delta_x / (plot_info.outter_radius * 2.0) + plot_info.center).round() - plot_info.center;
    let num_y =
        (-delta_y / (plot_info.outter_radius * 2.0) + plot_info.center).round() - plot_info.center;

    if num_x < -plot_info.center
        || num_y < -plot_info.center
        || num_x > plot_info.center
        || num_y > plot_info.center
    {
        None
    } else {
        Some((num_x, num_y))
    }
}
