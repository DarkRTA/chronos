#pragma once
#include <livesplit_core.h>

extern const char *GLOBAL_HOTKEYS;
void process_hotkey(const char key, const char *path, TimerRefMut timer,
		    HotkeySystemRefMut hotkey_system);
void init_semantic_colors();
int get_semantic_color(const char *color);
