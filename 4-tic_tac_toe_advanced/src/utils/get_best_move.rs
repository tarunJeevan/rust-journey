use super::get_winner;
use crate::models::Player;

pub fn get_best_move(board: &[char; 9], comp: &Player) -> usize {
    let mut best_score = i32::MIN;
    let mut move_index = 0;

    let mut board = *board;

    for i in 0..9 {
        if board[i] == ' ' {
            board[i] = comp.char();

            let score = minimax(&mut board, comp, false);

            board[i] = ' ';

            if score > best_score {
                best_score = score;
                move_index = i;
            }
        }
    }

    move_index
}

fn minimax(board: &mut [char; 9], comp: &Player, comp_turn: bool) -> i32 {
    let human = comp.other();

    let winner = get_winner(board);

    if winner == Some(comp.clone()) {
        return 1;
    }
    if winner == Some(human.clone()) {
        return -1;
    }
    if !board.contains(&' ') {
        return 0;
    }

    if comp_turn {
        let mut best_score = i32::MIN;

        for i in 0..9 {
            if board[i] == ' ' {
                board[i] = comp.char();

                let score = minimax(board, comp, false);

                board[i] = ' ';

                best_score = best_score.max(score);
            }
        }
        best_score
    } else {
        let mut best_score = i32::MAX;

        for i in 0..9 {
            if board[i] == ' ' {
                board[i] = human.char();

                let score = minimax(board, comp, true);

                board[i] = ' ';

                best_score = best_score.min(score);
            }
        }
        best_score
    }
}
