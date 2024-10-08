//
// Copyright (c) 2024 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use super::*;

#[test]
fn test_do_frontend_phases_does_frontend_phases_with_standard_library()
{
    let s = "
kernel mykernel() -> () = ();
";
    let s2 = &s[1..];
    match do_frontend_phases(s2) {
        Ok(_) => assert!(true),
        Err(errs) => {
            println!("{}", errs);
            assert!(false);
        },
    }
}
