// test1.rs

use std::path::Path;
use file_anal::*;

fn main() {
    print!("\n  {}\n","-- test file_anal --");

    let filename = "./src/lib.rs";
    print!("\n  processing {:?}", filename);

    let mut file_anal = FileAnal::new();
    let _success = file_anal.do_anal(Path::new(filename));

    print!("\n  lines:     {}", file_anal.lines());
    print!("\n  functions: {}", file_anal.functions());
    print!("\n  scopes:    {}", file_anal.scopes());


    print!("\n\n  {}","That's all Folks!\n\n");
}