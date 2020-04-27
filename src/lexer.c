funciton void
lexer_tokenize(lexer_t *lexer) {
	token_t token;
	do {
		token = lex_token();

		// Ignore comments.
		if (TOKEN_KIND_COMMENT == token.kind) {
			continue;
		}

		ARRAY_ADD(lexer->tokens, token);
	} while (TOKEN_KIND_END != token.kind);
}

function lexer_t
lexer_make(const char *file_path, string_u8_t source) {
	lexer_t result = {0};
	result.file_path = file_path;
	result.source = source;

	return result;
}

