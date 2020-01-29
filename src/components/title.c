#define _GNU_SOURCE
#include "components/components.h"

#include <stdlib.h>
#include <stdio.h>
#include <string.h>

#include <ncurses.h>
#include <livesplit_core.h>

#include "darksplit.h"

void render_title(TitleComponentStateRef state)
{
	char *line1 = strdup(TitleComponentState_line1(state));
	char *line2 = strdup(TitleComponentState_line2(state));

	int y, x;
	getyx(stdscr, y, x);
	printw("%-*.*s", WIDTH, WIDTH, line1);
	move(++y, 0);
	printw("%-*.*s", WIDTH, WIDTH, line2);

	int attempts = TitleComponentState_attempts(state);
	int finished = TitleComponentState_finished_runs(state);

	char *attstr = NULL;
	if (TitleComponentState_shows_attempts(state)) {
		if (TitleComponentState_shows_finished_runs(state))
			asprintf(&attstr, "%d / %d", finished, attempts);
		else
			asprintf(&attstr, "%d", attempts);
	} else {
		attstr = calloc(1, 1); //single null byte
	}

	mvprintw(y, WIDTH - strlen(attstr) - 2, "  %s", attstr);
	move(++y, 0);

	free(attstr);
	free(line2);
	free(line1);
}
