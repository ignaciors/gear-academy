use gstd::prelude::*;
use pebbles_game_io::*;

#[test]
fn test_game_initialization() {
    let init = PebblesInit {
        difficulty: DifficultyLevel::Easy,
        pebbles_count: 15,
        max_pebbles_per_turn: 2,
    };
    let state: GameState = init_game(init);
    assert_eq!(state.pebbles_count, 15);
    assert_eq!(state.max_pebbles_per_turn, 2);
    assert!(state.pebbles_remaining <= 15);
}

#[test]
fn test_user_wins() {
    // Simulate a game where the user wins.
}

#[test]
fn test_program_wins() {
    // Simulate a game where the program wins.
}

fn init_game(init: PebblesInit) -> GameState {
    // Helper function to initialize the game and return the state.
}
