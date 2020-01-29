#define _GNU_SOURCE
#include "components/components.h"

#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <ncurses.h>

#include <livesplit_core.h>

#include "darksplit.h"
#include "config.h"

static void draw_line(const char *label, int color, char *tim, char *frac)
{
	char *str;
	asprintf(&str, "%s%s", tim, frac);

	int offset = strlen(str) + 2;

	int y, x;
	getyx(stdscr, y, x);
	x = WIDTH - offset;

	mvprintw(y, 0, "%.*s", WIDTH, label);
	attron(color);
	mvprintw(y, x, "  %s", str);
	attroff(color);
	move(++y, 0);
	free(str);
}

void render_detailed_timer(DetailedTimerComponentStateRef state)
{
	char *time_time = strdup(DetailedTimerComponentState_timer_time(state));
	char *time_frac =
		strdup(DetailedTimerComponentState_timer_fraction(state));
	int time_color = get_semantic_color(
		DetailedTimerComponentState_timer_semantic_color(state));

	char *seg_time =
		strdup(DetailedTimerComponentState_segment_timer_time(state));
	char *seg_frac = strdup(
		DetailedTimerComponentState_segment_timer_fraction(state));

	draw_line("Time", time_color, time_time, time_frac);
	draw_line("Segment", 0, seg_time, seg_frac);

	free(time_time);
	free(time_frac);
	free(seg_time);
	free(seg_frac);
}
