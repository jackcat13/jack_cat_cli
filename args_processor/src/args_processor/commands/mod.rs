use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use webbrowser;

pub fn build_commands_collection() -> Vec<Box<dyn CommandTrait>> {
    return vec![Box::new(HelpCommand {}), Box::new(GitBrowseCommand {})];
}

fn commands_list() -> String {
    let mut list = String::from("");
    for command in build_commands_collection() {
        list.push('<');
        list.push_str(&command.command());
        list.push_str("> ");
    }
    list
}

pub trait CommandTrait {
    fn process(&self);
    fn command(&self) -> String;
    fn options(&self) -> HashMap<String, String>;
}

#[derive(Default, Debug)]
pub struct HelpCommand {}
impl CommandTrait for HelpCommand {
    fn process(&self) {
        println!("usage : jack_cat_cli <command> [OPTIONS]");
        println!("Available commands: {}", commands_list());
        println!("To know options of a particular command, please add --help [simple/verbose] option after the <command>.");
    }
    fn command(&self) -> String {
        String::from("help")
    }
    fn options(&self) -> HashMap<String, String> {
        HashMap::new()
    }
}

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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn help_command_should_return_command_name_with_generics() {
        let help_command = Box::new(HelpCommand {});
        assert_eq!(help_command.command(), "help");
    }

    #[test]
    fn help_command_should_return_options_with_generics() {
        let help_command = Box::new(HelpCommand {});
        let actual_options: Vec<String> = help_command.options().keys().cloned().collect();
        let expected_options: Vec<String> = vec![];
        assert_eq!(actual_options, expected_options);
    }

    #[test]
    fn git_browse_command_should_return_command_name_with_generics() {
        let git_browse_command = Box::new(GitBrowseCommand {});
        assert_eq!(git_browse_command.command(), "gitBrowse");
    }

    #[test]
    fn git_browse_command_should_return_options_with_generics() {
        let git_browse_command = Box::new(GitBrowseCommand {});
        let actual_options: Vec<String> = git_browse_command.options().keys().cloned().collect();
        let expected_options: Vec<String> = vec!["--help".to_string()];
        assert_eq!(actual_options, expected_options);
    }
}
