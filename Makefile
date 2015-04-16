RC = /usr/local/bin/rustc
OUT_DIR = ./build
LIB_DIR = ./lib

LIB_OPT = --extern rand=lib/librand-0.3.7.rlib
DEBUG_OPT = -g --cfg debug
RELEASE_OPT = -O --cfg release
TEST_OPT = --test

CrateName = RedStart
CrateType = bin

all: debug

# Build a release candidate
release:
	mkdir -p ${OUT_DIR}/release/
	${RC} -L ${LIB_DIR} ${LIB_OPT} --crate-name ${CrateName} --crate-type ${CrateType} ${RELEASE_OPT} --out-dir ${OUT_DIR}/release/ src/main.rs

# Build a debug candidate
debug:
	mkdir -p ${OUT_DIR}/debug/
	${RC} -L ${LIB_DIR} ${LIB_OPT} --crate-name ${CrateName} --crate-type ${CrateType} ${DEBUG_OPT} --out-dir ${OUT_DIR}/debug/ src/main.rs

test:
	mkdir -p ${OUT_DIR}/test/
	${RC} -L ${LIB_DIR} ${LIB_OPT} --crate-name ${CrateName} --crate-type ${CrateType} ${TEST_OPT} --out-dir ${OUT_DIR}/test/ src/main.rs
	${OUT_DIR}/test/RedStart

clean:
	rm -rf ${OUT_DIR}
