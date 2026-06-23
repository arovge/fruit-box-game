use crate::ui::*;
use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    #[default]
    MainMenu,
    Playing,
    Leaderboard,
}

pub fn play(_: On<Pointer<Click>>, mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Playing);
}

pub fn play_button() -> impl Scene {
    bsn! {
        button("Play")
        on(play)
    }
}
