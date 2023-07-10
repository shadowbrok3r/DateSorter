use std::fs;
use std::path::Path;
use std::io::stdin;
use chrono::{DateTime, NaiveDateTime, Utc};
use walkdir::WalkDir;
use filetime::FileTime;
use indicatif::{ProgressBar, ProgressStyle};

fn main() {
    // Prompt the user for a directory path
    let mut dir_path = String::new();
    println!("Please enter a directory path:");
    stdin().read_line(&mut dir_path).expect("Failed to read line");

    // Trim the newline character from the directory path
    let dir_path = dir_path.trim_end();

    // Count total number of files
    let total_files = WalkDir::new(dir_path).into_iter().filter_map(Result::ok).filter(|e| !e.path().is_dir()).count();

    // Create a new progress bar
    let bar = ProgressBar::new(total_files as u64);
    let sty = ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}")
        .unwrap();
    bar.set_style(sty);

    for entry in WalkDir::new(dir_path) {
        let entry = entry.unwrap();
        let path = entry.path();

        // Skip if it's a directory
        if path.is_dir() {
            continue;
        }

        // Get the creation date of the file
        let metadata = fs::metadata(path).unwrap();
        let filetime_mod = FileTime::from_last_modification_time(&metadata);
        let unix_time = filetime_mod.unix_seconds();
        let naive_datetime = NaiveDateTime::from_timestamp_opt(unix_time, 0).expect("Invalid timestamp");
        let creation_date = DateTime::<Utc>::from_utc(naive_datetime, Utc).format("%m.%d.%Y").to_string();

        // Create a new directory with the creation date as the name
        let new_dir_path = Path::new(dir_path).join(&creation_date);
        fs::create_dir_all(&new_dir_path).unwrap();

        // Move the file to the new directory
        let new_file_path = new_dir_path.join(path.file_name().unwrap());
        fs::rename(path, new_file_path).unwrap();

        // Increment the progress bar
        bar.inc(1);
    }

    // Finish the progress bar
    bar.finish();
}
