mod views;
mod global_state;
mod error;
mod parser;
mod save;

use views::layout;
use global_state::GlobalState;
use parser::{parse_args, parse_splits_file};

use cursive::Cursive;
use livesplit_core::run::Editor;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args = parse_args();
    let splits_file = args.splits_file.clone();
    let splits = parse_splits_file(args);

    let editor = Editor::new(splits)?;

    let globals = GlobalState {
        splits_editor: editor,
        splits_file: splits_file,
    };

    let backend = {
        let backend = cursive::backends::crossterm::Backend::init()?;
        let buffered = cursive_buffered_backend::BufferedBackend::new(backend);
        Box::new(buffered)
    };

    let mut siv = cursive::CursiveRunner::new(Cursive::new(), backend);
    cursive::logger::init();
    siv.load_toml(chronos::CURSIVE_THEME).unwrap();

    siv.add_global_callback('~', cursive::Cursive::toggle_debug_console);
    siv.set_user_data(globals);
    layout::render_layout(&mut siv);
    siv.run();
    Ok(())
}

