use crate::consts::*;
use crate::resources::*;

pub fn is_same_color(game_state: &GameState, x1: u8, y1: u8, x2: u8, y2: u8) -> bool {
    if x1 >= BOARD_LOGICAL_LEN
        || y1 >= BOARD_LOGICAL_LEN
        || x2 >= BOARD_LOGICAL_LEN
        || y2 >= BOARD_LOGICAL_LEN
    {
        return false;
    }

    if x1 == x2 && y1 == y2 {
        return true;
    }

    let pos1 = (x1, y1);
    let pos2 = (x2, y2);

    return (game_state.red_set.contains(&pos1) && game_state.red_set.contains(&pos2))
        || (game_state.yellow_set.contains(&pos1) && game_state.yellow_set.contains(&pos2));
}
