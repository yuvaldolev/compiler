global string_u8_t g_null_string_u8 = {0};

////////////////////////////////

function umm
cstring_count_char(const char *str) {
	umm count = 0;
	for (; str[count]; ++count);
	return count;
}

function umm
cstring_count_u8(const u8 *str) {
	umm count = 0;
	for (; str[count]; ++count);
	return count;
}

function umm
cstring_count_u16(const u16 *str) {
	umm count = 0;
	for (; str[count]; ++count);
	return count;
}

function umm
cstring_count_u32(const u32 *str) {
	umm count = 0;
	for (; str[count]; ++count);
	return count;
}

////////////////////////////////

function inline string_const_char_t
string_const_char_make(const char *str, umm count) {
	string_const_char_t string = {str, count};
	return string;
}

function inline string_const_u8_t
string_const_u8_make(const u8 *str, umm count) {
	string_const_u8_t string = {str, count};
	return string;
}

function inline string_const_u16_t
string_const_u16_make(const u16 *str, umm count) {
	string_const_u16_t string = {str, count};
	return string;
}

function inline string_const_u32_t
string_const_u32_make(const u32 *str, umm count) {
	string_const_u32_t string = {str, count};
	return string;
}

////////////////////////////////

function inline string_const_char_t
string_const_char_make_slow(const char *str) {
	umm count = cstring_count_char(str);
	string_const_char_t string = string_const_char_make(str, count);
	return string;
}

function inline string_const_u8_t
string_const_u8_make_slow(const u8 *str) {
	umm count = cstring_count_u8(str);
	string_const_u8_t string = string_const_u8_make(str, count);
	return string;
}

function inline string_const_u16_t
string_const_u16_make_slow(const u16 *str) {
	umm count = cstring_count_u16(str);
	string_const_u16_t string = string_const_u16_make(str, count);
	return string;
}

function inline string_const_u32_t
string_const_u32_make_slow(const u32 *str) {
	umm count = cstring_count_u32(str);
	string_const_u32_t string = string_const_u32_make(str, count);
	return string;
}

////////////////////////////////

function string_char_t
string_char_make_cap(char *str, umm count, umm capacity) {
	string_char_t string = {str, count, capacity};
	return string;
}

function string_u8_t
string_u8_make_cap(u8 *str, umm count, umm capacity) {
	string_u8_t string = {str, count, capacity};
	return string;
}

function string_u16_t
string_u16_make_cap(u16 *str, umm count, umm capacity) {
	string_u16_t string = {str, count, capacity};
	return string;
}

function string_u32_t
string_u32_make_cap(u32 *str, umm count, umm capacity) {
	string_u32_t string = {str, count, capacity};
	return string;
}

////////////////////////////////

function inline string_char_t
string_char_make(char *str, umm count) {
	string_char_t string = string_char_make_cap(str, count, count + 1);
	return string;
}

function inline string_u8_t
string_u8_make(u8 *str, umm count) {
	string_u8_t string = string_u8_make_cap(str, count, count + 1);
	return string;
}

function inline string_u16_t
string_u16_make(u16 *str, umm count) {
	string_u16_t string = string_u16_make_cap(str, count, count + 1);
	return string;
}

function inline string_u32_t
string_u32_make(u32 *str, umm count) {
	string_u32_t string = string_u32_make_cap(str, count, count + 1);
	return string;
}

////////////////////////////////

function inline string_char_t
string_char_make_slow(char *str) {
	umm count = cstring_count_char(str);
	string_char_t string = string_char_make(str, count);
	return string;
}

function inline string_u8_t
string_u8_make_slow(u8 *str) {
	umm count = cstring_count_u8(str);
	string_u8_t string = string_u8_make(str, count);
	return string;
}

function inline string_u16_t
string_u16_make_slow(u16 *str) {
	umm count = cstring_count_u16(str);
	string_u16_t string = string_u16_make(str, count);
	return string;
}

function inline string_u32_t
string_u32_make_slow(u32 *str) {
	umm count = cstring_count_u32(str);
	string_u32_t string = string_u32_make(str, count);
	return string;
}

////////////////////////////////

function inline b8
is_null_string_u8(string_u8_t str) {
	return ((str.str == g_null_string_u8.str) &&
			(str.count == g_null_string_u8.count) &&
		    (str.capacity == g_null_string_u8.capacity));
}

