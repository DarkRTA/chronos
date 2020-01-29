#include "components/components.h"

#include <stdlib.h>
#include <string.h>
#include <ncurses.h>

#include <livesplit_core.h>

#include "darksplit.h"
#include "config.h"

void render_splits(SplitsComponentStateRef state)
{
	//We need to loop through the splits array in 2 passes here.
	//This first pass is used to calculate the width of the columns.
	int colwidth[16] = { 0 };

	int len = SplitsComponentState_len(state);
	for (int i = 0; i < len; i++) {
		int colcount = SplitsComponentState_columns_len(state, i);
		for (int j = 0; j < colcount; j++) {
			int collen = strlen(
				SplitsComponentState_column_value(state, i, j));
			if (collen > colwidth[j])
				colwidth[j] = collen;
		}
	}

	//On the second pass, we will actually draw the splits.
	int y, x;
	getyx(stdscr, y, x);
	for (int i = 0; i < len; i++) {
		//if this is the current split, use reverse video.
		if (SplitsComponentState_is_current_split(state, i))
			attron(A_REVERSE);

		//get the name of the split and draw it.
		const char *name = SplitsComponentState_name(state, i);
		mvprintw(y, 0, "%-*.*s", WIDTH, WIDTH, name);

		//this loop will draw the columns
		int colcount = SplitsComponentState_columns_len(state, i);
		x = WIDTH;
		for (int j = 0; j < colcount; j++) {
			//move the cursor back the width of the current column
			x -= colwidth[j] + 1;
			char *val = strdup(
				SplitsComponentState_column_value(state, i, j));

			//get the semantic color for the column
			int color = get_semantic_color(
				SplitsComponentState_column_semantic_color(
					state, i, j));

			//draw the column
			attron(color);
			mvprintw(y, x, "%*s", colwidth[j] + 1, val);
			attroff(color);
			free(val);
		}
		//advance to the next line and disable reverse video
		move(++y, 0);
		attroff(A_REVERSE);
	}
}
