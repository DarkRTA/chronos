#pragma once
#include <livesplit_core.h>

uint16_t config_get_semantic_color(const char *color);
void config_init();
int config_ini_handler(
	void *d, const char *section, const char *name, const char *value);

struct Config {
	struct {
		char hks_enable;
		char hks_disable;
		char split;
		char reset;
		char reset_nosave;
		char undo;
		char skip;
		char pause;
		char undo_pause;
		char prev;
		char next;
		char save;
		char quit;
	} local_hk;

	HotkeyConfig global_hk;

	struct {
		uint16_t default_color;
		uint16_t ahead_gaining_time;
		uint16_t ahead_losing_time;
		uint16_t behind_losing_time;
		uint16_t behind_gaining_time;
		uint16_t best_segment;
		uint16_t not_running;
		uint16_t paused;
		uint16_t personal_best;
	} color;
};

extern struct Config CONFIG;
