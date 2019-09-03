CC=clang
CFLAGS=-Isrc -Iinclude -I. -O2
LDFLAGS=-L libs -llivesplit_core -lm -lpthread -ldl -lncursesw -ljansson

DEPS = include/livesplit_core.h \
	   src/components/components.h \
	   src/color.h \
	   src/render.h \
	   src/darksplit.h \

OBJ = src/color.o \
	  src/darksplit.o \
	  src/render.o \
	  src/components/blank_space.o \
	  src/components/current_comparison.o \
	  src/components/current_pace.o \
	  src/components/delta.o \
	  src/components/pb_chance.o \
	  src/components/possible_time_save.o \
	  src/components/previous_segment.o \
	  src/components/separator.o \
	  src/components/splits.o \
	  src/components/sum_of_best.o \
	  src/components/timer.o \
	  src/components/title.o \
	  src/components/total_playtime.o

.PHONY: clean all
.SUFFIXES:

all: darksplit

clean: 
	@rm -v $(OBJ) darksplit

%.o: %.c $(DEPS)
	@echo "CC   "$<
	@$(CC) -c -o $@ $< $(CFLAGS)

darksplit: $(OBJ) libs/liblivesplit_core.a
	@echo "LINK "$@
	@$(CC) -o $@ $(OBJ) $(LDFLAGS)

libs/liblivesplit_core.a:
	mkdir -p libs
	cd livesplit-core; \
	cargo build --release -p staticlib; \
	cp target/release/liblivesplit_core.a ../libs/liblivesplit_core.a; \

include/livesplit_core.h:
	mkdir -p include
	cd livesplit-core/capi/bind_gen; \
	cargo run; \
	cp ../bindings/livesplit_core.h ../../../include/livesplit_core.h; \
