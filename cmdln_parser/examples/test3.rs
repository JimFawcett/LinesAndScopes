/////////////////////////////////////////////////////////////
// TextFinder::cmdln_parser::test3.rs                      //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 27 Oct 2020  //
/////////////////////////////////////////////////////////////
/*
   Test HashMap behavior and collection of cmdln args
*/

use cmdln_parser::*;
use std::vec::Vec;
use std::string::String;

fn main() {

    print!("\n  {}\n","<-- cmdln_parser::test3 -->");

    print!("\n  {}","-- adding options --");
    let mut cmp = CmdLnParser::new();
    cmp.add_opt('a', "a's value");
    cmp.show_values('a');    

    cmp.add_opt('b', "b's value");
    cmp.show_values('b');
    cmp.add_opt('d', "a d value");
    cmp.add_opt('d', "another d value");
    cmp.show_values('d');

    print!("\n  {:?}", cmp.options());

    print!("\n\n  {}","test non-existing keys");
    cmp.show_values('r');
    cmp.show_values('s');

    print!("\n\n  {}\n", "-- test expanding options --");
    let v = vec![
        "a".to_string(), 
        "b,c".to_string(), 
        "d,e,f".to_string(), 
        "g".to_string()
    ];
    print!("\n  {:?}", v);
    print!("\n  {:?}", CmdLnParser::split_item_options(&v));

    print!("\n\n  {}","-- test parsing command line args --");

    print!("\n\n  Command line args are:\n  ");
    let args: Vec<String> = std::env::args().collect();
    let args = &args[1..];
    for arg in args {
        print!("{:?} ", arg);
    }

    let mut cmp = CmdLnParser::new();
    let args: Vec<String> = std::env::args().collect();
    let rslt = cmp.parse(&args);
    if rslt.is_err() {
        print!("\n  error: {:?}",rslt.unwrap_err());
        return;
    }
    print!("\n  {:?}","parsed options are:");
    print!("\n  {:#?}",cmp.options());

    let key = 'b';
    cmp.show_values_long(key);

    print!("\n\n  That's all Folks!\n\n");
}
