// librsync(rust) -- library for network deltas
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

extern crate libc;
extern crate librdiff;

/// Nul-terminated version number for ease of C binding.
pub static VERSION: &'static str = "3.0.0\0";

#[no_mangle]
pub extern fn rs_version() -> *const libc::c_char {
    // Version from environment has nul termination (I think we can count on this?)
    return VERSION.as_ptr() as *const libc::c_char;
}


#[cfg(test)]
#[test]
pub fn test_versions_consistent() {
    // I can't work out how to automatically store a static CString, but
    // let's at least check they're in sync, and that ours has a nul.
    assert_eq!(VERSION.as_bytes()[VERSION.len()-1], 0);
    let their_v = librdiff::VERSION;
    let l = their_v.len();
    assert_eq!(VERSION.len(), l + 1);
    assert_eq!(VERSION[0..l], their_v.to_string());
}
