/*

Copyright (c) nexB Inc. and others. All rights reserved.
ScanCode is a trademark of nexB Inc.
SPDX-License-Identifier: Apache-2.0
See http://www.apache.org/licenses/LICENSE-2.0 for the license text.
See https://github.com/aboutcode-org/purl-validator-rust for support or download.
See https://aboutcode.org for more information about nexB OSS projects.

*/

use fst::SetBuilder;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use std::io::Write;

fn main() {
    let input_file = Path::new("fst_builder/data/purls.txt");
    let output_file = Path::new("purls.fst");

    let file = File::create(output_file).expect("Cannot create FST file");
    let mut builder = SetBuilder::new(file).expect("Failed to create FST builder");

    let f = File::open(input_file).expect("Cannot open input file");
    let reader = BufReader::new(f);

    let mut lines: Vec<String> = reader
        .lines()
        .map(|l| l.expect("Failed to read line"))
        .filter(|s| !s.is_empty())
        .collect();

    lines.sort();

    let sorted_file = Path::new("fst_builder/data/purls.txt");
    let mut sorted_f = File::create(sorted_file).expect("Cannot create sorted file");
    for line in &lines {
        writeln!(sorted_f, "{}", line).expect("Failed to write line");
    }

    for line in lines {
        builder.insert(&line).unwrap();
    }

    builder.finish().expect("Failed to finish FST");
    println!("FST generated at {:?}", output_file);
}
