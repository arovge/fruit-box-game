use bevy::prelude::*;

pub fn title(text: impl Into<String>) -> impl Scene {
    bsn! {
        Text(fix_tab(text))
        TextFont {
            font_size: FontSize::Px(56.)
        }
    }
}

pub fn title2(text: impl Into<String>) -> impl Scene {
    bsn! {
        Text(fix_tab(text))
        TextFont {
            font_size: FontSize::Px(48.)
        }
    }
}

pub fn title3(text: impl Into<String>) -> impl Scene {
    bsn! {
        Text(fix_tab(text))
        TextFont {
            font_size: FontSize::Px(32.),
        }
    }
}

pub fn button(text: impl Into<String>) -> impl Scene {
    bsn! {
        title(text)
        TextColor(Color::BLACK)
        TextLayout {
            justify: Justify::Center,
        }
        Node {
            padding: UiRect::horizontal(Val::Px(100.)),
            border_radius: BorderRadius::all(Val::Px(12.)),
        }
        BackgroundColor(Color::WHITE)
    }
}

// Parlay doesn't seem to support tabs in bevy 0.19:
// https://github.com/linebender/parley/issues/302
// https://github.com/bevyengine/bevy/issues/22955
pub fn fix_tab(str: impl Into<String>) -> String {
    let text = str.into();
    let tab_width = 4;
    let mut result = String::with_capacity(text.len() * 2);
    let mut col = 0;

    for c in text.chars() {
        if c == '\t' {
            // Calculate spaces needed to reach the next tab stop
            let spaces_needed = tab_width - (col % tab_width);
            result.push_str(&" ".repeat(spaces_needed));
            col += spaces_needed;
        } else {
            result.push(c);
            col += 1;
        }
    }
    result
}
