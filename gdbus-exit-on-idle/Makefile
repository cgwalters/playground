all: gdbus-exit-on-idle-daemon

CFLAGS ?= -ggdb -O0
CC ?= gcc

gdbus-exit-on-idle-daemon: gdbus-exit-on-idle-daemon.c Makefile
	$(CC) $(CFLAGS) $$(pkg-config --cflags --libs gio-2.0 libsystemd-daemon) -o $@ $<
