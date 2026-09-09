use std::{
    env,
    ffi::OsStr,
    fs,
    io::{self, Write},
    path::Path,
    process::ExitCode,
};

use rewrite_core::{ImportSelector, Status, classify_tsx, inspect_tsx};

const USAGE: &str = "Usage: rewrite-workbench inspect <file.tsx>\n       rewrite-workbench bindings <file.tsx> <moduleSpecifier> <importedName>\n\nRead-only TSX inspection or direct named import classification. Prints JSON; does not generate edits.\nmoduleSpecifier matches the parsed import literal, not a resolved package or file.\nExit codes: 0 = analysis completed, 1 = invalid TSX, 2 = usage or I/O error.";

fn run() -> Result<ExitCode, String> {
    let args: Vec<_> = env::args_os().skip(1).collect();
    if args.len() == 1 && (args[0] == "--help" || args[0] == "-h") {
        println!("{USAGE}");
        return Ok(ExitCode::SUCCESS);
    }
    let classify = args.len() == 4 && args[0] == "bindings";
    if !classify && (args.len() != 2 || args[0] != "inspect") {
        return Err(USAGE.to_owned());
    }
    let selector = if classify {
        Some(ImportSelector {
            module_specifier: args[2]
                .to_str()
                .ok_or("moduleSpecifier must be UTF-8")?
                .to_owned(),
            imported_name: args[3]
                .to_str()
                .ok_or("importedName must be UTF-8")?
                .to_owned(),
        })
    } else {
        None
    };
    let path = Path::new(&args[1]);
    if path.extension() != Some(OsStr::new("tsx")) {
        return Err(
            "analysis requires a .tsx file; other syntax modes are not supported".to_owned(),
        );
    }
    let source = fs::read_to_string(path)
        .map_err(|error| format!("Cannot read UTF-8 input {}: {error}", path.display()))?;
    let mut stdout = io::stdout().lock();
    let (status, output) = if let Some(selector) = selector {
        let report = classify_tsx(&source, &selector);
        (
            report.status,
            serde_json::to_writer_pretty(&mut stdout, &report),
        )
    } else {
        let report = inspect_tsx(&source);
        (
            report.status,
            serde_json::to_writer_pretty(&mut stdout, &report),
        )
    };
    output.map_err(|error| format!("Cannot write report: {error}"))?;
    writeln!(stdout).map_err(|error| format!("Cannot write report: {error}"))?;
    Ok(if status == Status::Invalid {
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
