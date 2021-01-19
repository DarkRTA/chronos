#include "render.h"

#include <assert.h>
#include <stdio.h>
#include <string.h>

#include <termbox/termbox.h>

#include "components/components.h"

#define GET_AND_RENDER_STATE(l, s, i, t)                                       \
	render_##t(LayoutState_component_as_##t(s, i), l)

static void
render_component(LayoutStateRef state, size_t i, const char *type, int *line)
{
	if (!strcmp(type, "BlankSpace"))
		GET_AND_RENDER_STATE(line, state, i, blank_space);
	if (!strcmp(type, "DetailedTimer"))
		GET_AND_RENDER_STATE(line, state, i, detailed_timer);
	if (!strcmp(type, "KeyValue"))
		GET_AND_RENDER_STATE(line, state, i, key_value);
	if (!strcmp(type, "Separator"))
		GET_AND_RENDER_STATE(line, state, i, separator);
	if (!strcmp(type, "Splits"))
		GET_AND_RENDER_STATE(line, state, i, splits);
	if (!strcmp(type, "Text"))
		GET_AND_RENDER_STATE(line, state, i, text);
	if (!strcmp(type, "Timer"))
		GET_AND_RENDER_STATE(line, state, i, timer);
	if (!strcmp(type, "Title"))
		GET_AND_RENDER_STATE(line, state, i, title);
}

#undef GET_AND_RENDER_STATE

void render(LayoutStateRef state)
{
	tb_clear();
	size_t len = LayoutState_len(state);
	int line = 0;
	for (size_t i = 0; i < len; i++) {
		render_component(
			state,
			i,
			LayoutState_component_type(state, i),
			&line);
	}
	tb_present();
}
