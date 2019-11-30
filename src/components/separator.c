#include "components/components.h"

#include <string.h>
#include <ncurses.h>

#include <cjson/cJSON.h>

#include "darksplit.h"

void render_separator(cJSON *data)
{
	int y, x;
	getyx(stdscr, y, x);
	//TODO: dynamically allocate and use memset
	char *str = "--------------------------------------------------------";
	printw("%*.*s", WIDTH, WIDTH, str);
	move(++y, 0);
}
