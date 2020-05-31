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

static void load_splits(command_t *cmd);
static void load_layout(command_t *cmd);

static inline int process_hotkey(const char key, const char *path, TimerRefMut timer,
		    HotkeySystemRefMut hotkey_system);

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

	setlocale(LC_ALL, "");
	initscr();
	cbreak();
	curs_set(0);
	timeout(20);
	start_color();
	use_default_colors();

	config_init(&hotkey_system, SharedTimer_share(stimer));

	LayoutState state = LayoutState_new();
	for (;;) {
		int y, x;
		getmaxyx(stdscr, y, x);
		WIDTH = MIN(x, 50);
		TimerWriteLock lock = SharedTimer_write(stimer);
		TimerRefMut timer = TimerWriteLock_timer(lock);
		char key = getch();
		int brk = process_hotkey(key, path, timer, hotkey_system);
		Layout_update_state(layout, state, timer);
		render(state);
		TimerWriteLock_drop(lock);
		refresh();
		if (brk)
			break;
	}
	LayoutState_drop(state);
	endwin();

	SharedTimer_drop(stimer);
	Layout_drop(layout);
	HotkeySystem_drop(hotkey_system);
	command_free(&cmd);
}

static inline int process_hotkey(const char key, const char *path, TimerRefMut timer,
		    HotkeySystemRefMut hotkey_system)
{
	if (key == CONFIG.local_hk.hks_enable)
		HotkeySystem_activate(hotkey_system);
	if (key == CONFIG.local_hk.hks_disable)
		HotkeySystem_deactivate(hotkey_system);
	if (key == CONFIG.local_hk.split)
		Timer_split_or_start(timer);
	if (key == CONFIG.local_hk.reset)
		Timer_reset(timer, true);
	if (key == CONFIG.local_hk.reset_nosave)
		Timer_reset(timer, false);
	if (key == CONFIG.local_hk.undo)
		Timer_undo_split(timer);
	if (key == CONFIG.local_hk.skip)
		Timer_skip_split(timer);
	if (key == CONFIG.local_hk.pause)
		Timer_toggle_pause(timer);
	if (key == CONFIG.local_hk.undo_pause)
		Timer_undo_all_pauses(timer);
	if (key == CONFIG.local_hk.prev)
		Timer_switch_to_previous_comparison(timer);
	if (key == CONFIG.local_hk.next)
		Timer_switch_to_next_comparison(timer);
	if (key == CONFIG.local_hk.save) {
		const char *str = Timer_save_as_lss(timer);
		FILE *f = fopen(path, "w");
		fwrite(str, strlen(str), 1, f);
		fclose(f);
	}
	if (key == CONFIG.local_hk.quit)
		return 1;
	return 0;
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
