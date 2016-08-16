#ifndef DOTBC_H_INCLUDED
#define	DOTBC_H_INCLUDED

/*
 * Opaque type referencing an open DotBC handle.
 */
typedef void dotbc_archive_t;

/*
 * Open a DotBC file at the given path.
 */
int dotbc_open(const char* path, dotbc_archive_t** handle);

/*
 * Closes the given DotBC handle and frees all associated resources. The handle
 * cannot be used after this.
 */
void dotbc_handle_free(dotbc_archive_t* handle);

/*
 * Get the byte size of a file at the given path
 */
int dotbc_get_file_len_at(
        dotbc_archive_t* handle,
        const char* path,
        size_t* ret);

/*
 * Read the entire file at the given path into the given buffer.
 */
int dotbc_read_file_at(
        dotbc_archive_t* handle,
        const char* path,
        void* buf,
        size_t buf_len,
        size_t* num_read);

/*
 * Write the given file
 */
int dotbc_write_file_at(
        dotbc_archive_t* handle,
        const char* path,
        void* buf,
        size_t buf_len);

#endif
