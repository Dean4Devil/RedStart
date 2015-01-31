CC=gcc
CFLAGS=-Wall
LIBS=-lldap -lsasl2

all: clean build test

prelude:
	mkdir target

build: prelude
	$(CC) -c $(CFLAGS) -ggdb -fPIC src/ggnet.c -o target/ggnet.o $(LIBS)
	$(CC) -shared -ggdb -o target/libggnet.so target/ggnet.o $(LIBS)

test: 
	$(CC) -L./target $(CFLAGS) -ggdb -o target/test src/main.c -lggnet
	LD_LIBRARY_PATH=./target target/test


clean:
	rm -rf target/
