#ifndef TTYSPLIT_TTYSPLIT_H
#define TTYSPLIT_TTYSPLIT_H

#include "livesplit_core.h"
#define WIDTH (30)

#define json_value_to_string(obj, key) \
	json_string_value(json_object_get((obj), (key)))

#define CHK_NULL(expr) if ((expr) == NULL)
#define CHK_ERR(expr) if ((expr) < 0)

extern Timer timer;
extern Layout layout;
#endif
