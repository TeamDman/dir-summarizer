use ignore::WalkBuilder;
use std::path::PathBuf;
use std::env;
use std::process;

fn summarize_workspace(path: PathBuf) -> String {
    let mut summary = String::new();
    let mut ps1_count = 0;

    for result in WalkBuilder::new(path.clone()).build() {
        if let Ok(entry) = result {
            if entry.file_type().map_or(false, |ft| ft.is_file())
                // && entry.path().extension() == Some(OsStr::new("ps1"))
            {
                ps1_count += 1;
                let relative_path = entry.path().strip_prefix(&path).unwrap_or(entry.path());
                summary.push_str(&format!(
                    "=====BEGIN {:?}\n{}\n=====END {:?}\n",
                    relative_path,
                    std::fs::read_to_string(entry.path()).unwrap_or_default(),
                    relative_path
                ));
            }
        }
    }

    format!(
        "Found {} PowerShell scripts in {:?}\n{}",
        ps1_count, path, summary
    )
}
fn main() {
    // Get the first command line argument as a PathBuf, defaulting to the current directory if none is provided
    let args: Vec<String> = env::args().collect();
    let path = args.get(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| env::current_dir().unwrap_or_else(|err| {
            eprintln!("Error getting current directory: {}", err);
            process::exit(1);
        }));
    println!("Path to summarize: {:?}", path);

    let summary = summarize_workspace(path);
    println!("{}", summary);
}
