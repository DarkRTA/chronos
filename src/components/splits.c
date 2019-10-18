#include "components/components.h"

#include <string.h>
#include <ncurses.h>
#include <jansson.h>

#include "darksplit.h"
#include "color.h"

// FIXME: make this less shitty
// But no seriously this code is awful and I should consider rewriting it to
// something cleaner.
void render_splits(json_t *data)
{
	json_t *splits = json_object_get(data, "splits");
	json_t *split, *columns, *column;

	size_t i, j;
	const char *str;
	int colwidth[16] = {0};
	//We need to loop through the splits array in 2 passes here.
	//This first pass is used to calculate the width of the columns.
	json_array_foreach(splits, i, split) {
		columns = json_object_get(split, "columns");
		json_array_foreach(columns, j, column) {
			str = json_obj_string(column, "value");

			colwidth[j] = MAX(colwidth[j], strlen(str));
		}
	}

	//On the second pass, we will actually draw the splits.
	int y, x;
	getyx(stdscr, y, x);
	json_array_foreach(splits, i, split) {
		//if this is the current split, use reverse video.
		if (json_obj_bool(split, "is_current_split"))
			attron(A_REVERSE);

		//get the name of the split and draw it.
		str = json_obj_string(split, "name");
		mvprintw(y, 0, "%-*.*s", WIDTH, WIDTH, str);

		//this loop will draw the columns
		x = WIDTH;
		columns = json_object_get(split, "columns");
		json_array_foreach(columns, j, column) {
			//move the cursor back the width of the current column
			x -= colwidth[j] + 1;
			str = json_obj_string(column, "value");

			//get the semantic color for the column
			int color = get_semantic_color(json_obj_string(
				column,"semantic_color"));
			//draw the column
			attron(color);
			mvprintw(y, x, "%*s", colwidth[j] + 1, str);
			attroff(color);
		}
		//advance to the next line and disable reverse video
		move(++y, 0);
		attroff(A_REVERSE);
	}
}
