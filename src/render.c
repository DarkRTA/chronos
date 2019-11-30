#include "render.h"

#include <assert.h>
#include <stdio.h>
#include <string.h>

#include <ncurses.h>

#include <cjson/cJSON.h>

#include "components/components.h"

static void render_component(const char *name, cJSON *data)
{
	if (strcmp(name, "BlankSpace") == 0)
		render_blank_space(data);
	if (strcmp(name, "DetailedTimer") == 0)
		render_detailed_timer(data);
	if (strcmp(name, "KeyValue") == 0)
		render_key_value(data);
	if (strcmp(name, "Separator") == 0)
		render_separator(data);
	if (strcmp(name, "Splits") == 0)
		render_splits(data);
	if (strcmp(name, "Text") == 0)
		render_text(data);
	if (strcmp(name, "Timer") == 0)
		render_timer(data);
	if (strcmp(name, "Title") == 0)
		render_title(data);
}

void render(const char *json)
{
	erase();
	cJSON *tree = cJSON_Parse(json);

	cJSON *components = cJSON_GetObjectItem(tree, "components");
	cJSON *component;

	cJSON_ArrayForEach (component, components) {
		cJSON *data = component->child;
		render_component(data->string, data);
	}

	cJSON_Delete(tree);
	refresh();
}
