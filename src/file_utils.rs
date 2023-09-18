use std::{fs, io::{Write,Error}};

pub fn recursive_serach(folder: &str) -> Vec<String> {
    return scan_git_files(folder);
}

pub fn get_dotfiles_path() -> String {
    let mut path = std::env::var("HOME").unwrap();
    path.push_str("/.gitty");
    path
}

fn scan_git_files(path: &str) -> Vec<String> {
    let mut repos: Vec<String> = Vec::new();
    let paths = fs::read_dir(path).unwrap();
    for path in paths {
        let path = path.unwrap().path();
        if path.is_dir() {
            let path = path.to_str().unwrap();
            if path.contains(".git") {
                repos.push(path.to_string());
            } else if path.contains("node_modules")
                || path.contains("target")
                || path.contains("dist")
                || path.contains("build")
            {
                continue;
            } else {
                let mut new_repos = scan_git_files(path);
                repos.append(&mut new_repos);
            }
        }
    }
    repos
}

pub fn parse_file_lines_to_slice(file_path: &str) -> Vec<String> {
    let contents = match fs::read_to_string(file_path) {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Error reading file: {}", err);
            panic!("Error reading file");
        },
    };
    let mut lines: Vec<String> = Vec::new();
    for line in contents.lines() {
        lines.push(line.to_string());
    }
    lines
}

fn dump_slices_to_file(repos: &[&str], file_path: &str) -> Result<(), Error> {
    let mut file = fs::File::open(file_path)?;

    repos.iter().for_each(|repo| {
        file.write(repo.as_bytes()).unwrap();
    });

    Ok(())

}

// possible change the return type
pub fn add_new_slice_elems_to_file(file_path: &str, repos: &[String]) -> Result<(), Error> {
    let existing_repos = parse_file_lines_to_slice(file_path);
    let repos: Vec<&str> = repos
        .iter()
        .filter(|&repo| !existing_repos.contains(repo))
        .map(|repo| repo.as_str())
        .collect();

    dump_slices_to_file(&repos, file_path)
}