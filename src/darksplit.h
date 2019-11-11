#pragma once

#include "livesplit_core.h"

#define json_obj_string(obj, key)                                              \
	json_string_value(json_object_get((obj), (key)))
#define json_obj_bool(obj, key)                                                \
	json_boolean_value(json_object_get((obj), (key)))

#define MAX(a, b) ((a) > (b) ? (a) : (b))
#define MIN(a, b) ((a) < (b) ? (a) : (b))

extern int WIDTH;
