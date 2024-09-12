extern crate fitch_proof;
use std::env;

const DEFAULT_ALLOWED_VARIABLE_NAMES: &str = "x,y,z,u,v,w";

fn fail_open_file(filename: &str) -> ! {
    println!(
        "Oops, it seems like the file {} could not be opened. Are you sure it exists? Aborting.",
        filename
    );
    std::process::exit(1)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No file path given as an argument.");
        println!("Usage: {} <path_to_proof.txt>", args[0]);
        return;
    }
    let file_path = &args[1];
    let Ok(proof) = std::fs::read_to_string(file_path) else {
        fail_open_file(file_path)
    };
    let variables = DEFAULT_ALLOWED_VARIABLE_NAMES.to_string();
    let result: String = fitch_proof::check_proof_full(&proof, &variables);
    println!("{}", result);
}
