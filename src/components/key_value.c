#define _GNU_SOURCE
#include "components/components.h"

#include <stdio.h>
#include <string.h>

#include <ncurses.h>

#include <cjson/cJSON.h>

#include "darksplit.h"
#include "config.h"
void render_key_value(cJSON *data)
{
	const char *key = cJSON_GetObjectItem(data, "key")->valuestring;
	const char *value = cJSON_GetObjectItem(data, "value")->valuestring;

	int offset = strlen(value) + 2;

	int y, x;
	getyx(stdscr, y, x);
	x = WIDTH - offset;

	int color = get_semantic_color(
		cJSON_GetObjectItem(data, "semantic_color")->valuestring);

	mvprintw(y, 0, "%.*s", WIDTH, key);
	attron(color);
	mvprintw(y, x, "  %s", value);
	attroff(color);
	move(++y, 0);
}
