#pragma once
#include <livesplit_core.h>
void render_blank_space(BlankSpaceComponentStateRef state, int *line);
void render_detailed_timer(DetailedTimerComponentStateRef state, int *line);
void render_key_value(KeyValueComponentStateRef state, int *line);
void render_separator(SeparatorComponentStateRef state, int *line);
void render_splits(SplitsComponentStateRef state, int *line);
void render_timer(TimerComponentStateRef state, int *line);
void render_text(TextComponentStateRef state, int *line);
void render_title(TitleComponentStateRef state, int *line);
