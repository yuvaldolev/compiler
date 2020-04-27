#pragma once

typedef struct {
	const char *str;
	umm count;
} string_const_char_t;

typedef struct {
	union {
		const void *data;
		const u8 *str;
	};
	umm count;
} string_const_u8_t;

typedef struct {
	const u16 *str;
	umm count;
} string_const_u16_t;

typedef struct {
	const u32 *str;
	umm count;
} string_const_u32_t;

typedef struct {
	union {
		string_const_char_t string;
		struct {
			char *str;
			umm count;
		};
	};
	umm capacity;
} string_char_t;

typedef struct {
	union {
		string_const_u8_t string;
		struct {
			u8 *str;
			umm count;
		};
	};
	umm capacity;
} string_u8_t;

typedef struct {
	union {
		string_const_u16_t string;
		struct {
			u16 *str;
			umm count;
		};
	};
	umm capacity;
} string_u16_t;

typedef struct {
	union {
		string_const_u32_t string;
		struct {
			u32 *str;
			umm count;
		};
	};
	umm capacity;
} string_u32_t;

