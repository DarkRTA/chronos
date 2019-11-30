#define _GNU_SOURCE
#include "components/components.h"

#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <ncurses.h>

#include <cjson/cJSON.h>

#include "darksplit.h"
#include "config.h"

static void draw_line(const char *label, cJSON *data)
{
	const char *time_str = cJSON_GetObjectItem(data, "time")->valuestring;
	const char *time_frac =
		cJSON_GetObjectItem(data, "fraction")->valuestring;

	char *str;
	asprintf(&str, "%s%s", time_str, time_frac);

	int offset = strlen(str) + 2;

	int y, x;
	getyx(stdscr, y, x);
	x = WIDTH - offset;

	int color = get_semantic_color(
		cJSON_GetObjectItem(data, "semantic_color")->valuestring);

	mvprintw(y, 0, "%.*s", WIDTH, label);
	attron(color);
	mvprintw(y, x, "  %s", str);
	attroff(color);
	move(++y, 0);
	free(str);
}

void render_detailed_timer(cJSON *data)
{
	draw_line("Time", cJSON_GetObjectItem(data, "timer"));
	draw_line("Segment", cJSON_GetObjectItem(data, "segment_timer"));
}
