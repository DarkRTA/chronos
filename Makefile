CC=gcc
CFLAGS=-Isrc -Iinclude -I. -O2
LDFLAGS=-L libs -llivesplit_core -lm -lpthread -ldl -lncursesw -ljansson

DEPS = $(shell find src -type f -name *.h) \
	   include/livesplit_core.h
OBJ = $(patsubst %.c,%.o,$(shell find src -type f -name *.c))

.PHONY: clean all tags
.SUFFIXES:

all: darksplit

clean: 
	@rm -fv $(OBJ) darksplit

tags:
	@ctags --totals=yes --c-kinds=+defghlmpstuvxz -R include/** src/**

%.o: %.c $(DEPS)
	@echo "CC   "$<
	@$(CC) -c -o $@ $< $(CFLAGS)

darksplit: $(OBJ) libs/liblivesplit_core.a
	@echo "LINK "$@
	@$(CC) -o $@ $(OBJ) $(LDFLAGS)

libs/liblivesplit_core.a:
	@mkdir -p libs
	@cd livesplit-core; \
	git reset --hard; \
	git apply --3way ../livesplit_core.patch; \
	cargo build --release -p staticlib; \
	cp target/release/liblivesplit_core.a ../libs/liblivesplit_core.a; \

include/livesplit_core.h:
	@mkdir -p include
	@cd livesplit-core/capi/bind_gen; \
	cargo run; \
	cp ../bindings/livesplit_core.h ../../../include/livesplit_core.h; \
