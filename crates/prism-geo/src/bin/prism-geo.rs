// © 2026 aiaiaiai · aiaiaiai.org
// SPDX-License-Identifier: Apache-2.0

//! Strict, stateless H3 cell validation for clients outside Rust.
//!
//! The cell travels on stdin (not in command-line arguments or logs). Only a
//! success marker or a stable error code is printed; neither echoes the cell.

use std::io::Read;
use std::process::ExitCode;

use prism_geo::Grid;

fn invalid(code: &'static str) -> ExitCode {
    eprintln!("{code}");
    ExitCode::from(2)
}

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    let (Some("validate-cell"), Some(resolution), None) =
        (args.next().as_deref(), args.next(), args.next())
    else {
        return invalid("invalid_request");
    };

    let Ok(number) = resolution.parse::<u8>() else {
        return invalid("invalid_request");
    };
    let Ok(grid) = Grid::new(number) else {
        return invalid("invalid_request");
    };

    // A canonical H3 cell has 15 ASCII bytes. Read at most 128 bytes even for
    // hostile stdin; oversized, multiline and non-UTF-8 input fail closed.
    let mut bytes = Vec::new();
    if std::io::stdin().take(128).read_to_end(&mut bytes).is_err() {
        eprintln!("io_error");
        return ExitCode::FAILURE;
    }
    let Ok(input) = std::str::from_utf8(&bytes) else {
        return invalid("invalid_cell");
    };
    let cell = input.strip_suffix('\n').unwrap_or(input);
    if cell.len() != 15
        || !cell
            .bytes()
            .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
    {
        return invalid("invalid_cell");
    }

    match grid.parse(cell) {
        Ok(parsed) if parsed.to_string() == cell => {
            println!("valid");
            ExitCode::SUCCESS
        }
        _ => invalid("invalid_cell"),
    }
}
