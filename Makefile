RC = rustc
OUT_DIR = ./build
LIB_DIR = ./lib

DEBUG_OPT = -g --cfg debug
RELEASE_OPT = -O --cfg release
TEST_OPT = --test

CrateName = RedStart
CrateType = bin

all: debug

# Build a release candidate
release:
	${MAKE} -C ${LIB_DIR} release
	mkdir -p ${OUT_DIR}/release/
	${RC} --crate-name ${CrateName} --crate-type ${CrateType} ${RELEASE_OPT} --out-dir ${OUT_DIR}/release/ -L ${LIB_DIR} src/main.rs

# Build a debug candidate
debug:
	${MAKE} -C ${LIB_DIR} debug
	mkdir -p ${OUT_DIR}/debug/
	${RC} --crate-name ${CrateName} --crate-type ${CrateType} ${DEBUG_OPT} --out-dir ${OUT_DIR}/debug/ -L ${LIB_DIR} src/main.rs

test:
	mkdir -p ${OUT_DIR}/test/
	${RC} --crate-name ${CrateName} --crate-type ${CrateType} ${TEST_OPT} --out-dir ${OUT_DIR}/test/ -L ${LIB_DIR} src/main.rs
	${OUT_DIR}/test/RedStart

clean:
	${MAKE} -C ${LIB_DIR} clean
	rm -rf ${OUT_DIR}
