#include "components/components.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <termbox/termbox.h>

#include <livesplit_core.h>

#include "tb_extras.h"
#include "chronos.h"
#include "config.h"

void render_key_value(KeyValueComponentStateRef state, int *line)
{
	char *key = strdup(KeyValueComponentState_key(state));
	char *value = strdup(KeyValueComponentState_value(state));

	int color =
		config_get_semantic_color(KeyValueComponentState_semantic_color(state));

	int x = WIDTH - strlen(value);

	tb_put_string(0, *line, key, 0, 0);
	tb_put_string(x - 2, *line, "  ", 0, 0);
	tb_put_string(x, *line, value, color, 0);

	*line += 1;

	free(key);
	free(value);
}
