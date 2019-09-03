#include "components/components.h"

#include <string.h>
#include <ncurses.h>
#include <jansson.h>

#include "darksplit.h"

void render_blank_space(json_t *data)
{
	//yeah this is all it does
	int y, x;
	getyx(stdscr, y, x);
	move(++y, 0);
}
