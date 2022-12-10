#include "components/components.h"

#include <stdlib.h>
#include <stdio.h>
#include <string.h>

#include <termbox/termbox.h>
#include <livesplit_core.h>

#include "chronos.h"
#include "tb_extras.h"

void render_title(TitleComponentStateRef state, int *line)
{
	char *line1 = strdup(TitleComponentState_line1(state));
	char *line2 = strdup(TitleComponentState_line2(state));

	tb_put_string(0, (*line)++, line1, 0, 0);
	tb_put_string(0, *line, line2, 0, 0);

	int attempts = TitleComponentState_attempts(state);
	int finished = TitleComponentState_finished_runs(state);

	char attempts_string[50] = { 0 };

	if (TitleComponentState_shows_attempts(state)) {
		if (TitleComponentState_shows_finished_runs(state))
			snprintf(attempts_string, 50, "%d / %d", finished, attempts);
		else
			snprintf(attempts_string, 50, "%d", attempts);
	}

	int x = WIDTH - strlen(attempts_string);
	tb_put_string(x - 2, *line, "  ", 0, 0);
	tb_put_string(x, *line, attempts_string, 0, 0);
	*line += 1;

	free(line2);
	free(line1);
}
