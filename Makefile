RC = rustc
OUT_DIR = ./build
LIB_DIR = ./lib

DEBUG_OPT = -g
RELEASE_OPT = -O

CrateName = RedStart
CrateType = bin

all: debug

prelude:
	mkdir -p ${OUT_DIR}

# Build a release candidate
release:
	mkdir -p ${OUT_DIR}/release/
	${RC} --crate-name ${CrateName} --crate-type ${CrateType} ${RELEASE_OPT} --out-dir ${OUT_DIR}/release/ -L ${LIB_DIR} src/main.rs

# Build a debug candidate
debug:
	mkdir -p ${OUT_DIR}/debug/
	${RC} --crate-name ${CrateName} --crate-type ${CrateType} ${DEBUG_OPT} --out-dir ${OUT_DIR}/debug/ -L ${LIB_DIR} src/main.rs

clean:
	${MAKE} -C ${LIB_DIR} clean
	rm -rf ${OUT_DIR}/*
