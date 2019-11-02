#include "darksplit.h"

#include <fcntl.h>
#include <locale.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>

#include <ncurses.h>
#include "livesplit_core.h"

#include "render.h"
#include "config.h"

int WIDTH;

static SharedTimer stimer;
static Layout layout;
static HotkeySystem hotkey_system;

static void loop(char *path)
{
	initscr();
	cbreak();
	curs_set(0);
	timeout(20);
	start_color();
	use_default_colors();
	init_semantic_colors();

	for (;;) {
		int y, x;
		getmaxyx(stdscr, y, x);
		WIDTH = MIN(x, 50);
		TimerWriteLock lock = SharedTimer_write(stimer);
		TimerRefMut timer = TimerWriteLock_timer(lock);
		char key = getch();
		process_hotkey(key, path, timer, hotkey_system);
		render(Layout_state_as_json(layout, timer));
		TimerWriteLock_drop(lock);
		refresh();
	}
}

static Run load_splits(const char *path)
{
	int fd = open(path, O_RDONLY);
	if (fd < 0)
		return NULL;

	ParseRunResult maybe_run = Run_parse_file_handle(fd, path, 0);
	close(fd);

	if (!ParseRunResult_parsed_successfully(maybe_run))
		return NULL;

	return ParseRunResult_unwrap(maybe_run);
}

static Layout load_layout(const char *path)
{
	int fd = open(path, O_RDONLY);
	if (fd < 0)
		return NULL;

	Layout layout = Layout_parse_file_handle(fd);

	close(fd);

	return layout;
}

int main(int argc, char *argv[])
{
	setlocale(LC_ALL, "");

	if (argc < 2) {
		printf("darksplit - a command line speedrun timer\n");
		printf("usage:\n    %s <splits> [layout]\n", argv[0]);
		return 1;
	}

	layout = (argc > 2) ? load_layout(argv[2]) : Layout_default_layout();
	
	CHK_NULL(layout) {
		printf("Error loading layout");
		return 1;
	}

	Run run;
	CHK_NULL(run = load_splits(argv[1])) {
		printf("Error loading splits");
		return 1;
	}

	stimer = Timer_into_shared(Timer_new(run));

	hotkey_system = HotkeySystem_with_config(
		SharedTimer_share(stimer),
		HotkeyConfig_parse_json(GLOBAL_HOTKEYS));

	loop(argv[1]);

	SharedTimer_drop(stimer);
	Layout_drop(layout);
	HotkeySystem_drop(hotkey_system);
}
