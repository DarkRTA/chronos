#define _GNU_SOURCE
#include "components/components.h"

#include <stdio.h>
#include <string.h>

#include <ncurses.h>
#include <cjson/cJSON.h>

#include "darksplit.h"
#include "config.h"
void render_text(cJSON *data)
{
	cJSON *obj = cJSON_GetObjectItem(data, "text");
	cJSON *arr;
	const char *text;
	const char *val = "";

	if (cJSON_GetObjectItem(obj, "Center") == NULL) {
		arr = cJSON_GetObjectItem(obj, "Split");
		text = cJSON_GetArrayItem(arr, 0)->valuestring;
		val = cJSON_GetArrayItem(arr, 1)->valuestring;
	} else {
		text = cJSON_GetObjectItem(obj, "Center")->valuestring;
	}

	int y, x;
	getyx(stdscr, y, x);

	mvprintw(y, 0, "%s", text);

	if (strlen(val) != 0) {
		int offset = strlen(val) + 2;
		x = WIDTH - offset;
		mvprintw(y, x, "  %s", val);
	}

	move(++y, 0);
}
