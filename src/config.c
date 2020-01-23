#include "config.h"

#include <string.h>
#include <ncurses.h>

#include "darksplit.h"
#include <livesplit_core.h>


struct Config CONFIG;

static void init_semantic_colors();

static inline void config_color(size_t i, char rgb, short id, int r, int g,
				int b)
{
	CONFIG.colors[i].rgb = rgb;
	CONFIG.colors[i].id = id;
	CONFIG.colors[i].r = r;
	CONFIG.colors[i].g = g;
	CONFIG.colors[i].b = b;
}

static void config_default()
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

	// TODO: replace when HotkeyConfig_new() gets merged
	//hk = HotkeyConfig_new();
	HotkeyConfig hk = HotkeyConfig_parse_json("{}");
	//Split
	HotkeyConfig_set_value(hk, 0, SettingValue_from_string("NumPad0"));
	//Reset
	HotkeyConfig_set_value(hk, 1, SettingValue_from_string("NumPad1"));
	//Undo
	HotkeyConfig_set_value(hk, 2, SettingValue_from_string("NumPad8"));
	//Skip
	HotkeyConfig_set_value(hk, 3, SettingValue_from_string("NumPad2"));
	//Pause
	HotkeyConfig_set_value(hk, 4, SettingValue_from_string("NumPad5"));
	//Undo All Pauses
	HotkeyConfig_set_value(hk, 5, SettingValue_from_string("NumPad2"));
	//Previous Comparison
	HotkeyConfig_set_value(hk, 6, SettingValue_from_string("NumPad4"));
	//Next Comparison
	HotkeyConfig_set_value(hk, 7, SettingValue_from_string("NumPad6"));
	//Toggle timing method
	HotkeyConfig_set_value(hk, 8, SettingValue_from_string("NumPad9"));
	CONFIG.global_hk = hk;

	config_color(0, 0, -1, 0, 0, 0);
	config_color(1, 0, 12, 0, 0, 0);
	config_color(2, 0, 4, 0, 0, 0);
	config_color(3, 0, 9, 0, 0, 0);
	config_color(4, 0, 9, 0, 0, 0);
	config_color(5, 0, 10, 0, 0, 0);
	config_color(6, 0, -1, 0, 0, 0);
	config_color(7, 0, -1, 0, 0, 0);
	config_color(8, 0, 10, 0, 0, 0);
}

void config_init(/*out*/ HotkeySystem *hk_sys, SharedTimer stimer)
{
	config_default();
	init_semantic_colors();
	*hk_sys = HotkeySystem_with_config(stimer, CONFIG.global_hk);
	CONFIG.global_hk = NULL; //above function call consumed it
}

// colors
static void init_semantic_colors()
{
	for (int i = 0; i < 9; i++) {
		if (CONFIG.colors[i].rgb) {
			int r = CONFIG.colors[i].r * 1000 / 255;
			int g = CONFIG.colors[i].g * 1000 / 255;
			int b = CONFIG.colors[i].b * 1000 / 255;
			init_color(CONFIG.colors[i].id, r, g, b);
		}
		// ncurses color pairs start at 1
		init_pair(i + 1, CONFIG.colors[i].id, -1);
	}
}

int get_semantic_color(const char *color)
{
	if (!strcmp(color, "Default"))
		return COLOR_PAIR(1);
	if (!strcmp(color, "AheadGainingTime"))
		return COLOR_PAIR(2);
	if (!strcmp(color, "AheadLosingTime"))
		return COLOR_PAIR(3);
	if (!strcmp(color, "BehindLosingTime"))
		return COLOR_PAIR(4);
	if (!strcmp(color, "BehindGainingTime"))
		return COLOR_PAIR(5);
	if (!strcmp(color, "BestSegment"))
		return COLOR_PAIR(6);
	if (!strcmp(color, "NotRunning"))
		return COLOR_PAIR(7);
	if (!strcmp(color, "Paused"))
		return COLOR_PAIR(8);
	if (!strcmp(color, "PersonalBest"))
		return COLOR_PAIR(9);
	return 0;
}
