use livesplit_core::component::blank_space::State as BlankSpaceState;
use livesplit_core::component::detailed_timer::State as DetailedTimerState;
use livesplit_core::component::key_value::State as KeyValueState;
use livesplit_core::component::separator::State as SeparatorState;
use livesplit_core::component::splits::State as SplitsState;
use livesplit_core::component::timer::State as TimerState;
use livesplit_core::component::title::State as TitleState;
use livesplit_core::layout::ComponentState;
use livesplit_core::layout::LayoutState;
use livesplit_core::settings::Color as LsColor;
use livesplit_core::settings::Gradient;
use livesplit_core::settings::ListGradient;

use crate::terminal::Terminal;

// TODO: support reading colors from the layout, and make an attempt to support
// transparency

pub fn render(term: &mut Terminal, layout_state: LayoutState) {
    let (term_width, term_height) = crossterm::terminal::size().unwrap();
    let term_width = term_width as usize;
    let term_height = term_height as usize;

    let mut current_line = 0;
    term.resize(term_width, term_height);

    for component in layout_state.components.iter() {
        match component {
            ComponentState::Timer(state) => {
                render_timer(
                    term,
                    &mut current_line,
                    term_width,
                    state,
                    &layout_state,
                );
            }
            ComponentState::Title(state) => {
                render_title(
                    term,
                    &mut current_line,
                    term_width,
                    state,
                    &layout_state,
                );
            }
            ComponentState::KeyValue(state) => {
                render_key_value(
                    term,
                    &mut current_line,
                    term_width,
                    state,
                    &layout_state,
                );
            }
            ComponentState::Splits(state) => {
                render_splits(
                    term,
                    &mut current_line,
                    term_width,
                    state,
                    &layout_state,
                );
            }
            ComponentState::BlankSpace(state) => {
                render_blank_space(
                    term,
                    &mut current_line,
                    term_width,
                    state,
                    &layout_state,
                );
            }
            ComponentState::Separator(state) => {
                render_seperator(
                    term,
                    &mut current_line,
                    term_width,
                    state,
                    &layout_state,
                );
            }
            ComponentState::DetailedTimer(state) => {
                render_detailed_timer(
                    term,
                    &mut current_line,
                    term_width,
                    state,
                    &layout_state,
                );
            }
            _ => (),
        }
    }

    term.draw();
}

fn render_timer(
    term: &mut Terminal,
    current_line: &mut usize,
    layout_width: usize,
    state: &TimerState,
    layout_state: &LayoutState,
) {
    let output_string = format!("{}{}", state.time, state.fraction);
    let x_pos = layout_width.saturating_sub(output_string.chars().count());
    let color = process_gradient(&Gradient::Horizontal(
        state.top_color,
        state.bottom_color,
    ));
    let bg_color =
        process_gradient_bg(&state.background, &layout_state.background);

    term.clear_line(*current_line, bg_color);
    term.puts(x_pos, *current_line, color, bg_color, &output_string);
    *current_line += 1;
}
fn render_title(
    term: &mut Terminal,
    current_line: &mut usize,
    layout_width: usize,
    state: &TitleState,
    layout_state: &LayoutState,
) {
    let fg_color = state.text_color.unwrap_or(layout_state.text_color);
    let bg_color =
        process_gradient_bg(&state.background, &layout_state.background);

    let attempt_counter = match state.attempts {
        Some(attempts) => match state.finished_runs {
            Some(finished) => format!("  {attempts} / {finished}"),
            None => format!("{attempts}"),
        },
        None => "".to_string(),
    };

    let empty_string = Box::from("");
    let line_1 = state.line1.last().unwrap_or(&empty_string);
    let line_2 = state.line2.first().unwrap_or(&empty_string);

    term.clear_line(*current_line, bg_color);
    term.puts(0, *current_line, fg_color, bg_color, line_1);
    *current_line += 1;
    term.clear_line(*current_line, bg_color);
    term.puts(0, *current_line, fg_color, bg_color, line_2);
    let x_pos = layout_width.saturating_sub(attempt_counter.chars().count());
    term.puts(x_pos, *current_line, fg_color, bg_color, &attempt_counter);
    *current_line += 1;
}

fn render_key_value(
    term: &mut Terminal,
    current_line: &mut usize,
    layout_width: usize,
    state: &KeyValueState,
    layout_state: &LayoutState,
) {
    let key_color = state.key_color.unwrap_or(layout_state.text_color);
    let value_color = state.value_color.unwrap_or(layout_state.text_color);
    let bg_color =
        process_gradient_bg(&state.background, &layout_state.background);
    let value = format!("  {}", &state.value);
    let x_pos = layout_width.saturating_sub(value.chars().count());

    term.clear_line(*current_line, bg_color);
    term.puts(0, *current_line, key_color, bg_color, &state.key);
    term.puts(x_pos, *current_line, value_color, bg_color, &value);
    *current_line += 1;
}

fn render_splits(
    term: &mut Terminal,
    current_line: &mut usize,
    layout_width: usize,
    state: &SplitsState,
    layout_state: &LayoutState,
) {
    let column_count = state
        .splits
        .iter()
        .map(|a| a.columns.len())
        .max()
        .unwrap_or(0);

    let mut column_widths: Vec<usize> = Vec::new();
    column_widths.resize(column_count, 0);

    // justification: this range loop is needed as we can't assume that every
    // split has the same number of columns. since we have the `i` variable
    // anyway, there's no harm in indexing directly into `column_widths`.
    #[allow(clippy::needless_range_loop)]
    for i in 0..column_count {
        column_widths[i] = state
            .splits
            .iter()
            .filter_map(|split| {
                if i < split.columns.len() {
                    Some(split.columns[i].value.chars().count())
                } else {
                    None
                }
            })
            .max()
            .unwrap();
    }

    for (i, split) in state.splits.iter().enumerate() {
        let bg_color = match (split.is_current_split, state.background) {
            (false, ListGradient::Same(color)) => {
                process_gradient_bg(&color, &layout_state.background)
            }
            (false, ListGradient::Alternating(even, odd)) => {
                if i % 2 == 0 {
                    even
                } else {
                    odd
                }
            }
            (true, _) => process_gradient(&state.current_split_gradient),
        };

        let fg_color = layout_state.text_color;

        term.clear_line(*current_line, bg_color);
        term.puts(0, *current_line, fg_color, bg_color, &split.name);

        let mut x_pos = layout_width;
        for (j, column) in split.columns.iter().enumerate() {
            x_pos = x_pos.saturating_sub(column_widths[j] + 1);
            let fg_color = column.visual_color;
            let text = format!(
                "{: >w$}",
                column.value,
                w = (column_widths[j] + 1)
            );

            term.puts(x_pos, *current_line, fg_color, bg_color, &text);
        }

        *current_line += 1
    }
}

fn render_blank_space(
    term: &mut Terminal,
    current_line: &mut usize,
    _layout_width: usize,
    state: &BlankSpaceState,
    layout_state: &LayoutState,
) {
    let bg_color =
        process_gradient_bg(&state.background, &layout_state.background);
    term.clear_line(*current_line, bg_color);
    *current_line += 1;
}

fn render_seperator(
    term: &mut Terminal,
    current_line: &mut usize,
    layout_width: usize,
    _state: &SeparatorState,
    layout_state: &LayoutState,
) {
    let bg_color = process_gradient(&layout_state.background);
    let fg_color = layout_state.separators_color;
    term.puts(
        0,
        *current_line,
        fg_color,
        bg_color,
        &"-".repeat(layout_width),
    );
    *current_line += 1;
}

fn render_detailed_timer(
    term: &mut Terminal,
    current_line: &mut usize,
    layout_width: usize,
    state: &DetailedTimerState,
    layout_state: &LayoutState,
) {
    let bg_color =
        process_gradient_bg(&state.background, &layout_state.background);
    let text_color = layout_state.text_color;
    let timer_color = process_gradient(&Gradient::Horizontal(
        state.timer.top_color,
        state.timer.bottom_color,
    ));
    let segment_timer_color = process_gradient(&Gradient::Horizontal(
        state.segment_timer.top_color,
        state.segment_timer.bottom_color,
    ));

    let line1 = match &state.comparison1 {
        Some(x) => format!("{}: {}", x.name, x.time),
        None => "".into(),
    };

    let line2 = match &state.comparison2 {
        Some(x) => format!("{}: {}", x.name, x.time),
        None => "".into(),
    };

    let time = format!("  {}{}", state.timer.time, state.timer.fraction);
    let x_pos_time = layout_width.saturating_sub(time.chars().count());

    let segment_time = format!(
        "  {}{}",
        state.segment_timer.time, state.segment_timer.fraction
    );
    let x_pos_segment_time = layout_width.saturating_sub(time.chars().count());

    term.clear_line(*current_line, bg_color);
    term.puts(0, *current_line, text_color, bg_color, &line1);
    term.puts(x_pos_time, *current_line, timer_color, bg_color, &time);
    *current_line += 1;

    term.clear_line(*current_line, bg_color);
    term.puts(0, *current_line, text_color, bg_color, &line2);
    term.puts(
        x_pos_segment_time,
        *current_line,
        segment_timer_color,
        bg_color,
        &segment_time,
    );
    *current_line += 1;
}

fn process_gradient(gradient: &Gradient) -> LsColor {
    match gradient {
        Gradient::Transparent => LsColor::black(),
        Gradient::Plain(c) => *c,
        Gradient::Vertical(a, b) | Gradient::Horizontal(a, b) => LsColor {
            red: (a.red + b.red) / 2.0,
            green: (a.green + b.green) / 2.0,
            blue: (a.blue + b.blue) / 2.0,
            alpha: (a.alpha + b.alpha) / 2.0,
        },
    }
}

// TODO: clean this up
fn process_gradient_bg(gradient: &Gradient, layout_bg: &Gradient) -> LsColor {
    match gradient {
        Gradient::Transparent => process_gradient(layout_bg),
        Gradient::Plain(c) => *c,
        Gradient::Vertical(a, b) | Gradient::Horizontal(a, b) => LsColor {
            red: (a.red + b.red) / 2.0,
            green: (a.green + b.green) / 2.0,
            blue: (a.blue + b.blue) / 2.0,
            alpha: (a.alpha + b.alpha) / 2.0,
        },
    }
}
