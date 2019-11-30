#define _GNU_SOURCE
#include "components/components.h"

#include <stdlib.h>
#include <stdio.h>
#include <ncurses.h>

#include <cjson/cJSON.h>

#include "darksplit.h"
#include "config.h"

void render_timer(cJSON *data)
{
	int y, x;
	getyx(stdscr, y, x);
	const char *time_str = cJSON_GetObjectItem(data, "time")->valuestring;
	const char *time_frac =
		cJSON_GetObjectItem(data, "fraction")->valuestring;

	char *str;
	asprintf(&str, "%s%s", time_str, time_frac);

	int color = get_semantic_color(
		cJSON_GetObjectItem(data, "semantic_color")->valuestring);

	attron(color);
	printw("%*.*s", WIDTH, WIDTH, str);
	attroff(color);
	free(str);
	move(++y, 0);
}
