CC      := cc -flto
CFLAGS  := -Isrc -Ideps -Ilivesplit-core -I. -Wall -MMD -g -O2
LDFLAGS := -Llivesplit-core \
		  -llivesplit_core \
		  -lm \
		  -lpthread \
		  -ldl 

.PHONY: clean all tags
.SUFFIXES:
Q=@

all: chronos 

clean: 
	@rm -rfv obj chronos 

tags:
	@ctags --totals=yes --c-kinds=+defghlmpstuvxz -R \
		livesplit-core/livesplit_core.h \
		src/** \
		deps/**

obj/%.o: %.c obj/%.d livesplit-core/livesplit_core.h
	@echo "CC   "$<
	@mkdir -p $(@D)
	$Q$(CC) -c -o $@ $< $(CFLAGS) $(PROG_CFLAGS)

#timer
TIMER_PATHS := src deps
TIMER_OBJ := $(patsubst %.c,obj/%.o,$(shell find $(TIMER_PATHS) -type f -name *.c))
$(TIMER_OBJ): PROG_CFLAGS := -Isrc/timer
chronos: $(TIMER_OBJ) livesplit-core/liblivesplit_core.a
	@echo "LINK "$@
	$Q$(CC) -o $@ $(TIMER_OBJ) $(LDFLAGS)

# dependencies
DEPS = $(patsubst %.c,obj/%.d,$(shell find src deps -type f -name *.c))

$(patsubst %.c,obj/%.o,$(shell find deps -type f -name *.c)): \
	PROG_CFLAGS := -DINI_ALLOW_NO_VALUE=1

$(DEPS): ;
include $(DEPS)


livesplit-core/liblivesplit_core.a:
	@cd livesplit-core/livesplit-core; \
	git reset --hard; \
	git apply --3way ../livesplit_core.patch; \
	cargo build --release -p staticlib; \
	cp target/release/liblivesplit_core.a ../liblivesplit_core.a; \

livesplit-core/livesplit_core.h:
	@cd livesplit-core/livesplit-core/capi/bind_gen; \
	cargo run; \
	cp ../bindings/livesplit_core.h ../../../livesplit_core.h; \
