use super::splits;
use cursive::traits::Nameable;
use cursive::views::{LinearLayout, PaddedView, RadioButton, RadioGroup};
use cursive::Cursive;
use livesplit_core::TimingMethod;

use crate::global_state::GlobalState;

pub fn build_timing_methods(_s: &mut Cursive) -> LinearLayout {
    let mut radio_group =
        RadioGroup::new().on_change(|s, _v| change_timing_method(s));

    let real_time_button = radio_group
        .button_str("Real Time")
        .with_name("real_time_button");
    let game_time_button = radio_group
        .button_str("Game Time")
        .with_name("game_time_button");

    let padded_real_time_button =
        PaddedView::lrtb(0, 1, 0, 0, real_time_button);

    LinearLayout::horizontal()
        .child(padded_real_time_button)
        .child(game_time_button)
}

fn change_timing_method(s: &mut Cursive) {
    let real_time = s
        .find_name::<RadioButton<String>>("real_time_button")
        .unwrap();
    let game_time = s
        .find_name::<RadioButton<String>>("game_time_button")
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
