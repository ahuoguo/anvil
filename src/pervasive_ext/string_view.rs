// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: MIT

use vstd::prelude::*;
use vstd::string::*;

verus! {

pub type StringView = Seq<char>;

#[verifier(external_body)]
pub fn i32_to_string(i: i32) -> (s: String)
    ensures
        s@ == int_to_string_view(i as int),
{
    String::from_rust_string(i.to_string())
}

pub closed spec fn int_to_string_view(i: int) -> StringView;

#[verifier(external_body)]
pub proof fn int_to_string_view_injectivity()
    ensures
        forall |i: int, j: int| int_to_string_view(i) == int_to_string_view(j) ==> i == j,
{}

}
