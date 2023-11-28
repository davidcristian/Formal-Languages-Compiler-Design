use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};

pub type InputLine = Option<Result<String, io::Error>>;

pub fn open_file(file_path: &str) -> Result<Lines<BufReader<File>>, String> {
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => {
            let error = format!("could not open file: {}", e.to_string());
            return Err(error);
        }
    };

    let reader = BufReader::new(file);
    let lines = reader.lines();

    Ok(lines)
}

pub fn extract_line_data(line: InputLine) -> Option<String> {
    // we will ignore the errors from the reader in this function
    // because the lack of data will be caught in the handler functions
    // i.e. a missing line will trigger a custom error message

    match line {
        Some(line) => match line {
            Ok(line) => Some(line),
            Err(_) => None,
        },
        None => None,
    }
}

pub fn get_next_line(lines: &mut Lines<BufReader<File>>) -> Result<Option<String>, String> {
    // we are no longer ignoring errors from the reader in this function
    // because the lack of data is a serious error that should be handled
    // i.e. a missing line breaks the entire meaning of the file

    match lines.next() {
        Some(line) => match line {
            Ok(line) => Ok(Some(line)),
            Err(e) => {
                let error = format!("could not read line from file: {}", e.to_string());
                Err(error)
            }
        },
        None => Ok(None),
    }
}
