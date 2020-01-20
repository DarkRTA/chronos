#define _GNU_SOURCE
#include "components/components.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <ncurses.h>
#include <livesplit_core.h>

#include "darksplit.h"
#include "config.h"
void render_text(TextComponentStateRef state)
{
	char *left = NULL;
	char *right = NULL;

	if (TextComponentState_is_split(state)) {
		left = strdup(TextComponentState_left(state));
		right = strdup(TextComponentState_right(state));
	} else {
		left = strdup(TextComponentState_center(state));
		right = calloc(1, 1); //single null byte
	}

	int y, x;
	getyx(stdscr, y, x);
	mvprintw(y, 0, "%s", left);
	if (strlen(right) != 0) {
		int offset = strlen(right) + 2;
		x = WIDTH - offset;
		mvprintw(y, x, "  %s", right);
	}
	move(++y, 0);

	free(right);
	free(left);
}
