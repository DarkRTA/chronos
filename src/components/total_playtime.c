#define _GNU_SOURCE
#include "components/components.h"

#include <stdio.h>
#include <string.h>

#include <ncurses.h>
#include <jansson.h>

#include "darksplit.h"
#include "color.h"
void render_total_playtime(json_t *data)
{
	const char *text = json_obj_string(data, "text");
	const char *val = json_obj_string(data, "time");

	int offset = strlen(val) + 2;

	int y, x;
	getyx(stdscr, y, x);
	x = WIDTH - offset;

	mvprintw(y, 0, "%-.*s", WIDTH, text);
	mvprintw(y, x, "  %s", val);
	move(++y, 0);
}
