// © 2026 aiaiaiai · aiaiaiai.org
// SPDX-License-Identifier: Apache-2.0

use std::io::Write;
use std::process::{Command, Output, Stdio};

const CELL: &str = "860335a97ffffff";

fn run(args: &[&str], input: &[u8]) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_prism-geo"))
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(input).unwrap();
    child.wait_with_output().unwrap()
}

#[test]
fn valid_canonical_cell_is_accepted_without_echoing_it() {
    let result = run(&["validate-cell", "6"], format!("{CELL}\n").as_bytes());
    assert!(result.status.success());
    assert_eq!(result.stdout, b"valid\n");
    assert!(result.stderr.is_empty());
    assert!(
        !result
            .stdout
            .windows(CELL.len())
            .any(|w| w == CELL.as_bytes())
    );
}

#[test]
fn only_a_real_cell_of_the_exact_resolution_is_accepted() {
    for input in [
        CELL.to_uppercase(),
        "850335a97ffffff".into(),
        "8a0335a97ffffff".into(),
        "860335a97fffff".into(),
        "860335a97ffffffa".into(),
        "860335a97ffffff\nextra".into(),
        "50.4501,30.5234".into(),
        "860000000000000".into(),
        " ".into(),
    ] {
        let result = run(&["validate-cell", "6"], input.as_bytes());
        assert_eq!(result.status.code(), Some(2), "{input}");
        assert!(result.stdout.is_empty());
        assert_eq!(result.stderr, b"invalid_cell\n");
    }
}

#[test]
fn missing_or_invalid_resolution_and_arguments_are_rejected() {
    for args in [
        vec![],
        vec!["validate-cell"],
        vec!["validate-cell", "16"],
        vec!["validate-cell", "-1"],
        vec!["validate-cell", "6", "860335a97ffffff"],
        vec!["unknown", "6"],
    ] {
        let result = run(&args, CELL.as_bytes());
        assert_eq!(result.status.code(), Some(2), "{args:?}");
        assert!(result.stdout.is_empty());
        assert_eq!(result.stderr, b"invalid_request\n");
    }
}

#[test]
fn oversized_and_non_utf8_inputs_fail_closed() {
    for input in [vec![b'x'; 4096], vec![0xff, 0xfe]] {
        let result = run(&["validate-cell", "6"], &input);
        assert_eq!(result.status.code(), Some(2));
        assert_eq!(result.stderr, b"invalid_cell\n");
    }
}
