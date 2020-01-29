#include "components/components.h"

#include <string.h>
#include <ncurses.h>

#include "livesplit_core.h"

#include "darksplit.h"

void render_blank_space(BlankSpaceComponentStateRef state)
{
	int y, x;
	getyx(stdscr, y, x);
	move(++y, 0);
}
