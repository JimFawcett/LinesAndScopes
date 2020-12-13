/////////////////////////////////////////////////////////////
// TextFinder::cmdln_parser::test2.rs                      //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 27 Oct 2020  //
/////////////////////////////////////////////////////////////
/*
    test2:
    Test 
*/
use cmdln_parser::*;
use std::collections::HashMap;
// use std::collections::hash_map::Entry;
use std::vec::Vec;
use std::string::String;

fn main() {
    print!("\n  {}\n","<-- cmdln_parser::test2 -->");

    print!("\n  -- hashmap insertion --");
    let mut hm = HashMap::<char, String>::new();
    print!("\n  {:?}", hm);
    hm.insert('a', "first insert".to_string());
    print!("\n  {:?}", hm);
    hm.insert('a', "second insert".to_string());
    print!("\n  {:?}", hm);
    hm.entry('z');     // nothing happens: no panic, nothing added to hm - that's good
    print!("\n  {:?}\n    - no change - that's good\n", hm);

    print!("\n  {:?}","-- test padding options --");
    let mut cmp = CmdLnParser::new();
    cmp.add_opt('a', "a's value");
    cmp.show_values('a');    

    cmp.add_opt('b', "b's value");
    cmp.show_values('b');
    cmp.add_opt('d', "a d value");
    cmp.add_opt('d', "another d value");
    cmp.show_values('d');

    print!("\n  {:?}", cmp.options());

    cmp.show_values('r');
    cmp.show_values('s');

    print!("-- test collecting cmdln args --");
    let args: Vec<String> = std::env::args().collect();
    let args = &args[1..];
    for arg in args {
        print!("{} ", arg);
    }
    print!("\n\n  That's all Folks\n\n");
}
