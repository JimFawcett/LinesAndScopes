/////////////////////////////////////////////////////////////
// TextFinder::cmdln_parser::test1.rs                      //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 27 Oct 2020  //
/////////////////////////////////////////////////////////////
/*
    test1:
    Test HashMap operations and collecting cmdln args
*/
// use cmdln_parser::*;
use std::collections::HashMap;
// use std::collections::hash_map::Entry;
use std::vec::Vec;
use std::string::String;

fn main() {
    print!("\n  {}\n","<-- cmdln_parser::test1 -->");

    print!("\n  test hashmap insertions");
    let mut hm = HashMap::<char, String>::new();
    print!("\n  {:?}", hm);
    hm.insert('a', "first insert".to_string());
    print!("\n  {:?}", hm);
    hm.insert('a', "second insert".to_string());
    print!("\n  {:?}", hm);
    print!("\n\n  {}","testing entry('z') - key doesn't exist");
    hm.entry('z');     // nothing happens: no panic, nothing added to hm - that's good
    print!("\n  {:?}\n    - no change - that's good\n", hm);

    print!("\n  testing collection of command line args:\n    ");
    let args: Vec<String> = std::env::args().collect();
    let args = &args[1..];
    for arg in args {
        print!("{:?} ", arg);
    }
    print!("\n\n  That's all Folks!\n\n");
}
