use crate::state::*;
use crate::ui::*;
use bevy::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), menu.spawn());
    }
}

fn menu() -> impl Scene {
    bsn! {
        #Menu
        DespawnOnExit::<GameState>(GameState::MainMenu)
        Node {
            display: Display::Flex,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(50.),
            height: Val::Percent(100.),
            width: Val::Percent(100.),
        }
        Children [
            title("MDR Simulator"),
            title2("Welcome Refiner"),
            play_button()
        ]
    }
}
