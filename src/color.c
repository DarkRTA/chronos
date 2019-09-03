#include "color.h"

#include <string.h>

#include <ncurses.h>
void init_semantic_colors() {
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

int get_semantic_color(const char *color) {
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
