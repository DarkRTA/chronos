#define _GNU_SOURCE
#include "tb_extras.h"

#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <termbox/termbox.h>

// TODO: use utf8
void tb_put_string(int x, int y, const char *str, uint16_t fg, uint16_t bg)
{
	size_t len = strlen(str);

	for (int i = 0; i < len; i++) {
		tb_change_cell(x + i, y, str[i], fg, bg);
	}
}

void tb_printf(int x, int y, int fg, int bg, char *fmt, ...)
{
	va_list args;
	va_start(args, fmt);
	char *str;
	vasprintf(&str, fmt, args);
	va_end(args);
	tb_put_string(x, y, str, fg, bg);
	free(str);
}
