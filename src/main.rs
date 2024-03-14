// Import necessary crates
use clap::{App, Arg};
use tokio::process::Command;
use walkdir::WalkDir;

//  main function (for Tokio, use the #[tokio::main] macro for async support)
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command-line arguments
    let matches = App::new("pylint_runner_rs")
        .version("0.1.0")
        .author("Your Name")
        .about("Runs Pylint on multiple Python files or directories.")
        .arg(Arg::with_name("paths")
             .short('p')
             .long("paths")
             .value_name("FILE_OR_DIR")
             .help("Sets the input files or directories to use")
             .multiple(true)
             .takes_value(true))
        .get_matches();

    // 4. Collect all Python files from the provided paths
    let mut files_to_lint = Vec::new();
    if let Some(paths) = matches.values_of("paths") {
        for path in paths {
            for entry in WalkDir::new(path).into_iter().filter_map(Result::ok) {
                let entry_path = entry.path();
                if entry_path.is_file() && entry_path.extension().map_or(false, |ext| ext == "py") {
                    files_to_lint.push(entry_path.to_path_buf());
                }
            }
        }
    }

    // 5. Execute Pylint on each Python file found
    for file_path in files_to_lint {
        let output = Command::new("pylint")
            .arg(file_path.to_str().unwrap())
            .output()
            .await?;

        // Handle the output, for example, printing it
        println!("Pylint output for {}: {}", file_path.display(), String::from_utf8_lossy(&output.stdout));
    }

    Ok(())
}
