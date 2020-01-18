#include "components/components.h"

#include <stdlib.h>
#include <string.h>
#include <ncurses.h>

#include <cjson/cJSON.h>

#include "darksplit.h"
#include "config.h"

// FIXME: make this less shitty
// But no seriously this code is awful and I should consider rewriting it to
// something cleaner.
void render_splits(cJSON *data)
{
	cJSON *splits = cJSON_GetObjectItem(data, "splits");
	cJSON *split, *columns, *column;

	const char *str;
	int colwidth[16] = { 0 };
	//We need to loop through the splits array in 2 passes here.
	//This first pass is used to calculate the width of the columns.
	cJSON_ArrayForEach (split, splits) {
		columns = cJSON_GetObjectItem(split, "columns");
		int j = 0;
		cJSON_ArrayForEach(column, columns) {
			str = cJSON_GetObjectItem(column, "value")->valuestring;
			colwidth[j] = MAX(colwidth[j], strlen(str));
			j++;
		}
	}

	//On the second pass, we will actually draw the splits.
	int y, x;
	getyx(stdscr, y, x);
	cJSON_ArrayForEach (split, splits) {
		//if this is the current split, use reverse video.
		if (cJSON_GetObjectItem(split, "is_current_split")->valueint)
			attron(A_REVERSE);

		//get the name of the split and draw it.
		str = cJSON_GetObjectItem(split, "name")->valuestring;
		mvprintw(y, 0, "%-*.*s", WIDTH, WIDTH, str);

		//this loop will draw the columns
		x = WIDTH;
		int j = 0;
		columns = cJSON_GetObjectItem(split, "columns");
		cJSON_ArrayForEach (column, columns) {
			//move the cursor back the width of the current column
			x -= colwidth[j] + 1;
			str = cJSON_GetObjectItem(column, "value")->valuestring;

			//get the semantic color for the column
			int color = get_semantic_color(
				cJSON_GetObjectItem(column, "semantic_color")
					->valuestring);
			//draw the column
			attron(color);
			mvprintw(y, x, "%*s", colwidth[j] + 1, str);
			attroff(color);

			j++;
		}
		//advance to the next line and disable reverse video
		move(++y, 0);
		attroff(A_REVERSE);
	}
}
