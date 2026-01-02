use glob::glob;
use std::{env, fs, path::PathBuf};

fn main() {
    let mut args: Vec<String> = env::args().collect();

    if args.iter().any(|a| a == "--help" || a == "-h") {
        println!(
            "Usage: readall [GLOB_PATTERNS...]

Read all the files for copy-pasting.

Examples:
  readall \"src/**/*.rs\" \"README.md\"
"
        );
        return;
    }

    // Drop program name only
    args.remove(0);

    if args.is_empty() {
        eprintln!("readall: no patterns provided (try --help)");
        std::process::exit(2);
    }

    let mut files: Vec<PathBuf> = Vec::new();

    for pattern in &args {
        match glob(pattern) {
            Ok(paths) => {
                for p in paths {
                    match p {
                        Ok(path) => {
                            if path.is_file() {
                                files.push(path);
                            }
                        }
                        Err(err) => eprintln!("readall: glob match error for '{pattern}': {err}"),
                    }
                }
            }
            Err(err) => eprintln!("readall: invalid glob pattern '{pattern}': {err}"),
        }
    }

    files.sort();
    files.dedup();

    for path in files {
        let display = path.display();
        match fs::read_to_string(&path) {
            Ok(content) => {
                println!("{display}");
                println!("────────────────────────────────────────");
                println!("{content}");
                println!("════════════════════════════════════════\n");
            }
            Err(err) => {
                eprintln!("readall: cannot read {display}: {err}");
            }
        }
    }
}
