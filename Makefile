CC=rustc
OUT_DIR=./build

CrateName=RedStart


all: build

prelude:
	mkdir -p build

build: prelude
	${CC} --crate-name ${CrateName} --crate-type bin -g --out-dir ${OUT_DIR} -L ./lib src/main.rs

clean:
	rm -rf build/*
