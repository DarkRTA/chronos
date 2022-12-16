use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::stdout;
use std::io::BufWriter;
use std::io::Cursor;
use std::time::Duration;

use clap::Parser;
use crossterm::cursor;
use crossterm::event;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::terminal::EnterAlternateScreen;
use crossterm::terminal::LeaveAlternateScreen;
use crossterm::ExecutableCommand;
use livesplit_core::auto_splitting;
use livesplit_core::layout::LayoutSettings;
use livesplit_core::run::parser;
use livesplit_core::run::saver::livesplit;
use livesplit_core::HotkeySystem;
use livesplit_core::Layout;
use livesplit_core::Timer;

use std::panic;

use chronos::Config;

pub mod renderer;
pub mod terminal;

pub const DEFAULT_LAYOUT: &[u8; 6413] = include_bytes!("default_layout.ls1l");

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

fn register_panic_handler() {
    let default_hook = panic::take_hook();

    panic::set_hook(Box::new(move |info| {
        drop(stdout().execute(LeaveAlternateScreen));
        drop(stdout().execute(cursor::Show));
        drop(crossterm::terminal::disable_raw_mode());
        default_hook(info);
    }));
}

fn main() -> Result<(), Box<dyn Error>> {
    register_panic_handler();

    let args = Args::parse();
    let splits_file = fs::read(&args.splits)?;
    let run = parser::composite::parse(
        &splits_file,
        Some((&args.splits).into()),
        true,
    )?
    .run;
    let stimer = Timer::new(run)?.into_shared();

    let mut layout = match args.layout {
        Some(s) => {
            let settings = LayoutSettings::from_json(File::open(s)?)?;
            Layout::from_settings(settings)
        }
        None => {
            let layout = DEFAULT_LAYOUT;
            let cursor = Cursor::new(layout);
            let settings = LayoutSettings::from_json(cursor)?;
            Layout::from_settings(settings)
        }
    };

    let config = match &args.config {
        Some(path) => toml::de::from_slice(&fs::read(path)?)?,
        None => Config::default(),
    };

    // we actually do nothing with this but will need to hold onto it as the
    // mere act of creating one will start a thread that needs to be kept alive
    let _auto_splitter = match &args.auto_splitter {
        Some(path) => {
            let runtime = auto_splitting::Runtime::new(stimer.clone());
            runtime.load_script_blocking(path.into())?;
            Some(runtime)
        }
        None => None,
    };

    let mut hotkey_system = match HotkeySystem::with_config(
        stimer.clone(),
        config.global_hotkeys,
    ) {
        Ok(hks) => Some(hks),
        Err(err) => {
            println!("error intitializing the hotkey system: {err}");
            println!("global hotkeys will be unavailable");
            println!("press any key to continue...");
            crossterm::terminal::enable_raw_mode()?;
            drop(event::read());
            crossterm::terminal::disable_raw_mode()?;
            None
        }
    };

    let mut term = terminal::Terminal::new();
    let mut stdout = stdout();
    crossterm::terminal::enable_raw_mode().unwrap();
    stdout.execute(EnterAlternateScreen).unwrap();
    stdout.execute(cursor::Hide).unwrap();

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
                    KeyCode::F(1) => {
                        if let Some(ref mut hks) = hotkey_system {
                            hks.activate()?;
                        }
                    }
                    KeyCode::F(2) => {
                        if let Some(ref mut hks) = hotkey_system {
                            hks.deactivate()?;
                        }
                    }
                    KeyCode::F(3) => {
                        stdout.execute(LeaveAlternateScreen).unwrap();
                        let file = File::create(&args.splits)?;
                        let writer = BufWriter::new(file);
                        livesplit::save_run(
                            timer.snapshot().run(),
                            livesplit::IoWrite(writer),
                        )?;
                        println!("splits saved successfully...\r");
                        drop(event::read()?);
                        term.force_redraw();
                        stdout.execute(EnterAlternateScreen).unwrap();
                    }
                    KeyCode::Char(chr) => {
                        config.local_hotkeys.do_key(chr, &mut timer);
                    }
                    _ => (),
                }
            }
        }
    }

    stdout.execute(LeaveAlternateScreen).unwrap();
    stdout.execute(cursor::Show).unwrap();
    crossterm::terminal::disable_raw_mode().unwrap();

    Ok(())
}
