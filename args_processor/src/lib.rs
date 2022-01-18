pub mod args_processor {

    mod commands;
    use commands::build_commands_collection;
    use std::collections::HashMap;

    const NO_ARG_PROVIDED_ERROR: &str = "No arguments provided!";
    const OPTIONS_WITH_NO_VALUE_ERROR: &str = "No value provided for an option!";
    const INVALID_COMMAND_ERROR: &str =
        "Invalid command. Either the command or the options are incorrect!";

    pub fn check_arguments_number(number: usize) -> bool {
        if number == 0 {
            panic!("{}", NO_ARG_PROVIDED_ERROR.to_string())
        } else {
            true
        }
    }

    pub fn build_args_processor(arguments: Vec<String>) -> ArgsProcessor {
        let mut args_processor = ArgsProcessor {
            command: String::from(""),
            options: HashMap::new(),
        };
        let arguments_iter = arguments.iter().filter(|arg| !arg.contains("jack_cat_cli")); //filter to remove script from arguments list
        let mut current_option = String::from("");
        for arg in arguments_iter {
            if args_processor.command == *"" {
                args_processor.command = String::from(arg)
            } else if current_option == *"" {
                current_option = String::from(arg);
            } else if current_option != *"" {
                args_processor
                    .options
                    .insert(current_option, String::from(arg));
                current_option = String::from("");
            }
        }
        if current_option != *"" {
            panic!("{}", OPTIONS_WITH_NO_VALUE_ERROR.to_string())
        }
        args_processor
    }

    #[derive(Default, Debug)]
    pub struct ArgsProcessor {
        pub command: String,
        pub options: HashMap<String, String>,
    }

    impl ArgsProcessor {
        pub fn find_valid_command_and_process(&self) {
            let mut found = false;
            for command in build_commands_collection() {
                if command.command() == self.command && self.validate_options(command.options()) {
                    command.process();
                    found = true;
                }
            }
            if !found {
                panic!("{}", INVALID_COMMAND_ERROR.to_string())
            };
        }

        fn validate_options(&self, options: HashMap<String, String>) -> bool {
            let keys: Vec<String> = options.keys().cloned().collect();
            for option in self.options.keys() {
                if keys.contains(option) || keys.is_empty() {
                    return false;
                }
            }
            true
        }
    }
}

#[cfg(test)]
mod tests {

    use super::args_processor::*;
    use std::collections::HashMap;

    #[test]
    fn check_arguments_number_should_return_true_when_stricly_positive() {
        assert_eq!(check_arguments_number(1), true);
    }

    #[test]
    #[should_panic(expected = "No arguments provided!")]
    fn check_arguments_number_should_return_panic_error_when_zero() {
        check_arguments_number(0);
    }

    #[test]
    fn build_args_processor_should_return_processor_with_command() {
        let command = || String::from("help");
        let processor = build_args_processor(vec![command()]);
        let expected_hash_map: HashMap<String, String> = HashMap::default();
        assert_eq!(processor.command, String::from(command()));
        assert_eq!(processor.options, expected_hash_map);
    }

    #[test]
    fn build_args_processor_should_return_processor_with_options() {
        let processor = build_args_processor(vec![
            "help".to_string(),
            "--optionTest".to_string(),
            "value".to_string(),
        ]);
        let mut expected_hash_map: HashMap<String, String> = HashMap::default();
        expected_hash_map.insert(String::from("--optionTest"), String::from("value"));
        assert_eq!(processor.command, String::from("help"));
        assert_eq!(processor.options, expected_hash_map);
    }

    #[test]
    #[should_panic(expected = "No value provided for an option!")]
    fn build_args_processor_should_return_error_with_option_without_value() {
        build_args_processor(vec!["help".to_string(), "--optionTest".to_string()])
            .find_valid_command_and_process();
    }

    #[test]
    #[should_panic(expected = "Invalid command. Either the command or the options are incorrect!")]
    fn build_args_processor_should_return_error_with_invalid_command() {
        build_args_processor(vec![
            "doesnotexist".to_string(),
            "--optionTest".to_string(),
            "valueTest".to_string(),
        ])
        .find_valid_command_and_process();
    }

    #[test]
    #[should_panic(expected = "Invalid command. Either the command or the options are incorrect!")]
    fn build_args_processor_should_return_error_with_inexisting_option() {
        build_args_processor(vec![
            "help".to_string(),
            "--optionTest".to_string(),
            "valueTest".to_string(),
        ])
        .find_valid_command_and_process();
    }
}
