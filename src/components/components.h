#pragma once
#include <jansson.h>
void render_blank_space(json_t *data);
void render_detailed_timer(json_t *data);
void render_key_value(json_t *data);
void render_separator(json_t *data);
void render_splits(json_t *data);
void render_timer(json_t *data);
void render_text(json_t *data);
void render_title(json_t *data);
