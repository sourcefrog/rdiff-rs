// librdiff(rust) -- library for network deltas
// Copyright 2015, 2016 Martin Pool.

// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public License
// as published by the Free Software Foundation; either version 2.1 of
// the License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this program; if not, write to the Free Software
// Foundation, Inc., 675 Mass Ave, Cambridge, MA 02139, USA.


// TODO: Expose version string and number.


#[repr(C)]
pub enum RsResult {
    RS_DONE = 0,

    /// Blocked waiting for more data.
    RS_BLOCKED =    1,

    /// The job is still running, and not yet finished or blocked.
    RS_RUNNING  =       2,

    RS_TEST_SKIPPED =   77,     //< Test neither passed or failed.

    RS_IO_ERROR =    100,    //< Error in file or network IO. */
    RS_SYNTAX_ERROR =   101,    //< Command line syntax error. */
    RS_MEM_ERROR =    102,    //< Out of memory. */
    /// Unexpected end of input file, perhaps due to a truncated file
    /// or dropped network connection.
    RS_INPUT_ENDED =    103,
    /// Bad magic number at start of stream.  Probably not a librsync file,
    /// or possibly the wrong kind of file or from an incompatible
    /// library version.
    RS_BAD_MAGIC =      104,
    RS_UNIMPLEMENTED =  105,    //< Author is lazy. */
    RS_CORRUPT =        106,    //< Unbelievable value in stream. */
    RS_INTERNAL_ERROR = 107,    //< Probably a library bug. */
    /// Bad value passed in to library,     probably an application bug.
    RS_PARAM_ERROR =    108,
}
