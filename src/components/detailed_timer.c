#define _GNU_SOURCE
#include "components/components.h"

#include <stdio.h>
#include <string.h>
#include <ncurses.h>
#include <jansson.h>

#include "darksplit.h"
#include "color.h"


static void draw_line(const char *label, json_t *data) {
	const char *time_str =
		json_string_value(json_object_get(data, "time"));
	const char *time_frac =
		json_string_value(json_object_get(data, "fraction"));

	char *str;
	asprintf(&str, "%s%s", time_str, time_frac);

	int offset = strlen(str) + 2;

	int y, x;
	getyx(stdscr, y, x);
	x = WIDTH - offset;

	int color = get_semantic_color(
			json_obj_string(data, "semantic_color"));

	mvprintw(y, 0, "%.*s", WIDTH, label);
	attron(color);
	mvprintw(y, x, "  %s", str);
	attroff(color);
	move(++y, 0);
	free(str);
}

void render_detailed_timer(json_t *data)
{
	draw_line("Time", json_object_get(data, "timer"));
	draw_line("Segment", json_object_get(data, "segment_timer"));

}
