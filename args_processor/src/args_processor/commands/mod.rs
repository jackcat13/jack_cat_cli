mod git_browse_command;
mod help_command;

use git_browse_command::GitBrowseCommand;
use help_command::HelpCommand;
use std::collections::HashMap;

pub fn build_commands_collection() -> Vec<Box<dyn CommandTrait>> {
    vec![Box::new(HelpCommand {}), Box::new(GitBrowseCommand {})]
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
    fn may_process(&self, options: HashMap<String, String>) {
        let mut help_iter = options.iter().filter(|&(key, _)| key == "--help");
        if let Some((_, value)) = help_iter.next() {
            self.process_help(value.to_string());
            return;
        }
        self.process();
    }
    fn process(&self);
    fn process_help(&self, value: String);
    fn command(&self) -> String;
    fn options(&self) -> HashMap<String, String>;
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
