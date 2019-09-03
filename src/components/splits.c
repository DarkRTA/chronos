#include "components/components.h"

#include <string.h>
#include <ncurses.h>
#include <jansson.h>

#include "darksplit.h"
#include "color.h"

#define json_value_to_string(obj, key) \
	json_string_value(json_object_get((obj), (key)))
void render_splits(json_t *data)
{
	json_t *splits = json_object_get(data, "splits");
	json_t *split, *columns, *column;

	size_t i, j;
	const char *str;
	int colwidth[16] = {0};
	json_array_foreach(splits, i, split) {
		columns = json_object_get(split, "columns");
		json_array_foreach(columns, j, column) {
			str = json_value_to_string(column, "value");

			colwidth[j] = MAX(colwidth[j], strlen(str));
		}
	}

	int y, x;
	getyx(stdscr, y, x);
	json_array_foreach(splits, i, split) {
		if (json_boolean_value(json_object_get(split, "is_current_split")))
			attron(A_REVERSE);

		str = json_value_to_string(split, "name");
		mvprintw(y, 0, "%-*.*s", WIDTH, WIDTH, str);

		x = WIDTH;
		columns = json_object_get(split, "columns");
		json_array_foreach(columns, j, column) {
			x -= colwidth[j] + 1;
			str = json_value_to_string(column, "value");

			int color = get_semantic_color(json_value_to_string(
				column,"semantic_color"));
			attron(color);
			mvprintw(y, x, "%*s", colwidth[j] + 1, str);
			attroff(color);
		}
		move(++y, 0);
		attroff(A_REVERSE);
	}
}
