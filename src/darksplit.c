#include "darksplit.h"

#include <fcntl.h>
#include <locale.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#include <ncurses.h>

#include <livesplit_core.h>
#include <commander/commander.h>

#include "render.h"
#include "config.h"

int WIDTH;

static SharedTimer stimer = NULL;
static Layout layout      = NULL;
static HotkeySystem hotkey_system;

static const char *path = NULL;

static void loop()
{
	setlocale(LC_ALL, "");
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

static void load_splits(command_t *cmd)
{
	path = cmd->arg;
	int fd = open(cmd->arg, O_RDONLY);
	if (fd < 0) {
		puts("Error opening splits file\n");
		exit(1);
	}

	ParseRunResult maybe_run = Run_parse_file_handle(fd, cmd->arg, 0);
	close(fd);

	if (!ParseRunResult_parsed_successfully(maybe_run)) {
		puts("Error parsing splits file\n");
		exit(1);
	}

	Run run = ParseRunResult_unwrap(maybe_run);

	stimer = Timer_into_shared(Timer_new(run));
}

static void load_layout(command_t *cmd)
{
	int fd = open(cmd->arg, O_RDONLY);
	if (fd < 0) {
		puts("Error opening layout file\n");
		exit(1);
	}

	layout = Layout_parse_file_handle(fd);
	if (layout == NULL) {
		puts("Error parsing layout file\n");
		exit(1);
	}

	close(fd);
}


int main(int argc, char *argv[])
{
	command_t cmd;
	command_init(&cmd, argv[0], __DATE__ " " __TIME__);
	command_option(&cmd, "-l", "--layout <arg>", "layout file to use",
			load_layout);
	command_option(&cmd, "-s", "--splits <arg>", "split file to use",
			load_splits);
	command_parse(&cmd, argc, argv);

	if (stimer == NULL) {
		puts("No splits loaded. See \"darksplit --help\"");
		exit(1);
	}

	if (layout == NULL) {
		layout = Layout_default_layout();
	}

	hotkey_system = HotkeySystem_with_config(
		SharedTimer_share(stimer),
		HotkeyConfig_parse_json(GLOBAL_HOTKEYS));

	loop();

	SharedTimer_drop(stimer);
	Layout_drop(layout);
	HotkeySystem_drop(hotkey_system);
	command_free(&cmd);
}
