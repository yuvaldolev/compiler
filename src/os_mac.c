#include <fcntl.h>
#include <stdlib.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <unistd.h>

function string_u8_t
os_read_entire_file(const char * file_path)
{
	string_u8_t contents = {0};

	// Open the file for reading.
	int fd = open(file_path, O_RDONLY);
	if (-1 == fd) {
		goto lbl_cleanup;
	}

	// Get the size of the file to read.
	struct stat file_info;
	if (0 != fstat(fd, &file_info)) {
		goto lbl_cleanup;
	}

	// Allocate the contents string.
	contents.str = malloc(file_info.st_size);
	if (!contents.str) {
		goto lbl_cleanup;
	}

	// Read the file.
	u8 *cursor = contents.str;
	umm bytes_remaining = file_info.st_size;
	while (0 < bytes_remaining) {
		ssize_t bytes_read = read(fd, cursor, bytes_remaining);
		if (-1 == bytes_read) {
			// Read failed... Free contents and go to cleanup.
			free(contents.str);
			contents.str = 0;
			goto lbl_cleanup;
		}

		cursor += bytes_read;
		bytes_remaining -= bytes_read;
	}

	contents.count = file_info.st_size;
	contents.capacity = file_info.st_size;

lbl_cleanup:
	if (-1 != fd) {
		close(fd);
	}

	return contents;
}

