pub fn build_commands_collection() -> Vec<Box<dyn CommandTrait>> {
    return vec![Box::new(HelpCommand {})];
}

fn commands_list() -> String {
    let mut list = String::from("");
    for command in build_commands_collection() {
        list.push_str(&command.command());
    }
    return list;
}

pub trait CommandTrait {
    fn process(&self);
    fn command(&self) -> String;
    fn options(&self) -> Vec<String>;
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
    fn options(&self) -> Vec<String> {
        vec![]
    }
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
        let expected_options: Vec<String> = vec![];
        assert_eq!(help_command.options(), expected_options);
    }
}
