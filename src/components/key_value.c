#define _GNU_SOURCE
#include "components/components.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <ncurses.h>

#include <livesplit_core.h>

#include "darksplit.h"
#include "config.h"
void render_key_value(KeyValueComponentStateRef state)
{
	char *key = strdup(KeyValueComponentState_key(state));
	char *value = strdup(KeyValueComponentState_value(state));

	int color = get_semantic_color(
		KeyValueComponentState_semantic_color(state));

	int offset = strlen(value) + 2;

	int y, x;
	getyx(stdscr, y, x);
	x = WIDTH - offset;

	mvprintw(y, 0, "%.*s", WIDTH, key);
	attron(color);
	mvprintw(y, x, "  %s", value);
	attroff(color);
	move(++y, 0);

	free(key);
	free(value);
}
