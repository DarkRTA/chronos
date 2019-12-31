#define _GNU_SOURCE
#include "components/components.h"

#include <stdlib.h>
#include <stdio.h>
#include <string.h>

#include <ncurses.h>
#include <cjson/cJSON.h>

#include "darksplit.h"

void render_title(cJSON *data)
{
	int y, x;
	getyx(stdscr, y, x);
	const char *str = cJSON_GetObjectItem(data, "line1")->valuestring;
	printw("%-*.*s", WIDTH, WIDTH, str);
	move(++y, 0);
	str = cJSON_GetObjectItem(data, "line2")->valuestring;
	printw("%-*.*s", WIDTH, WIDTH, str);

	cJSON *attempts = cJSON_GetObjectItem(data, "attempts");
	cJSON *finished = cJSON_GetObjectItem(data, "finished_runs");
	char *attstr = NULL;

	if (attempts != NULL) {
		if (finished != NULL)
			asprintf(&attstr, "%d / %d", finished->valueint,
				 attempts->valueint);
		else
			asprintf(&attstr, "%d", attempts->valueint);
	} else {
		attstr = calloc(1, 1); //single null byte
	}
	mvprintw(y, WIDTH - strlen(attstr) - 2, "  %s", attstr);
	free(attstr);
	move(++y, 0);
}
