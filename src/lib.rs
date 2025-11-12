/*

Copyright (c) nexB Inc. and others. All rights reserved.
ScanCode is a trademark of nexB Inc.
SPDX-License-Identifier: Apache-2.0
See http://www.apache.org/licenses/LICENSE-2.0 for the license text.
See https://github.com/aboutcode-org/purl-validator-rust for support or download.
See https://aboutcode.org for more information about nexB OSS projects.

*/

//! A library to validate whether a PURL actually exists.
//!
//! **purl-validator** is a Rust library for validating
//! [`Package URLs` (PURLs)](https://github.com/package-url/purl-spec).
//! It works fully offline, including in **air-gapped** or **restricted environments**,
//! and answers one key question: **Does the package this PURL represents actually exist?**
//!
//!
//! # Examples
//!
//! Simplest way to use `validate` is as follows:
//!
//! ```
//! use purl_validator::validate;
//!
//! let result: bool = validate("pkg:nuget/FluentValidation");
//! ```
//!

use fst::Set;
use memmap2::Mmap;
use once_cell::sync::Lazy;
use std::env;
use std::fs::File;
use std::path::Path;

static VALIDATOR: Lazy<Set<Mmap>> = Lazy::new(|| {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("purls.fst");
    load_fst(&path)
});

fn load_fst(path: &Path) -> Set<Mmap> {
    let file = File::open(path).expect("Failed to open FST file");
    let mmap = unsafe { Mmap::map(&file).expect("Failed to mmap FST file") };
    Set::new(mmap).expect("Failed to load FST from mmap")
}

fn strip_and_check_purl(packageurl: &str, fst_map: &Set<Mmap>) -> bool {
    let trimmed_packageurl = packageurl.trim_end_matches("/");
    fst_map.contains(trimmed_packageurl)
}

/// Validate a Package URL (PURL)
///
/// Returns `true` if the given base PURL represents an existing package,
/// otherwise returns `false`.
///
/// Use pre-built FST (Finite State Transducer) to perform lookups and confirm whether
/// the **base PURL** exists.
pub fn validate(packageurl: &str) -> bool {
    strip_and_check_purl(packageurl, &VALIDATOR)
}

#[cfg(test)]
mod validate_tests;
