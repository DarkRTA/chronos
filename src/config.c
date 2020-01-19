#include "config.h"

#include <string.h>
#include <ncurses.h>

#include "darksplit.h"
#include <livesplit_core.h>

//hotkeys
const char *GLOBAL_HOTKEYS =
"{"
	"\"split\":			null,"
	"\"reset\":			null,"
	"\"undo\":			null,"
	"\"skip\":			null,"
	"\"pause\":			null,"
	"\"undo_all_pauses\":		null,"
	"\"previous_comparison\":	null,"
	"\"next_comparison\":		null,"
	"\"toggle_timing_method\":	null"
"}";


int process_hotkey(const char key, const char *path, TimerRefMut timer,
		    HotkeySystemRefMut hotkey_system)
{
	switch (key) {
	case 'o':
		HotkeySystem_activate(hotkey_system);
		break;
	case 'O':
		HotkeySystem_deactivate(hotkey_system);
		break;
	case ' ':
		Timer_split_or_start(timer);
		break;
	case 'x':
		Timer_reset(timer, true);
		break;
	case 'X':
		Timer_reset(timer, false);
		break;
	case 'c':
		Timer_undo_split(timer);
		break;
	case 'v':
		Timer_skip_split(timer);
		break;
	case 'b':
		Timer_toggle_pause(timer);
		break;
	case 'n':
		Timer_undo_all_pauses(timer);
		break;
	case ',':
		Timer_switch_to_previous_comparison(timer);
		break;
	case '.':
		Timer_switch_to_next_comparison(timer);
		break;
	case 's':; //empty statement here to allow declaration
		const char *str = Timer_save_as_lss(timer);
		FILE *f = fopen(path, "w");
		fwrite(str, strlen(str), 1, f);
		fclose(f);
		break;
	case 'q':
		return 1;
		break;
	}
	return 0;
}

// colors
static void init_color_hex(int id, int r, int g, int b)
{
	r = r * 1000 / 255;
	g = g * 1000 / 255;
	b = b * 1000 / 255;
	init_color(id, r, g, b);
}

void init_semantic_colors()
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
		return 0;
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
