use super::commands_list;
use super::CommandTrait;
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct HelpCommand {}
impl CommandTrait for HelpCommand {
    fn process(&self) {
        println!("usage : jack_cat_cli <command> [OPTIONS]");
        println!("Available commands: {}", commands_list());
        println!("To know options of a particular command, please add --help [simple/verbose] option after the <command>.");
    }
    fn process_help(&self, _value: String) {}
    fn command(&self) -> String {
        String::from("help")
    }
    fn options(&self) -> HashMap<String, String> {
        HashMap::new()
    }
}
