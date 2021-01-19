#include "components/components.h"

#include <stdlib.h>
#include <string.h>
#include <termbox/termbox.h>

#include <livesplit_core.h>

#include "chronos.h"
#include "tb_extras.h"
#include "config.h"

void render_splits(SplitsComponentStateRef state, int *line)
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
	for (int i = 0; i < len; i++) {
		//if this is the current split, use reverse video.
		uint16_t attr = SplitsComponentState_is_current_split(state, i)
			? TB_REVERSE
			: 0;

		//get the name of the split and draw it.
		const char *name = SplitsComponentState_name(state, i);
		for (int k = 0; k < WIDTH; k++) {
			tb_change_cell(k, *line, ' ', attr, 0);
		}
		tb_put_string(0, *line, name, attr, 0);

		//this loop will draw the columns
		int colcount = SplitsComponentState_columns_len(state, i);
		int x = WIDTH;
		for (int j = 0; j < colcount; j++) {
			//move the cursor back the width of the current column
			x -= colwidth[j] + 1;
			char *val = strdup(
				SplitsComponentState_column_value(state, i, j));

			//get the semantic color for the column
			int color = config_get_semantic_color(
				SplitsComponentState_column_semantic_color(
					state,
					i,
					j));

			//draw the column
			for (int k = x; k < x + colwidth[j] + 1; k++) {
				tb_change_cell(k, *line, ' ', color | attr, 0);
			}
			tb_put_string(
				x + (colwidth[j] - strlen(val)) + 1,
				*line,
				val,
				color | attr,
				0);
			free(val);
		}
		//advance to the next line
		(*line)++;
	}
}
