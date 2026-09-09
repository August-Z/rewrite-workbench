use std::{
    env,
    ffi::OsStr,
    fs,
    io::{self, Write},
    path::Path,
    process::ExitCode,
};

use rewrite_core::{Status, inspect_tsx};

const USAGE: &str = "Usage: rewrite-workbench inspect <file.tsx>\n\nRead-only TSX parsing and binding spike. Prints JSON; does not generate edits.\nExit codes: 0 = inspection completed, 1 = invalid TSX, 2 = usage or I/O error.";

fn run() -> Result<ExitCode, String> {
    let args: Vec<_> = env::args_os().skip(1).collect();
    if args.len() == 1 && (args[0] == "--help" || args[0] == "-h") {
        println!("{USAGE}");
        return Ok(ExitCode::SUCCESS);
    }
    if args.len() != 2 || args[0] != "inspect" {
        return Err(USAGE.to_owned());
    }
    let path = Path::new(&args[1]);
    if path.extension() != Some(OsStr::new("tsx")) {
        return Err(
            "inspect requires a .tsx file; other syntax modes are not supported".to_owned(),
        );
    }
    let source = fs::read_to_string(path)
        .map_err(|error| format!("Cannot read UTF-8 input {}: {error}", path.display()))?;
    let report = inspect_tsx(&source);
    let mut stdout = io::stdout().lock();
    serde_json::to_writer_pretty(&mut stdout, &report)
        .map_err(|error| format!("Cannot write report: {error}"))?;
    writeln!(stdout).map_err(|error| format!("Cannot write report: {error}"))?;
    Ok(if report.status == Status::Invalid {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    })
}

fn main() -> ExitCode {
    match run() {
        Ok(code) => code,
        Err(message) => {
            eprintln!("{message}");
            ExitCode::from(2)
        }
    }
}
