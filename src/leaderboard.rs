use crate::state::*;
use crate::ui::*;
use bevy::{color::palettes::tailwind::YELLOW_400, prelude::*};

pub struct LeaderboardPlugin;

impl Plugin for LeaderboardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Leaderboard), setup);
    }
}

#[derive(Component, Clone, Copy, Eq, PartialEq)]
pub struct Score {
    pub id: usize,
    pub score: usize,
}

fn setup(mut commands: Commands, scores: Query<&Score>) {
    let new_score = *scores.iter().max_by_key(|s| s.id).unwrap();
    let top_scores: Vec<Score> = {
        let mut top_scores = scores.iter().map(|s| *s).collect::<Vec<Score>>();
        top_scores.sort_by(|a, b| a.score.cmp(&b.score).reverse());
        top_scores.into_iter().take(5).collect()
    };
    let is_new_score_top_score = top_scores.contains(&new_score);
    let display_scores = {
        let mut top_scores = top_scores
            .iter()
            .enumerate()
            .map(|(index, score)| {
                let rank = index + 1;
                row(format!("{rank}\t{}", score.score), score == &new_score)
            })
            .collect::<Vec<_>>();
        if !is_new_score_top_score {
            top_scores.push(row(format!("Score\t{}", new_score.score), true))
        }
        top_scores
    };

    commands.spawn_scene(bsn! {
        #Leaderboard
        DespawnOnExit::<GameState>(GameState::Leaderboard)
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
            title("Leaderboard"),
            title3("Rank\tScore"),
            {display_scores},
            play_button()
        ]
    });
}

fn row(text: impl Into<String>, is_highlighted: bool) -> impl Scene {
    let text_color = if is_highlighted {
        Color::from(YELLOW_400)
    } else {
        Color::WHITE
    };
    bsn! {
        title3(text)
        TextColor(text_color)
    }
}
