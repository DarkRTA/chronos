#define _GNU_SOURCE
#include "components/components.h"

#include <stdio.h>
#include <string.h>
#include <ncurses.h>
#include <jansson.h>

#include "darksplit.h"

void render_title(json_t *data)
{
	int y, x;
	getyx(stdscr, y, x);
	const char *str = json_string_value(json_object_get(data, "line1"));
	printw("%-*.*s", WIDTH, WIDTH, str);
	move(++y, 0);
	str = json_string_value(json_object_get(data, "line2"));
	printw("%-*.*s", WIDTH, WIDTH, str);

	int attempts = json_real_value(json_object_get(data, "attempts"));
	int finished = json_real_value(json_object_get(data, "finished_runs"));
	char *attstr;

	if (!json_is_null(json_object_get(data, "attempts"))) {
		if (!json_is_null(json_object_get(data, "finished_runs"))) 
			asprintf(&attstr, "%d / %d", finished, attempts);
		else 
			asprintf(&attstr, "%d", attempts);
		
	}
	mvprintw(y, WIDTH - strlen(attstr) - 2, "  %s", attstr);
	free(attstr);
	move(++y, 0);
}
