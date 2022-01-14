pub mod args_processor{

    use std::collections::HashMap;

    const NO_ARG_PROVIDED_ERROR: &str = "No arguments provided!";
    const OPTIONS_WITH_NO_VALUE_ERROR: &str = "No value provided for an option!";

    pub fn check_arguments_number(number: usize) -> bool{
        if number == 0 { panic!("{}", NO_ARG_PROVIDED_ERROR.to_string()) }
        else { return true }
    }

    pub fn build_args_processor(arguments: Vec<String>) -> ArgsProcessor {
        let mut args_processor = ArgsProcessor::default();
        let arguments_iter = arguments.iter();
        let mut current_option = "";
        for arg in arguments_iter {
            if args_processor.command == "" { args_processor.command = String::from(arg) }
            else if current_option == String::from("") {
                current_option = arg;
            }
            else if current_option != "" {
                args_processor.options.insert(String::from(current_option), String::from(arg));
                current_option = "";
            }
        }
        if current_option != "" { panic!("{}", OPTIONS_WITH_NO_VALUE_ERROR.to_string()) }
        return args_processor;
    }

    #[derive(Default)]
    #[derive(Debug)]
    pub struct ArgsProcessor{
        pub command: String,
        pub options: HashMap<String, String>
    }

    impl ArgsProcessor{

    }
}

#[cfg(test)]
mod tests {

    use std::collections::HashMap;
    use super::args_processor::*;

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
    fn build_args_processor_should_return_empty_processor() {
        let processor = build_args_processor(Vec::new());
        let expected_hash_map: HashMap<String, String> = HashMap::default();
        assert_eq!(processor.command, String::from(""));
        assert_eq!(processor.options, expected_hash_map);
    }

    #[test]
    fn build_args_processor_should_return_processor_with_command() {
        let command = || String::from("cmdTest");
        let processor = build_args_processor(vec![command()]);
        let expected_hash_map: HashMap<String, String> = HashMap::default();
        assert_eq!(processor.command, String::from(command()));
        assert_eq!(processor.options, expected_hash_map);
    }

    #[test]
    fn build_args_processor_should_return_processor_with_options() {
        let processor = build_args_processor(vec!("cmdTest".to_string(), "--optionTest".to_string(), "value".to_string()));
        let mut expected_hash_map: HashMap<String, String> = HashMap::default();
        expected_hash_map.insert(String::from("--optionTest"), String::from("value"));
        assert_eq!(processor.command, String::from("cmdTest"));
        assert_eq!(processor.options, expected_hash_map);
    }

    #[test]
    #[should_panic(expected = "No value provided for an option!")]
    fn build_args_processor_should_return_error_with_option_without_value() {
        build_args_processor(vec!("cmdTest".to_string(), "--optionTest".to_string()));
    }
}
