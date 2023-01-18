use super::splits;
use chronos::UniqueID;
use cursive::traits::{Nameable, Resizable};
use cursive::views::{
    LinearLayout, PaddedView, RadioButton, RadioGroup, ResizedView,
};

use cursive::view::SizeConstraint::{Fixed, Full};
use cursive::Cursive;
use livesplit_core::TimingMethod;

use crate::global_state::GlobalState;

pub fn build_timing_methods(_s: &mut Cursive) -> ResizedView<LinearLayout> {
    let mut radio_group = RadioGroup::new();

    let real_time_editor_id = UniqueID::new();
    let real_time_button = radio_group
        .button_str("Real Time")
        .with_name(real_time_editor_id.to_string());

    let game_time_editor_id = UniqueID::new();
    let game_time_button = radio_group
        .button_str("Game Time")
        .with_name(game_time_editor_id.to_string());

    radio_group.set_on_change(move |s, _v| {
        change_timing_method(
            s,
            &real_time_editor_id.to_string(),
            &game_time_editor_id.to_string(),
        )
    });

    let padded_real_time_button =
        PaddedView::lrtb(0, 1, 0, 0, real_time_button);

    LinearLayout::horizontal()
        .child(padded_real_time_button)
        .child(game_time_button)
        .resized(Full, Fixed(1))
}

fn change_timing_method(
    s: &mut Cursive,
    real_time_editor_id: &str,
    game_time_editor_id: &str,
) {
    let real_time = s
        .find_name::<RadioButton<String>>(real_time_editor_id)
        .unwrap();

    let game_time = s
        .find_name::<RadioButton<String>>(game_time_editor_id)
        .unwrap();

    let globals: &mut GlobalState = s.user_data().unwrap();
    if real_time.is_selected() {
        globals
            .splits_editor
            .select_timing_method(TimingMethod::RealTime);
    } else if game_time.is_selected() {
        globals
            .splits_editor
            .select_timing_method(TimingMethod::GameTime);
    }
    splits::refresh_splits(s);
}
