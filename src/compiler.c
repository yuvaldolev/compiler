#include <stdio.h>

#include "compiler.h"
#include "string.c"

#include "os_mac.c"

enum {
	PROG_ARGS_PROGRAM_NAME = 0,
	PROG_ARGS_FILE_PATH,

	PROG_ARGS_COUNT,
};

function void
load_file(const char *file_path) {
	string_u8_t source = os_read_entire_file(file_path);
	if (is_null_string_u8(source)) {
		fprintf(stderr, "ERROR: No Such File '%s'.\n", file_path);
		return;
	}

	lexer_t lexer = lexer_make(file_path, contents);
	lexer_tokenize_file(lexer);
}

int main(int argc, const char * const argv[]) {
	// Validate the command line arguments.
	if (PROG_ARGS_COUNT != argc) {
		fprintf(stderr, "USAGE: %s <file_path>\n",
				argv[PROG_ARGS_PROGRAM_NAME]);
		return -1;
	}

	// Lex & parse the file to compile.
	load_file(argv[PROG_ARGS_FILE_PATH]);
}

