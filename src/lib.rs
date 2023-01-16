use livesplit_core::hotkey::Hotkey;
use livesplit_core::HotkeyConfig;
use livesplit_core::Timer;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Display;
use std::sync::atomic::AtomicU64;

pub const CURSIVE_THEME: &str = include_str!("cursive-theme.toml");
pub const DEFAULT_LAYOUT: &[u8; 6413] = include_bytes!("default_layout.ls1l");
pub const DEFAULT_SPLITS: &[u8] = include_bytes!("default_splits.lss");

#[derive(Serialize, Deserialize)]
#[serde(default)]
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
    pub toggle_timing_method: char,
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
            toggle_timing_method: '/',
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
        if chr == self.toggle_timing_method {
            timer.toggle_timing_method();
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub local_hotkeys: LocalHotkeys,
    pub global_hotkeys: HotkeyConfig,
}

pub enum ConfigValue {
    Dummy,
    LocalHotkey(char),
    GlobalHotkey(Option<Hotkey>),
}

impl Display for ConfigValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match self {
            Self::Dummy => "".into(),
            Self::LocalHotkey(x) => x.to_string(),
            Self::GlobalHotkey(x) => match x {
                Some(x) => x.to_string(),
                None => "unbound".into(),
            },
        };

        write!(f, "{result}")
    }
}

impl From<char> for ConfigValue {
    fn from(x: char) -> Self {
        ConfigValue::LocalHotkey(x)
    }
}

impl From<Option<Hotkey>> for ConfigValue {
    fn from(x: Option<Hotkey>) -> Self {
        ConfigValue::GlobalHotkey(x)
    }
}

impl TryFrom<ConfigValue> for char {
    type Error = &'static str;
    fn try_from(value: ConfigValue) -> Result<Self, Self::Error> {
        match value {
            ConfigValue::LocalHotkey(x) => Ok(x),
            _ => Err("conversion failed"),
        }
    }
}

impl TryFrom<ConfigValue> for Option<Hotkey> {
    type Error = &'static str;
    fn try_from(value: ConfigValue) -> Result<Self, Self::Error> {
        match value {
            ConfigValue::GlobalHotkey(x) => Ok(x),
            _ => Err("conversion failed"),
        }
    }
}

pub struct ConfigField {
    pub name: String, // not unique
    pub value: ConfigValue,
}

impl ConfigField {
    pub fn new(name: String, value: ConfigValue) -> Self {
        ConfigField { name, value }
    }
}

impl Config {
    // TODO: this really should just be done with rust's equivilant of an X macro
    pub fn list(&self) -> Vec<ConfigField> {
        vec![
            ConfigField::new("-- Local Hotkeys --".into(), ConfigValue::Dummy),
            ConfigField::new(
                "Split or Start".into(),
                self.local_hotkeys.split_or_start.into(),
            ),
            ConfigField::new(
                "Reset without Saving".into(),
                self.local_hotkeys.reset_nosave.into(),
            ),
            ConfigField::new("Reset".into(), self.local_hotkeys.reset.into()),
            ConfigField::new(
                "Undo Split".into(),
                self.local_hotkeys.undo_split.into(),
            ),
            ConfigField::new(
                "Skip Split".into(),
                self.local_hotkeys.skip_split.into(),
            ),
            ConfigField::new(
                "Toggle Pause".into(),
                self.local_hotkeys.toggle_pause.into(),
            ),
            ConfigField::new(
                "Undo all Pauses".into(),
                self.local_hotkeys.undo_all_pauses.into(),
            ),
            ConfigField::new(
                "Previous Comparison".into(),
                self.local_hotkeys.previous_comparison.into(),
            ),
            ConfigField::new(
                "Next Comparison".into(),
                self.local_hotkeys.next_comparison.into(),
            ),
            ConfigField::new(
                "Toogle Timing Method".into(),
                self.local_hotkeys.toggle_timing_method.into(),
            ),
            ConfigField::new("-- Global Hotkeys --".into(), ConfigValue::Dummy),
            ConfigField::new(
                "Split or Start".into(),
                self.global_hotkeys.split.into(),
            ),
            ConfigField::new("Reset".into(), self.global_hotkeys.reset.into()),
            ConfigField::new(
                "Undo Split".into(),
                self.global_hotkeys.undo.into(),
            ),
            ConfigField::new(
                "Skip Split".into(),
                self.global_hotkeys.skip.into(),
            ),
            ConfigField::new(
                "Toggle Pause".into(),
                self.global_hotkeys.pause.into(),
            ),
            ConfigField::new(
                "Undo All Pauses".into(),
                self.global_hotkeys.undo_all_pauses.into(),
            ),
            ConfigField::new(
                "Previous Comparison".into(),
                self.global_hotkeys.previous_comparison.into(),
            ),
            ConfigField::new(
                "Next Comparison".into(),
                self.global_hotkeys.next_comparison.into(),
            ),
            ConfigField::new(
                "Toggle Timing Method".into(),
                self.global_hotkeys.toggle_timing_method.into(),
            ),
        ]
    }

    pub fn set(&mut self, idx: usize, value: ConfigValue) {
        match idx {
            0 => (),
            1 => self.local_hotkeys.split_or_start = value.try_into().unwrap(),
            2 => self.local_hotkeys.reset_nosave = value.try_into().unwrap(),
            3 => self.local_hotkeys.reset = value.try_into().unwrap(),
            4 => self.local_hotkeys.undo_split = value.try_into().unwrap(),
            5 => self.local_hotkeys.skip_split = value.try_into().unwrap(),
            6 => self.local_hotkeys.toggle_pause = value.try_into().unwrap(),
            7 => self.local_hotkeys.undo_all_pauses = value.try_into().unwrap(),
            8 => {
                self.local_hotkeys.previous_comparison =
                    value.try_into().unwrap()
            }
            9 => self.local_hotkeys.next_comparison = value.try_into().unwrap(),
            10 => {
                self.local_hotkeys.toggle_timing_method =
                    value.try_into().unwrap()
            }
            11 => (),
            12 => self.global_hotkeys.split = value.try_into().unwrap(),
            13 => self.global_hotkeys.reset = value.try_into().unwrap(),
            14 => self.global_hotkeys.undo = value.try_into().unwrap(),
            15 => self.global_hotkeys.skip = value.try_into().unwrap(),
            16 => self.global_hotkeys.pause = value.try_into().unwrap(),
            17 => {
                self.global_hotkeys.undo_all_pauses = value.try_into().unwrap()
            }
            18 => {
                self.global_hotkeys.previous_comparison =
                    value.try_into().unwrap()
            }
            19 => {
                self.global_hotkeys.next_comparison = value.try_into().unwrap()
            }
            20 => {
                self.global_hotkeys.toggle_timing_method =
                    value.try_into().unwrap()
            }
            _ => panic!("invalid index"),
        }
    }
}

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct UniqueID(u64);

impl UniqueID {
    pub fn new() -> Self {
        static ID_COUNT: AtomicU64 = AtomicU64::new(0);
        let id = ID_COUNT.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

        if id == u64::MAX {
            panic!("id overflow");
        };

        UniqueID(id)
    }
}

impl Default for UniqueID {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for UniqueID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UniqueID({})", self.0)
    }
}
