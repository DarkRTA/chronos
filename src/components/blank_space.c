#include "components/components.h"

#include <string.h>
#include <termbox/termbox.h>

#include "livesplit_core.h"

#include "chronos.h"

void render_blank_space(UNUSED BlankSpaceComponentStateRef state, int *line)
{
	*line += 1;
}
