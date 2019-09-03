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
Timer timer;
Layout layout;


static void loop(char *path)
{
	initscr();
	cbreak();
	curs_set(0);
	timeout(17);
	start_color();
	use_default_colors();
	init_semantic_colors();

	const char *str;
	FILE *f;
	for (;;) {
		char key = getch();
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
		errno =	EINVAL;
		return NULL;
	}

	close(fd);
	return ParseRunResult_unwrap(maybe_run);
}

static Layout load_layout(const char *path)
{
	FILE *f;
	if (path != NULL) {
		f = fopen(path, "rb");
		CHK_NULL(f)
			return NULL;

		CHK_ERR(fseek(f, 0, SEEK_END)) 
			goto fail;

		long fsize = ftell(f);

		CHK_ERR(fseek(f, 0, SEEK_SET))
			goto fail;

		char *string = malloc(fsize + 1);
		assert(string != NULL);

		CHK_ERR(fread(string, 1, fsize, f)) {
			free(string);
			goto fail;
		}

		fclose(f);

		string[fsize] = 0;

		Layout ret = Layout_parse_json(string);
		free(string);
		return ret;
	}
		return Layout_default_layout();
fail:
		fclose(f);
		return Layout_default_layout();
}


int main(int argc, char *argv[])
{
	setlocale(LC_ALL,"");

	if (argc < 2) {
		printf("ttysplit - a command line speedrun timer\n");
		printf("usage:\n    %s <splits> [layout]\n", argv[0]);
		return 1;
	}

	Run run;
	CHK_NULL(run = load_splits(argv[1])) {
		printf("Error loading splits");
		return 1;
	}

	if (argc > 2) 
		layout = load_layout(argv[2]);
	else
		layout = load_layout(NULL);
	
	CHK_NULL(layout) {
		printf("Error loading layout");
		return 1;
	}

	timer = Timer_new(run);
	
	loop(argv[1]);

	Timer_drop(timer);
	Layout_drop(layout);

}
