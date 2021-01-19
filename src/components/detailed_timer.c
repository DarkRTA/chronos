#define _GNU_SOURCE
#include "components/components.h"

#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <termbox/termbox.h>

#include <livesplit_core.h>

#include "chronos.h"
#include "config.h"
#include "tb_extras.h"

static void
draw_line(const char *label, int *line, int color, char *time, char *frac)
{
	char *str;
	asprintf(&str, "%s%s", time, frac);
	int x = WIDTH - strlen(str);

	tb_put_string(0, *line, label, 0, 0);
	tb_put_string(x - 2, *line, "  ", 0, 0);
	tb_put_string(x, *line, str, color, 0);
	*line += 1;
	free(str);
}

void render_detailed_timer(DetailedTimerComponentStateRef state, int *line)
{
	char *time_time = strdup(DetailedTimerComponentState_timer_time(state));
	char *time_frac =
		strdup(DetailedTimerComponentState_timer_fraction(state));
	int time_color = config_get_semantic_color(
		DetailedTimerComponentState_timer_semantic_color(state));

	char *seg_time =
		strdup(DetailedTimerComponentState_segment_timer_time(state));
	char *seg_frac = strdup(
		DetailedTimerComponentState_segment_timer_fraction(state));

	draw_line("Time", line, time_color, time_time, time_frac);
	draw_line("Segment", line, 0, seg_time, seg_frac);

	free(time_time);
	free(time_frac);
	free(seg_time);
	free(seg_frac);
}
