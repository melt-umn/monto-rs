# The type of build that the FFI bindings were built with.
# Should only ever be `debug' or `release'.
BUILD_TYPE=release

# The directory where the .so lives.
TARGET_DIR=../../../target/$(BUILD_TYPE)

CFLAGS+=-I../../include
LDFLAGS+=-L$(TARGET_DIR)
LDLIBS+=-lmonto_rs
SOURCES=$(wildcard *.c)
OBJECTS=$(patsubst %.c,%.o,$(SOURCES))
EXECNAME=example-service

.PHONY: all clean run
all: $(EXECNAME)
clean:
	@rm -f $(EXECNAME) $(OBJECTS)
run: $(EXECNAME)
	@LD_LIBRARY_PATH=$(TARGET_DIR) ./$(EXECNAME)

$(EXECNAME): $(OBJECTS)
	$(CC) $(LDFLAGS) -o $(@) $(^) $(LDLIBS)

%.o: %.c
	$(CC) -c $(CFLAGS) -o $(@) $(<)
