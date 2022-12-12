use crossterm::cursor::MoveTo;
use crossterm::queue;
use crossterm::style::Color as CrosstermColor;
use crossterm::style::Print;
use crossterm::style::SetBackgroundColor;
use crossterm::style::SetForegroundColor;
use crossterm::terminal::Clear;
use livesplit_core::settings::Color;
use std::io::stdout;
use std::io::Stdout;
use std::io::Write;

#[derive(Clone, PartialEq)]
pub struct TerminalCell {
    pub bg_color: Color,
    pub fg_color: Color,
    pub character: char,
}

impl Default for TerminalCell {
    fn default() -> Self {
        TerminalCell {
            bg_color: Color::black(),
            fg_color: Color::white(),
            character: ' ',
        }
    }
}

pub struct Terminal {
    on_screen: Vec<TerminalCell>,
    to_draw: Vec<TerminalCell>,
    should_redraw: bool,
    width: usize,
    height: usize,
    output: Stdout,
}

impl Terminal {
    pub fn new() -> Self {
        let output = stdout();
        Terminal {
            on_screen: Vec::new(),
            to_draw: Vec::new(),
            should_redraw: false,
            width: 0,
            height: 0,
            output,
        }
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        if width != self.width || height != self.height {
            self.should_redraw = true;
            self.width = width;
            self.height = height;

            self.on_screen = Vec::new();
            self.to_draw = Vec::new();
            self.to_draw.resize(width * height, TerminalCell::default());
        }
    }

    pub fn set_cell(&mut self, x: usize, y: usize, cell: TerminalCell) {
        if x >= self.width || y >= self.height {
            // XXX: idomatic rust says we should ensure we are in bounds and
            // raise some form of error if we are not
            //
            // for simplicity of implementation, anything out of bounds is just
            // going to be written "off-screen", as in we do absolutely nothing
            //
            // worth noting that crossterm doesn't do any sort of check like
            // this either so this likely isn't the worst thing the world
            return;
        }
        self.to_draw[y * self.width + x] = cell;
    }

    pub fn puts(
        &mut self,
        x: usize,
        y: usize,
        fg_color: Color,
        bg_color: Color,
        string: &str,
    ) {
        for (i, chr) in string.chars().enumerate() {
            if i + x > self.width {
                break;
            }

            self.set_cell(
                i + x,
                y,
                TerminalCell {
                    fg_color,
                    bg_color,
                    character: chr,
                },
            );
        }
    }

    pub fn clear_line(&mut self, y: usize, bg: Color) {
        self.puts(0, y, Color::transparent(), bg, &" ".repeat(self.width));
    }

    pub fn draw(&mut self) {
        let redraw_list: Vec<(usize, usize, &TerminalCell)> =
            if self.should_redraw {
                queue! {
                    self.output,
                    Clear(crossterm::terminal::ClearType::All),
                }
                .unwrap();

                self.to_draw
                    .iter()
                    .enumerate()
                    .map(|(i, cell)| {
                        let x = i % self.width;
                        let y = i / self.width;
                        (x, y, cell)
                    })
                    .collect()
            } else {
                self.to_draw
                    .iter()
                    .zip(&self.on_screen)
                    .enumerate()
                    .filter_map(|(i, (cell, compared))| {
                        let x = i % self.width;
                        let y = i / self.width;

                        if cell != compared {
                            Some((x, y, cell))
                        } else {
                            None
                        }
                    })
                    .collect()
            };

        for (x, y, cell) in redraw_list.iter() {
            queue! {
                self.output,
                MoveTo((*x).try_into().unwrap(), (*y).try_into().unwrap()),
                SetForegroundColor(convert_color(&cell.fg_color)),
                SetBackgroundColor(convert_color(&cell.bg_color)),
                Print(cell.character),
            }
            .unwrap();
        }

        self.output.flush().unwrap();
        self.on_screen = self.to_draw.clone();
        self.should_redraw = false;
    }

    pub fn force_redraw(&mut self) {
        self.should_redraw = true;
    }
}
fn convert_color(color: &Color) -> CrosstermColor {
    let bytes = color.to_rgba8();

    CrosstermColor::Rgb {
        r: bytes[0],
        g: bytes[1],
        b: bytes[2],
    }
}
