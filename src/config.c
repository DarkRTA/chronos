#include "config.h"

#include <stdlib.h>
#include <stdio.h>
#include <string.h>

#include "chronos.h"
#include <livesplit_core.h>
#include <termbox/termbox.h>

// I know this is awful. Don't complain to me about it.

struct Config CONFIG;

void config_init()
{
	CONFIG.local_hk.hks_enable = 'o';
	CONFIG.local_hk.hks_disable = 'O';
	CONFIG.local_hk.split = ' ';
	CONFIG.local_hk.reset = 'x';
	CONFIG.local_hk.reset_nosave = 'X';
	CONFIG.local_hk.undo = 'c';
	CONFIG.local_hk.skip = 'v';
	CONFIG.local_hk.pause = 'b';
	CONFIG.local_hk.undo_pause = 'n';
	CONFIG.local_hk.prev = ',';
	CONFIG.local_hk.next = '.';
	CONFIG.local_hk.save = 's';
	CONFIG.local_hk.quit = 'q';

	CONFIG.global_hk = HotkeyConfig_new();
	//Split
	HotkeyConfig_set_value(
		CONFIG.global_hk,
		0,
		SettingValue_from_string("NumPad0"));
	//Reset
	HotkeyConfig_set_value(
		CONFIG.global_hk,
		1,
		SettingValue_from_string("NumPad1"));
	//Undo
	HotkeyConfig_set_value(
		CONFIG.global_hk,
		2,
		SettingValue_from_string("NumPad8"));
	//Skip
	HotkeyConfig_set_value(
		CONFIG.global_hk,
		3,
		SettingValue_from_string("NumPad2"));
	//Pause
	HotkeyConfig_set_value(
		CONFIG.global_hk,
		4,
		SettingValue_from_string("NumPad5"));
	//Undo All Pauses
	HotkeyConfig_set_value(
		CONFIG.global_hk,
		5,
		SettingValue_from_string("NumPad2"));
	//Previous Comparison
	HotkeyConfig_set_value(
		CONFIG.global_hk,
		6,
		SettingValue_from_string("NumPad4"));
	//Next Comparison
	HotkeyConfig_set_value(
		CONFIG.global_hk,
		7,
		SettingValue_from_string("NumPad6"));
	//Toggle timing method
	HotkeyConfig_set_value(
		CONFIG.global_hk,
		8,
		SettingValue_from_string("NumPad9"));

	CONFIG.color.default_color = 0;
	CONFIG.color.ahead_gaining_time = 41;
	CONFIG.color.ahead_losing_time = 78;
	CONFIG.color.behind_losing_time = 160;
	CONFIG.color.behind_gaining_time = 167;
	CONFIG.color.best_segment = 220;
	CONFIG.color.not_running = 145;
	CONFIG.color.paused = 102;
	CONFIG.color.personal_best = 39;
}

#define LOCAL_HK(k)                                                            \
	do {                                                                   \
		if (!strcmp(section, "local_hotkeys") && !strcmp(name, #k)) {  \
			CONFIG.local_hk.k = value[0];                          \
		}                                                              \
	} while (0)

#define GLOBAL_HK(i, k)                                                        \
	do {                                                                   \
		if (!strcmp(section, "global_hotkeys") && !strcmp(name, #k)) { \
			HotkeyConfig_set_value(                                \
				CONFIG.global_hk,                              \
				i,                                             \
				SettingValue_from_string(value));              \
		}                                                              \
	} while (0)

#define COLOR(k)                                                               \
	do {                                                                   \
		if (!strcmp(section, "colors") && !strcmp(name, #k)) {         \
			CONFIG.color.k = atoi(value);                          \
		}                                                              \
	} while (0)

int config_ini_handler(
	void *d, const char *section, const char *name, const char *value)
{
	//empty values should be treated as a single space
	if (!strcmp("", value)) {
		value = " ";
	}

	LOCAL_HK(hks_enable);
	LOCAL_HK(hks_disable);
	LOCAL_HK(split);
	LOCAL_HK(reset);
	LOCAL_HK(reset_nosave);
	LOCAL_HK(undo);
	LOCAL_HK(skip);
	LOCAL_HK(pause);
	LOCAL_HK(undo_pause);
	LOCAL_HK(next);
	LOCAL_HK(prev);
	LOCAL_HK(save);
	LOCAL_HK(quit);

	GLOBAL_HK(0, split);
	GLOBAL_HK(1, reset);
	GLOBAL_HK(2, undo);
	GLOBAL_HK(3, skip);
	GLOBAL_HK(4, pause);
	GLOBAL_HK(5, undo_pause);
	GLOBAL_HK(6, next);
	GLOBAL_HK(7, prev);
	GLOBAL_HK(8, timing_method);

	COLOR(default_color);
	COLOR(ahead_gaining_time);
	COLOR(ahead_losing_time);
	COLOR(behind_losing_time);
	COLOR(behind_gaining_time);
	COLOR(best_segment);
	COLOR(not_running);
	COLOR(paused);
	COLOR(personal_best);

	return 0;
}

#undef LOCAL_HK
#undef GLOBAL_HK
#undef COLOR

uint16_t config_get_semantic_color(const char *color)
{
	if (!strcmp(color, "Default"))
		return CONFIG.color.default_color;
	if (!strcmp(color, "AheadGainingTime"))
		return CONFIG.color.ahead_gaining_time;
	if (!strcmp(color, "AheadLosingTime"))
		return CONFIG.color.ahead_losing_time;
	if (!strcmp(color, "BehindLosingTime"))
		return CONFIG.color.behind_losing_time;
	if (!strcmp(color, "BehindGainingTime"))
		return CONFIG.color.behind_gaining_time;
	if (!strcmp(color, "BestSegment"))
		return CONFIG.color.best_segment;
	if (!strcmp(color, "NotRunning"))
		return CONFIG.color.not_running;
	if (!strcmp(color, "Paused"))
		return CONFIG.color.paused;
	if (!strcmp(color, "PersonalBest"))
		return CONFIG.color.personal_best;
	return 0;
}
