#define ARRAY_ADD(array, value) \
	do { \
		_ARRAY_MAY_GROW(array, 1); \
		(array)[_ARRAY_HEADER(array)->count++] = (value); \
	} while (0)

#define ARRAY_GET_COUNT(array) \
	((array) ? _ARRAY_HEADER(array)->count : 0)

#define _ARRAY_MAY_GROW(array, grow_by) \
	do { \
		if (!(array) || _ARRAY_HEADER(array)->count + (grow_by) > \
			_ARRAY_HEADER(array)->capacity) { \
			_ARRAY_GROW(array, grow_by, 0); \
		} \
	} while (0)

#define _ARROW_GROW(array, grow_by, min_capacity) \
	do { \
		(array) = _array_grow(array, sizeof(*(array)), grow_by, min_capacity);
	} while (0)
