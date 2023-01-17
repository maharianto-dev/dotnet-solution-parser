use std::{error::Error, io::stdin, fs::read_to_string};

use crate::structs::parsed_solution_struct::ParsedSolutionStruct;
mod structs;

// fn main() -> Result<(), Box<dyn Error>> {
//     println!("Enter solution path:");
//     let mut solution_path = String::new();
//     stdin().read_line(&mut solution_path)?;
//     solution_path = solution_path.trim_end().trim_start_matches('\"').trim_end_matches('\"').to_string();
//     println!("\"{}\" entered!", solution_path);
//     println!();
//     parse_solution(&solution_path)?;
//     Ok(())
// }

pub fn parse_solution(path: &str) -> Result<(), Box<dyn Error>> {
    let mut project_list = Vec::new();
    println!("Reading solution file in {}", path);
    let solution_content = read_to_string(path)?;
    println!();
    println!("Contents:");
    let content_lines: Vec<_> = solution_content.lines().collect();
    for content_line in content_lines {
        if content_line.starts_with("Project") {
            let projects = content_line.split(" = ").collect::<Vec<&str>>();
            // check if project is a folder solution if yes the skip
            let mut project_type_id = projects[0].to_string();
            project_type_id = project_type_id.trim_start_matches("Project(\"{").to_string();
            project_type_id = project_type_id.trim_end_matches("}\")").to_string();
            // only register if it is not type of solution folder
            if project_type_id != "2150E333-8FDC-42A3-9474-1A3956D46DE8" {
                project_list.push(ParsedSolutionStruct::init_from_sln_project_line_str(&(projects[1].to_string())));
            }
        }
    }

    let project_count = &project_list.len();

    for proj in &project_list {
        println!("{:?}", proj);
    }
    println!("Found {} projects in solution", project_count);
    Ok(())
}