mod models;
mod utils;

use utils::scanner::Scanner;

fn main() {
    const PROGRAM_FILE_PATH: &str = "../../1/p3.oli";
    const TOKEN_FILE_PATH: &str = "../../2/token.in";

    let scanner = Scanner::new(TOKEN_FILE_PATH);
    scanner.scan(PROGRAM_FILE_PATH);
}
