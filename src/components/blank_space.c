#include "components/components.h"

#include <string.h>
#include <ncurses.h>

#include <cjson/cJSON.h>

#include "darksplit.h"

void render_blank_space(cJSON *data)
{
	int y, x;
	getyx(stdscr, y, x);
	move(++y, 0);
}
