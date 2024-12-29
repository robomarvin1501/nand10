use std::{
    env,
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

mod parser;
mod token_stream;
mod tokeniser;
mod tokens;

static JACK_FILE_EXTENSION: &'static str = "jack";
static VM_FILE_EXTENSION: &'static str = "vm";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Invalid usage, please use: JackAnalyzer <input path>")
    }
    let argument_path = env::args().nth(1).expect("No path provided");
    let argument_path = fs::canonicalize(&argument_path).expect("Invalid path provided");

    let files_to_compile: Vec<PathBuf> = if argument_path.is_dir() {
        fs::read_dir(&argument_path)
            .expect("Failed to read directory")
            .filter_map(|entry| entry.ok().map(|e| e.path()))
            .collect()
    } else {
        vec![argument_path]
    };

    compile_files(files_to_compile);
}

fn compile_files(input_paths: Vec<PathBuf>) {
    for input_path in input_paths {
        if let Some(extension) = input_path.extension() {
            if extension.to_str().unwrap_or("").to_lowercase() != JACK_FILE_EXTENSION {
                continue;
            }
        } else {
            continue;
        }

        let input_file = PathBuf::from(&input_path);
        let output_path = create_vm_file_path(&input_path).unwrap();

        compile_file(input_file, &output_path);
    }
}

fn compile_file(input_path: PathBuf, output_path: &PathBuf) {
    let contents: String =
        fs::read_to_string(&input_path).expect("Should have been able to read file");

    // let file_name = &input_path.file_stem().unwrap().to_str().unwrap();
    let input_file_path = &input_path.to_str().unwrap();
    let output_file_path = &output_path.to_str().unwrap();

    // Parse the file
    println!("Parsing: {input_file_path}");
    let xml = parser::parse(contents);
    // Append the output
    println!("Output: {output_file_path}");
    match xml {
        Ok(code) => write_to_file(output_path, vec![code]),
        Err(e) => panic!("Tried to compile, but got error {:?}", e),
    }
}

fn write_to_file(path: &PathBuf, s: Vec<String>) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)
        .unwrap();
    for line in s {
        file.write(line.as_bytes()).unwrap();
    }
}

fn create_vm_file_path(input: &Path) -> Result<PathBuf, String> {
    if !input.is_file() {
        return Err(format!("Input path {:?} is not a file", input));
    }
    // Input is a file, change its extension to VM_FILE_EXTENSION
    let mut new_file_path = input.to_path_buf();
    new_file_path.set_extension(VM_FILE_EXTENSION);
    Ok(new_file_path)
}
