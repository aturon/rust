// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![crate_id="static_methods_crate#0.1"]
#![crate_type = "lib"]

use std::int;

pub trait read {
    fn readMaybe(s: StrBuf) -> Option<Self>;
}

impl read for int {
    fn readMaybe(s: StrBuf) -> Option<int> {
        from_str::<int>(s.as_slice())
    }
}

impl read for bool {
    fn readMaybe(s: StrBuf) -> Option<bool> {
        match s.as_slice() {
          "true" => Some(true),
          "false" => Some(false),
          _ => None
        }
    }
}

pub fn read<T:read>(s: StrBuf) -> T {
    match read::readMaybe(s) {
      Some(x) => x,
      _ => fail!("read failed!")
    }
}
