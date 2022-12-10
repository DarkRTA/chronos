#include "components/components.h"

#include <string.h>
#include <stdlib.h>
#include <termbox/termbox.h>

#include <livesplit_core.h>

#include "chronos.h"

void render_separator(UNUSED SeparatorComponentStateRef state, int *line)
{
	for (int i = 0; i < WIDTH; i++) {
		tb_change_cell(i, *line, '-', 0, 0);
	}
	(*line)++;
}
