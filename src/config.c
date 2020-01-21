#include "config.h"

#include <string.h>
#include <ncurses.h>

#include "darksplit.h"
#include <livesplit_core.h>

struct Config CONFIG;

static void init_semantic_colors();

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
}

void config_init(/*out*/ HotkeySystem *hk_sys, SharedTimer stimer)
{
	config_default();
	init_semantic_colors();
	*hk_sys = HotkeySystem_with_config(stimer, CONFIG.global_hk);
	CONFIG.global_hk = NULL; //above function call consumed it
}

// colors
static void init_color_hex(int id, int r, int g, int b)
{
	r = r * 1000 / 255;
	g = g * 1000 / 255;
	b = b * 1000 / 255;
	init_color(id, r, g, b);
}

static void init_semantic_colors()
{
	// Default
	init_pair(1, -1, -1);
	// AheadGainingTime
	init_pair(2, 12, -1);
	// AheadLosingTime
	init_pair(3, 4, -1);
	// BehindLosingTime
	init_pair(4, 9, -1);
	// BehindGainingTime
	init_pair(5, 1, -1);
	// BestSegment
	init_pair(6, 10, -1);
	// NotRunning
	init_pair(7, -1, -1);
	// Paused
	init_pair(8, -1, -1);
	// PersonalBest
	init_pair(9, 10, -1);
}

int get_semantic_color(const char *color)
{
	if (strcmp(color, "Default") == 0)
		return COLOR_PAIR(1);
	if (strcmp(color, "AheadGainingTime") == 0)
		return COLOR_PAIR(2);
	if (strcmp(color, "AheadLosingTime") == 0)
		return COLOR_PAIR(3);
	if (strcmp(color, "BehindLosingTime") == 0)
		return COLOR_PAIR(4);
	if (strcmp(color, "BehindGainingTime") == 0)
		return COLOR_PAIR(5);
	if (strcmp(color, "BestSegment") == 0)
		return COLOR_PAIR(6);
	if (strcmp(color, "NotRunning") == 0)
		return COLOR_PAIR(7);
	if (strcmp(color, "Paused") == 0)
		return COLOR_PAIR(8);
	if (strcmp(color, "PersonalBest") == 0)
		return COLOR_PAIR(9);
	return 0;
}
