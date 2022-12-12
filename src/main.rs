use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::BufWriter;
use std::io::Cursor;
use std::time::Duration;

use clap::Parser;
use crossterm::event;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use livesplit_core::layout::LayoutSettings;
use livesplit_core::run::parser;
use livesplit_core::HotkeyConfig;
use livesplit_core::HotkeySystem;
use livesplit_core::Layout;
use livesplit_core::Timer;
use livesplit_core::auto_splitting;
use livesplit_core::run::saver::livesplit;
use serde::Deserialize;
use serde::Serialize;

mod renderer;
mod terminal;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    /// WASM-based auto splitter to use
    auto_splitter: Option<String>,
    #[arg(short, long)]
    /// config file to use
    config: Option<String>,
    #[arg(short, long)]
    /// layout file to use
    layout: Option<String>,
    #[arg(short, long)]
    /// splits file to use
    splits: String,
}

#[derive(Serialize, Deserialize)]
pub struct LocalHotkeys {
    split_or_start: char,
    reset_nosave: char,
    reset: char,
    undo_split: char,
    skip_split: char,
    toggle_pause: char,
    undo_all_pauses: char,
    previous_comparison: char,
    next_comparison: char,
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
struct Config {
    local_hotkeys: LocalHotkeys,
    global_hotkeys: HotkeyConfig,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let splits_file = fs::read(&args.splits)?;
    let run =
        parser::composite::parse(&splits_file, Some((&args.splits).into()), true)?
            .run;
    let stimer = Timer::new(run)?.into_shared();

    let mut layout = match args.layout {
        Some(s) => {
            let settings = LayoutSettings::from_json(File::open(s)?)?;
            Layout::from_settings(settings)
        }
        None => {
            let layout = include_bytes!("default_layout.ls1l");
            let cursor = Cursor::new(layout);
            let settings = LayoutSettings::from_json(cursor)?;
            Layout::from_settings(settings)
        }
    };

    let config = match &args.config {
        Some(path) => toml::de::from_slice(&fs::read(path)?)?,
        None => Config::default()
    };

    // we actually do nothing with this but will need to hold onto it as the
    // mere act of creating one will start a thread that needs to be kept alive
    let _auto_splitter = match &args.auto_splitter {
        Some(path) => {
            let runtime = auto_splitting::Runtime::new(stimer.clone());
            runtime.load_script_blocking(path.into())?;
            Some(runtime)
        },
        None => None,
    };

    let mut hotkey_system = HotkeySystem::with_config(stimer.clone(), config.global_hotkeys)?;
    let mut term = terminal::Terminal::new();
    loop {
        let timer = stimer.read().unwrap();
        let state = layout.state(&timer.snapshot());
        renderer::render(&mut term, state);
        drop(timer); // release the lock so we can obtain a write lock later

        if event::poll(Duration::from_secs_f64(0.02))? {
            if let Event::Key(k) = event::read()? {
                let mut timer = stimer.write().unwrap();
                match k.code {
                    KeyCode::Esc => break,
                    KeyCode::F(1) => hotkey_system.activate()?,
                    KeyCode::F(2) => hotkey_system.deactivate()?,
                    KeyCode::F(3) => {
                        let file = File::create(&args.splits)?;
                        let writer = BufWriter::new(file);
                        livesplit::save_run(timer.snapshot().run(), livesplit::IoWrite(writer))?;
                    },
                    KeyCode::Char(chr) => {
                        config.local_hotkeys.do_key(chr, &mut timer);
                    }
                    _ => (),
                }
            }
        }
    }

    Ok(())
}
