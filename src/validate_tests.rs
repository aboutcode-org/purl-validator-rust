/*

Copyright (c) nexB Inc. and others. All rights reserved.
ScanCode is a trademark of nexB Inc.
SPDX-License-Identifier: Apache-2.0
See http://www.apache.org/licenses/LICENSE-2.0 for the license text.
See https://github.com/aboutcode-org/purl-validator-rust for support or download.
See https://aboutcode.org for more information about nexB OSS projects.

*/

use super::*;
use std::path::Path;

#[test]
fn test_validate_with_custom_file() {
    let test_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data/test_purls.fst");
    let validator = load_fst(&test_path);

    assert!(strip_and_check_purl(
        "pkg:nuget/FluentUtils.EnumExtensions",
        &validator
    ));
    assert!(!strip_and_check_purl("pkg:example/nonexistent", &validator));
}

#[test]
fn test_validate_with_packageurl_trailing_slash() {
    let test_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data/test_purls.fst");
    let validator = load_fst(&test_path);

    assert!(validator.contains("pkg:nuget/FluentUtils.EnumExtensions"));
    assert!(strip_and_check_purl(
        "pkg:nuget/FluentUtils.EnumExtensions/",
        &validator
    ));
}
