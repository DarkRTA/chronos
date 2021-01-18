CC      := clang
CFLAGS  := -Isrc -Ideps -Ilivesplit-core -I. -Wall -MMD
LDFLAGS := -Llivesplit-core \
		  -llivesplit_core \
		  -lm \
		  -lpthread \
		  -ldl \
		  -lncursesw
OBJ = $(patsubst %.c,obj/%.o,$(shell find src deps -type f -name *.c))
DEPS = $(patsubst %.o,%.d,$(OBJ))

.PHONY: clean all tags
.SUFFIXES:
Q=@

all: darksplit

clean: 
	@rm -rfv obj darksplit

tags:
	@ctags --totals=yes --c-kinds=+defghlmpstuvxz -R \
		livesplit-core/livesplit_core.h \
		src/** \
		deps/**

obj/%.o: %.c obj/%.d livesplit-core/livesplit_core.h
	@echo "CC   "$<
	@mkdir -p $(@D)
	$Q$(CC) -c -o $@ $< $(CFLAGS)

$(DEPS): ;
include $(DEPS)

darksplit: $(OBJ) livesplit-core/liblivesplit_core.a
	@echo "LINK "$@
	$Q$(CC) -o $@ $(OBJ) $(LDFLAGS)

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
