LIB_DIRS = libldap

# Build a release candidate
release:
	set -e; for dir in ${LIB_DIRS} ; do ${MAKE} -C $$dir release; done

# Build a debug candidate
debug:
	set -e; for dir in ${LIB_DIRS} ; do ${MAKE} -C $$dir debug; done

# Build a test harnish
test:
	set -e; for dir in ${LIB_DIRS} ; do ${MAKE} -C $$dir test; done

# Clean build directories
clean:
	set -e; for dir in ${LIB_DIRS} ; do ${MAKE} -C $$dir clean; done
