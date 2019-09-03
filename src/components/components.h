#ifndef TTYSPLIT_COMPONENTS_COMPONENTS_H
#define TTYSPLIT_COMPONENTS_COMPONENTS_H
#include <jansson.h>
void render_blank_space(json_t *data);
void render_current_comparison(json_t *data);
void render_current_pace(json_t *data);
void render_delta(json_t *data);
void render_detailed_timer(json_t *data);
void render_pb_chance(json_t *data);
void render_possible_time_save(json_t *data);
void render_previous_segment(json_t *data);
void render_separator(json_t *data);
void render_splits(json_t *data);
void render_sum_of_best(json_t *data);
void render_total_playtime(json_t *data);
void render_timer(json_t *data);
void render_title(json_t *data);
#endif
