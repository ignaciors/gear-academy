use gstd::{exec, msg, prelude::*};
use pebbles_game_io::*;

fn get_random_u32() -> u32 {
    let salt = msg::id();
    let (hash, _num) = exec::random(salt.into()).expect("get_random_u32(): random call failed");
    u32::from_le_bytes([hash[0], hash[1], hash[2], hash[3]])
}

pub fn init() {
    let init: PebblesInit = msg::load().expect("Unable to load PebblesInit");
    let first_player = if get_random_u32() % 2 == 0 {
        Player::User
    } else {
        Player::Program
    };

    let mut state = GameState {
        pebbles_count: init.pebbles_count,
        max_pebbles_per_turn: init.max_pebbles_per_turn,
        pebbles_remaining: init.pebbles_count,
        difficulty: init.difficulty,
        first_player: first_player.clone(),
        winner: None,
    };

    if let Player::Program = first_player {
        let pebbles_to_remove = match init.difficulty {
            DifficultyLevel::Easy => get_random_u32() % init.max_pebbles_per_turn + 1,
            DifficultyLevel::Hard => calculate_best_move(init.pebbles_count, init.max_pebbles_per_turn),
        };
        state.pebbles_remaining -= pebbles_to_remove;
        msg::send!(msg::source(), PebblesEvent::CounterTurn(pebbles_to_remove));
    }

    msg::reply(state, 0).expect("Unable to reply");
}

pub fn handle() {
    let action: PebblesAction = msg::load().expect("Unable to load PebblesAction");
    let mut state: GameState = msg::load().expect("Unable to load GameState");

    match action {
        PebblesAction::Turn(pebbles) => {
            if pebbles < 1 || pebbles > state.max_pebbles_per_turn {
                panic!("Invalid number of pebbles");
            }
            state.pebbles_remaining -= pebbles;
            if state.pebbles_remaining == 0 {
                state.winner = Some(Player::User);
                msg::send!(msg::source(), PebblesEvent::Won(Player::User));
                msg::reply(state, 0).expect("Unable to reply");
                return;
            }
            let pebbles_to_remove = match state.difficulty {
                DifficultyLevel::Easy => get_random_u32() % state.max_pebbles_per_turn + 1,
                DifficultyLevel::Hard => calculate_best_move(state.pebbles_remaining, state.max_pebbles_per_turn),
            };
            state.pebbles_remaining -= pebbles_to_remove;
            if state.pebbles_remaining == 0 {
                state.winner = Some(Player::Program);
                msg::send!(msg::source(), PebblesEvent::Won(Player::Program));
                msg::reply(state, 0).expect("Unable to reply");
                return;
            }
            msg::send!(msg::source(), PebblesEvent::CounterTurn(pebbles_to_remove));
        }
        PebblesAction::GiveUp => {
            state.winner = Some(Player::Program);
            msg::send!(msg::source(), PebblesEvent::Won(Player::Program));
        }
        PebblesAction::Restart { difficulty, pebbles_count, max_pebbles_per_turn } => {
            state = GameState {
                pebbles_count,
                max_pebbles_per_turn,
                pebbles_remaining: pebbles_count,
                difficulty,
                first_player: if get_random_u32() % 2 == 0 { Player::User } else { Player::Program },
                winner: None,
            };
            if let Player::Program = state.first_player {
                let pebbles_to_remove = match state.difficulty {
                    DifficultyLevel::Easy => get_random_u32() % state.max_pebbles_per_turn + 1,
                    DifficultyLevel::Hard => calculate_best_move(state.pebbles_count, state.max_pebbles_per_turn),
                };
                state.pebbles_remaining -= pebbles_to_remove;
                msg::send!(msg::source(), PebblesEvent::CounterTurn(pebbles_to_remove));
            }
        }
    }

    msg::reply(state, 0).expect("Unable to reply");
}

pub fn state() {
    let state: GameState = msg::load().expect("Unable to load GameState");
    msg::reply(state, 0).expect("Unable to reply");
}

fn calculate_best_move(pebbles_remaining: u32, max_pebbles_per_turn: u32) -> u32 {
    // Implement a strategy to calculate the best move for hard difficulty.
    // For simplicity, you can use a basic strategy here.
    (pebbles_remaining - 1) % (max_pebbles_per_turn + 1) + 1
}
