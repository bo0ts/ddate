CFLAGS += \
	-Wall -Wextra -g -ggdb  \
	-std=c11 -O2 -fPIE -march=native -D_FORTIFY_SOURCE=2 \
	-fstack-protector-strong --param=ssp-buffer-size=1 -flto \
	-fsanitize=undefined

ifeq "$(CC)" "clang"
CFLAGS += -Weverything -fno-diagnostics-color -fsanitize-trap=undefined
endif

LDFLAGS += -Wl,-z,relro,-z,now,--build-id -pie

all: ddate

clean:
	rm -f ddate

.PHONY: all clean
