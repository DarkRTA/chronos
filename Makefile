CC=gcc
CFLAGS=-Isrc -Ilivesplit-core -I. -O2
LDFLAGS=-L livesplit-core -llivesplit_core -lm -lpthread -ldl -lncursesw -ljansson

DEPS = $(shell find src -type f -name *.h) \
	   livesplit-core/livesplit_core.h
OBJ = $(patsubst %.c,obj/%.o,$(shell find src -type f -name *.c))

.PHONY: clean all tags
.SUFFIXES:

all: darksplit

clean: 
	@rm -rfv obj darksplit

tags:
	@ctags --totals=yes --c-kinds=+defghlmpstuvxz -R \
		livesplit-core/livesplit_core.h \
		src/**

obj/%.o: %.c $(DEPS)
	@echo "CC   "$<
	@mkdir -p $(shell dirname $@)
	@$(CC) -c -o $@ $< $(CFLAGS)

darksplit: $(OBJ) livesplit-core/liblivesplit_core.a
	@echo "LINK "$@
	@$(CC) -o $@ $(OBJ) $(LDFLAGS)

livesplit-core/liblivesplit_core.a:
	@cd livesplit-core/livesplit-core; \
	git reset --hard; \
	git apply --3way ../../livesplit_core.patch; \
	cargo build --release -p staticlib; \
	cp target/release/liblivesplit_core.a ../liblivesplit_core.a; \

livesplit-core/livesplit_core.h:
	@cd livesplit-core/livesplit-core/capi/bind_gen; \
	cargo run; \
	cp ../bindings/livesplit_core.h ../../../livesplit_core.h; \
