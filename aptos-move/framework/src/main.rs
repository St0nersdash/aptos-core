// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

#![forbid(unsafe_code)]

use framework::release::ReleaseOptions;
use structopt::StructOpt;

// Generates the compiled stdlib and transaction scripts. Until this is run changes to the source
// modules/scripts, and changes in the Move compiler will not be reflected in the stdlib used for
// genesis, and everywhere else across the code-base unless otherwise specified.
fn main() {
    let mut release_options = ReleaseOptions::from_args();
    // TODO: reactivate this once builder is fixed.
    release_options.no_script_builder = true;
    release_options.create_release();
}