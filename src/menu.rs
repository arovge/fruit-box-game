use crate::state::*;
use bevy::{ecs::spawn::SpawnWith, prelude::*};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), setup);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        DespawnOnExit(GameState::MainMenu),
        Node {
            display: Display::Flex,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(50.),
            height: Val::Percent(100.),
            width: Val::Percent(100.),
            ..default()
        },
        Children::spawn((
            Spawn((
                Text::new("MDR Simulator"),
                TextFont {
                    font_size: FontSize::Px(56.),
                    ..default()
                },
            )),
            Spawn((
                Text::new("Welcome Refiner"),
                TextFont {
                    font_size: FontSize::Px(48.),
                    ..default()
                },
            )),
            SpawnWith(|parent: &mut ChildSpawner| {
                parent
                    .spawn((
                        Text::new("Play"),
                        TextColor(Color::BLACK),
                        TextFont {
                            font_size: FontSize::Px(48.),
                            ..default()
                        },
                        TextLayout {
                            justify: Justify::Center,
                            ..default()
                        },
                        Node {
                            padding: UiRect::horizontal(Val::Px(100.)),
                            border_radius: BorderRadius::all(Val::Px(12.)),
                            ..default()
                        },
                        BackgroundColor(Color::WHITE),
                    ))
                    .observe(play);
            }),
        )),
    ));
}
