use super::CommandTrait;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use webbrowser;

#[derive(Default, Debug)]
pub struct GitBrowseCommand {}
impl CommandTrait for GitBrowseCommand {
    fn process(&self) {
        println!("{} // URL ", retrieve_git_url());
        if webbrowser::open(&retrieve_git_url()).is_ok() {
            println!("Could open the web page properly")
        } else {
            panic!("Could not open the web page!")
        }
    }
    fn command(&self) -> String {
        String::from("gitBrowse")
    }
    fn options(&self) -> HashMap<String, String> {
        let mut options = HashMap::<String, String>::new();
        options.insert(
            "--help".to_string(),
            "[simple/verbose] option to have simple or complete help for the command.".to_string(),
        );
        options
    }
}

fn retrieve_git_url() -> String {
    if let Ok(lines) = read_lines(".git/config") {
        for line in lines.flatten() {
            if line.contains("url = ") {
                return str::replace(&line, "url = ", "").trim().to_string();
            }
        }
    }
    panic!("Could not retrieve git configuration to open url in browser!");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
