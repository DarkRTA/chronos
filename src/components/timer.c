#define _GNU_SOURCE
#include "components/components.h"

#include <stdio.h>
#include <ncurses.h>
#include <jansson.h>

#include "darksplit.h"
#include "color.h"

void render_timer(json_t *data)
{
	const char *time_str =
		json_string_value(json_object_get(data, "time"));
	const char *time_frac =
		json_string_value(json_object_get(data, "fraction"));

	char *str;
	asprintf(&str, "%s%s", time_str, time_frac);

	const char *colorstr = json_string_value(
			json_object_get(data, "semantic_color"));

	int color = get_semantic_color(colorstr);
	attron(color);
	printw("%*.*s\n", WIDTH, WIDTH, str);
	attroff(color);
	free(str);
}
