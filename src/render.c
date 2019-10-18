#include "render.h" 

#include <assert.h>
#include <stdio.h>
#include <string.h>

#include <ncurses.h>
#include <jansson.h>

#include "components/components.h"

static void render_component(const char *name, json_t *data) 
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
	json_error_t error;
	json_t *tree = json_loads(json, JSON_DECODE_INT_AS_REAL, &error);
	assert(tree != NULL);
	json_t *array = json_object_get(tree, "components");

	size_t index;
	json_t *component;

	json_array_foreach(array, index, component) {
		const char *name;
		json_t *data;
		json_object_foreach(component, name, data) {
			render_component(name, data);
		}
	}

	refresh();
	json_decref(tree);
}


