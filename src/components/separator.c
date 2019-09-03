#include "components/components.h"

#include <string.h>
#include <ncurses.h>
#include <jansson.h>

#include "darksplit.h"

void render_separator(json_t *data)
{
	int y, x;
	getyx(stdscr, y, x);
	//this should be enough dashes for most use cases
	char *str = "--------------------------------------------------------";
	printw("%*.*s", WIDTH, WIDTH, str);
	move(++y, 0);
}
