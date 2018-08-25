/**
 * \enum rs_result
 * \brief Return codes from nonblocking rsync operations.
 * \see rs_strerror()
 * \see api_callbacks
 */
typedef enum rs_result {
    RS_DONE =        0,    /**< Completed successfully. */
    RS_BLOCKED =    1,     /**< Blocked waiting for more data. */

    /** The job is still running, and not yet finished or blocked.
     * (This value should never be seen by the application.) */
    RS_RUNNING  =       2,

    RS_TEST_SKIPPED =   77,     /**< Test neither passed or failed. */

    RS_IO_ERROR =    100,    /**< Error in file or network IO. */
    RS_SYNTAX_ERROR =   101,    /**< Command line syntax error. */
    RS_MEM_ERROR =    102,    /**< Out of memory. */
    /** Unexpected end of input file, perhaps due to a truncated file
     * or dropped network connection. */
    RS_INPUT_ENDED =    103,
    RS_BAD_MAGIC =      104,    /**< Bad magic number at start of
                                   stream.  Probably not a librsync
                                   file, or possibly the wrong kind of
                                   file or from an incompatible
                                   library version. */
    RS_UNIMPLEMENTED =  105,    /**< Author is lazy. */
    RS_CORRUPT =        106,    /**< Unbelievable value in stream. */
    RS_INTERNAL_ERROR = 107,    /**< Probably a library bug. */
    RS_PARAM_ERROR =    108     /**< Bad value passed in to library,
                                 * probably an application bug. */
} rs_result;

/**
 * Return an English description of a ::rs_result value.
 */
char const *rs_strerror(rs_result r);

/** Return a pointer to the library version, eg "3.0.0" */
extern const char* rs_version();
