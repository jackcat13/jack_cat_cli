use std::env;
use args_processor::args_processor::check_arguments_number;
use args_processor::args_processor::build_args_processor;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args_number = args.len()-1;
    if check_arguments_number(args_number) {
        let args_processor = build_args_processor(args);
    }

}