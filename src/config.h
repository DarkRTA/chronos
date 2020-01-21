#pragma once
#include <livesplit_core.h>

int get_semantic_color(const char *color);

void config_init(/*out*/ HotkeySystem *hk_sys, SharedTimer stimer);

struct Color {
	char is_rgb;
	char id;
	char r;
	char g;
	char b;
};

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
};

extern struct Config CONFIG;
