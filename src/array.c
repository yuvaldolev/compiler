function void*
_array_grow(void *array, umm element_size, umm grow_by, umm min_capacity) {
	umm min_count = ARRAY_GET_COUNT(array) + grow_by;

	// Compute the minimum capacity needed.
	umm new_capacity = min_capacity;
	if (min_count > new_capacity) {
		new_capacity = min_count;
	}

	// Check if we actually need to grow the array.
	if (new_capacity <= ARRAY_GET_CAPACITY(array)) {
		return array;
	}

	// Increase needed capacity to guarantee O(1) amortized.
	if (new_capacity < (2 * ARRAY_GET_CAPACITY(array))) {
		new_capacity = 2 * ARRAY_GET_CAPACITY(array);
	}

	// Grow the array.
	void *new_array = malloc(sizeof(array_header_t) +
			                 (element_size * new_capacity));
	if (!new_array) {
		return 0;
	}

	// Copy & free the previous array's memory.
	if (array) {
		memcpy(new_array, _ARRAY_HEADER(array), sizeof(array_header_t) +
			   (ARRAY_GET_COUNT(array) * element_size));
		free(_ARRAY_HEADER(array));
	}

	new_array = ((u8*)new_array) + sizeof(array_header_t);
	if (!array) {
		_ARRAY_HEADER(array)->count = 0;
	}
	_ARRAY_HEADER(array)->capacity = new_capacity;

	return new_array;
}

