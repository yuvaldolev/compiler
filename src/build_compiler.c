#include "basic.h"

#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>

#define DIR_SRC "src"
#define DIR_RUN_TREE "run_tree"

#define COMPILER_FLAGS "-Wall -Werror -Wno-missing-braces "

function int
systemf(const char * fmt, ...) {
	va_list arg_list;

	// Format the command.
	char buffer[4096];
	va_start(arg_list, fmt);
	vsnprintf(buffer, sizeof(buffer), fmt, arg_list);
	va_end(arg_list);

	// Execute the command.
	int result = system(buffer);
	return result;
}

int main(void) {
	systemf("mkdir -p %s", DIR_RUN_TREE);
	systemf("clang %s %s/compiler.c -o %s/compiler",
		   COMPILER_FLAGS, DIR_SRC, DIR_RUN_TREE);
}

