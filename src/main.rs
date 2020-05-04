use std::{env, io, fs};
use blocks_world::pddl_parser::make_block_problem_from;
use anyhop::BacktrackPreference::*;
use anyhop::BacktrackStrategy::*;
use anyhop::AnytimePlanner;
use std::fs::File;
use std::io::Write;

fn main() -> io::Result<()> {
    let mut results = anyhop::summary_csv_header();
    for file in env::args().skip(1) {
        if file.ends_with("*") {
            let mut no_star = file.clone();
            no_star.pop();
            for entry in fs::read_dir(".")? {
                let entry = entry?;
                let entry = entry.file_name();
                let entry = entry.to_str();
                let entry_name = entry.unwrap();
                if entry_name.starts_with(no_star.as_str()) {
                    assess_file(entry_name, &mut results)?;
                }
            }
        } else {
            assess_file(file.as_str(), &mut results)?;
        }
    }
    let mut output = File::create("results.csv")?;
    write!(output, "{}", results.as_str())?;
    Ok(())
}

fn assess_file(file: &str, results: &mut String) -> io::Result<()> {
    println!("Running {}", file);
    let (start, goal) = make_block_problem_from(file)?;
    for strategy in vec![Alternate(LeastRecent), Steady(LeastRecent), Steady(MostRecent)] {
        let outcome = AnytimePlanner::plan(&start, &goal, None, strategy, &|p| p.len(), 1, true);
        let row = outcome.summary_csv_row(format!("{}_{:?}", file, strategy).as_str());
        print!("{}", row);
        results.push_str(row.as_str());
    }
    Ok(())
}
