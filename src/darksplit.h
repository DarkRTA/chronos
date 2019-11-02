#ifndef DARKSPLIT_DARKSPLIT_H
#define DARKSPLIT_DARKSPLIT_H

#include "livesplit_core.h"

#define json_obj_string(obj, key)                                              \
	json_string_value(json_object_get((obj), (key)))
#define json_obj_bool(obj, key) json_string_value(json_object_get((obj), (key)))

#define CHK_NULL(expr) if ((expr) == NULL)
#define CHK_ERR(expr) if ((expr) < 0)
#define MAX(a, b) ((a) > (b) ? (a) : (b))
#define MIN(a, b) ((a) < (b) ? (a) : (b))

extern int WIDTH;
#endif
