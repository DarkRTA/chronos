#define _GNU_SOURCE
#include "components/components.h"

#include <stdio.h>
#include <ncurses.h>
#include <jansson.h>

#include "darksplit.h"
#include "config.h"

void render_timer(json_t *data)
{
	int y, x;
	getyx(stdscr, y, x);
	const char *time_str = json_string_value(json_object_get(data, "time"));
	const char *time_frac =
		json_string_value(json_object_get(data, "fraction"));

	char *str;
	asprintf(&str, "%s%s", time_str, time_frac);

	const char *colorstr =
		json_string_value(json_object_get(data, "semantic_color"));

	int color = get_semantic_color(colorstr);
	attron(color);
	printw("%*.*s", WIDTH, WIDTH, str);
	attroff(color);
	free(str);
	move(++y, 0);
}
