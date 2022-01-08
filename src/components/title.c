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

	char *attstr = NULL;
	if (TitleComponentState_shows_attempts(state)) {
		if (TitleComponentState_shows_finished_runs(state))
			asprintf(&attstr, "%d / %d", finished, attempts);
		else
			asprintf(&attstr, "%d", attempts);
	} else {
		attstr = calloc(1, 1); //single null byte
	}

	int x = WIDTH - strlen(attstr);
	tb_put_string(x - 2, *line, "  ", 0, 0);
	tb_put_string(x, *line, attstr, 0, 0);
	*line += 1;

	free(attstr);
	free(line2);
	free(line1);
}
