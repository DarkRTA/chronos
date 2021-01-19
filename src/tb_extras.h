#pragma once
#include <stdint.h>
void tb_put_string(int x, int y, const char *str, uint16_t fg, uint16_t bg);
void tb_printf(int x, int y, int fg, int bg, char *fmt, ...);
