#define _GNU_SOURCE
#include "components/components.h"

#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <ncurses.h>

#include <livesplit_core.h>

#include "darksplit.h"
#include "config.h"

void render_timer(TimerComponentStateRef state)
{
	char *time_str = strdup(TimerComponentState_time(state));
	char *time_frac = strdup(TimerComponentState_fraction(state));
	int color =
		get_semantic_color(TimerComponentState_semantic_color(state));

	char *str;
	asprintf(&str, "%s%s", time_str, time_frac);

	int y, x;
	getyx(stdscr, y, x);

	attron(color);
	printw("%*.*s", WIDTH, WIDTH, str);
	attroff(color);
	move(++y, 0);

	free(str);
	free(time_str);
	free(time_frac);
}
