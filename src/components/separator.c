#include "components/components.h"

#include <string.h>
#include <stdlib.h>
#include <ncurses.h>

#include <livesplit_core.h>

#include "darksplit.h"

void render_separator(SeparatorComponentStateRef state)
{
	int y, x;
	getyx(stdscr, y, x);
	char *str = calloc(WIDTH + 1, 1);
	memset(str, '-', WIDTH);
	printw("%s", str);
	move(++y, 0);
	free(str);
}
