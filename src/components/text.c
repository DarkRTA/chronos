#include "components/components.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <termbox/termbox.h>
#include <livesplit_core.h>

#include "chronos.h"
#include "tb_extras.h"
#include "config.h"

void render_text(TextComponentStateRef state, int *line)
{
	char *left = NULL;
	char *right = NULL;

	if (TextComponentState_is_split(state)) {
		left = strdup(TextComponentState_left(state));
		right = strdup(TextComponentState_right(state));
	} else {
		left = strdup(TextComponentState_center(state));
		right = calloc(1, 1); //single null byte
	}

	tb_put_string(0, *line, left, 0, 0);

	if (strlen(right) != 0) {
		int x = WIDTH - strlen(right);
		tb_put_string(x - 2, *line, "  ", 0, 0);
		tb_put_string(x, *line, right, 0, 0);
	}

	*line += 1;

	free(right);
	free(left);
}
