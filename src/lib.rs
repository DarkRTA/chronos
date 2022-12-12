use livesplit_core::HotkeyConfig;
use livesplit_core::Timer;
use serde::Deserialize;
use serde::Serialize;

pub mod terminal;
pub mod renderer;

pub const DEFAULT_LAYOUT: &[u8; 6413] = include_bytes!("default_layout.ls1l");

#[derive(Serialize, Deserialize)]
pub struct LocalHotkeys {
    pub split_or_start: char,
    pub reset_nosave: char,
    pub reset: char,
    pub undo_split: char,
    pub skip_split: char,
    pub toggle_pause: char,
    pub undo_all_pauses: char,
    pub previous_comparison: char,
    pub next_comparison: char,
}

impl Default for LocalHotkeys {
    fn default() -> Self {
        LocalHotkeys {
            split_or_start: ' ',
            reset_nosave: 'X',
            reset: 'x',
            undo_split: 'c',
            skip_split: 'v',
            toggle_pause: 'b',
            undo_all_pauses: 'n',
            previous_comparison: ',',
            next_comparison: '.',
        }
    }
}

impl LocalHotkeys {
    pub fn do_key(&self, chr: char, timer: &mut Timer) {
        // why....
        if chr == self.split_or_start {
            timer.split_or_start();
        }
        if chr == self.reset_nosave {
            timer.reset(false);
        }
        if chr == self.reset {
            timer.reset(true);
        }
        if chr == self.undo_split {
            timer.undo_split();
        }
        if chr == self.skip_split {
            timer.skip_split();
        }
        if chr == self.toggle_pause {
            timer.toggle_pause();
        }
        if chr == self.undo_all_pauses {
            timer.undo_all_pauses();
        }
        if chr == self.previous_comparison {
            timer.switch_to_previous_comparison();
        }
        if chr == self.next_comparison {
            timer.switch_to_next_comparison();
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub local_hotkeys: LocalHotkeys,
    pub global_hotkeys: HotkeyConfig,
}
