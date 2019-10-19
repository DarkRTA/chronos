#include "darksplit.h"

#include <assert.h>
#include <errno.h>
#include <fcntl.h>
#include <locale.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#include <ncurses.h>
#include "livesplit_core.h"

#include "render.h"

#include "color.h"
int WIDTH;

static SharedTimer stimer;
static Layout layout;
static HotkeySystem hotkey_system;

//I should come up with a better way to configure these.
//Make sure you dont conflict with the local hotkeys defined
//below in the switch statement.
static char *global_hotkeys = 
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

static void loop(char *path)
{
	initscr();
	cbreak();
	curs_set(0);
	timeout(20);
	start_color();
	use_default_colors();
	init_semantic_colors();

	const char *str;
	char key;
	int y, x;
	FILE *f;
	TimerWriteLock lock;
	TimerRefMut timer;
	for (;;) {
		getmaxyx(stdscr, y, x);
		WIDTH = MIN(x, 50);
		key = getch();
		lock =SharedTimer_write(stimer);
		timer = TimerWriteLock_timer(lock);
		switch (key) {
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
		case 's':
			str = Timer_save_as_lss(timer);
			f = fopen(path, "w");
			fwrite(str, strlen(str), 1, f);
			fclose(f);
			break;
		case 'q':
			endwin();
			return;
			break;
		}
		render(Layout_state_as_json(layout, timer));
		TimerWriteLock_drop(lock);
		refresh();
	}

}

static Run load_splits(const char *path)
{
	int fd = open(path, O_RDONLY);
	if (fd < 0) {
		return NULL;
	}

	ParseRunResult maybe_run = Run_parse_file_handle(fd, path, 0);
	if(!ParseRunResult_parsed_successfully(maybe_run)) {
		close(fd);
		return NULL;
	}

	close(fd);
	return ParseRunResult_unwrap(maybe_run);
}

static Layout load_layout(const char *path)
{
	FILE *f;
	CHK_NULL(f = fopen(path, "rb"))
		return NULL;

	fseek(f, 0, SEEK_END);
	long fsize = ftell(f);
	fseek(f, 0, SEEK_SET);

	char *string = malloc(fsize + 1);

	CHK_ERR(fread(string, 1, fsize, f)) {
		free(string);
		goto fail;
	}

	fclose(f);

	string[fsize] = 0;

	Layout ret = Layout_parse_json(string);
	free(string);
	return ret;
fail:
	fclose(f);
	return NULL;
}


int main(int argc, char *argv[])
{
	setlocale(LC_ALL,"");

	if (argc < 2) {
		printf("darksplit - a command line speedrun timer\n");
		printf("usage:\n    %s <splits> [layout]\n", argv[0]);
		return 1;
	}

	if (argc > 2) 
		layout = load_layout(argv[2]);
	else
		layout = Layout_default_layout();
	
	CHK_NULL(layout) {
		printf("Error loading layout");
		return 1;
	}

	Run run;
	CHK_NULL(run = load_splits(argv[1])) {
		printf("Error loading splits");
		return 1;
	}


	Timer timer = Timer_new(run);
	stimer = Timer_into_shared(timer);

	HotkeyConfig hkconfig = HotkeyConfig_parse_json(global_hotkeys);

	hotkey_system = HotkeySystem_with_config(
			SharedTimer_share(stimer),
			hkconfig
	);

	HotkeySystem_activate(hotkey_system);

	loop(argv[1]);

	SharedTimer_drop(stimer);
	Layout_drop(layout);
	HotkeySystem_drop(hotkey_system);

}
