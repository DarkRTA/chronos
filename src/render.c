#include "render.h"

#include <assert.h>
#include <stdio.h>
#include <string.h>

#include <ncurses.h>

#include "components/components.h"


#define GET_AND_RENDER_STATE(s, i, t) \
	render_##t(LayoutState_component_as_##t(s, i))

static void render_component(LayoutStateRef state, size_t i, 
			     const char *type)
{
	if (strcmp(type, "BlankSpace") == 0)
		GET_AND_RENDER_STATE(state, i, blank_space);
	if (strcmp(type, "DetailedTimer") == 0)
		GET_AND_RENDER_STATE(state, i, detailed_timer);
	if (strcmp(type, "KeyValue") == 0)
		GET_AND_RENDER_STATE(state, i, key_value);
	if (strcmp(type, "Separator") == 0)
		GET_AND_RENDER_STATE(state, i, separator);
	if (strcmp(type, "Splits") == 0)
		GET_AND_RENDER_STATE(state, i, splits);
	if (strcmp(type, "Text") == 0)
		GET_AND_RENDER_STATE(state, i, text);
	if (strcmp(type, "Timer") == 0)
		GET_AND_RENDER_STATE(state, i, timer);
	if (strcmp(type, "Title") == 0)
		GET_AND_RENDER_STATE(state, i, title);
}

#undef GET_AND_RENDER_STATE

void render(LayoutState state)
{
	erase();
	size_t len = LayoutState_len(state);
	for (size_t i = 0; i < len; i++) {
		render_component(state, i, LayoutState_component_type(state, i));
	}
	LayoutState_drop(state);
	refresh();
}
