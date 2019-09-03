#ifndef TTYSPLIT_TTYSPLIT_H
#define TTYSPLIT_TTYSPLIT_H

#include "livesplit_core.h"

#define json_value_to_string(obj, key) \
	json_string_value(json_object_get((obj), (key)))

#define CHK_NULL(expr) if ((expr) == NULL)
#define CHK_ERR(expr) if ((expr) < 0)
#define MAX(a, b) ((a) > (b) ? (a) : (b))
#define MIN(a, b) ((a) < (b) ? (a) : (b))

extern int WIDTH;
extern Timer timer;
extern Layout layout;
#endif
