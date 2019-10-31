#define _GNU_SOURCE
#include "components/components.h"

#include <stdio.h>
#include <string.h>

#include <ncurses.h>
#include <jansson.h>

#include "darksplit.h"
#include "config.h"
void render_text(json_t *data)
{
	json_t *obj = json_object_get(data, "text");
	json_t *arr;
	const char *text;
	const char *val = "";

	if (json_object_get(obj, "Center") == NULL) {
		arr = json_object_get(obj, "Split");
		text = json_string_value(json_array_get(arr, 0));
		val = json_string_value(json_array_get(arr, 1));
	} else {
		text = json_obj_string(obj, "Center");
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
