#define _GNU_SOURCE
#include "components/components.h"

#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <termbox/termbox.h>

#include <livesplit_core.h>

#include "chronos.h"
#include "config.h"
#include "tb_extras.h"

void render_timer(TimerComponentStateRef state, int *line)
{
	char *time_str = strdup(TimerComponentState_time(state));
	char *time_frac = strdup(TimerComponentState_fraction(state));
	int color =
		config_get_semantic_color(TimerComponentState_semantic_color(state));

	char *str;
	int x = WIDTH - asprintf(&str, "%s%s", time_str, time_frac);
	tb_put_string(x, *line, str, color, 0);
	*line += 1;

	free(str);
	free(time_str);
	free(time_frac);
}
