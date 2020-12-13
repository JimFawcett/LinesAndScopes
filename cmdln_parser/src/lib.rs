/////////////////////////////////////////////////////////////
// TextFinder::cmdln_parser::lib.rs                        //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 27 Oct 2020  //
/////////////////////////////////////////////////////////////
/// 

/*---------------------------------------------------------
    CmdLnParser assumes that 
    - all odd numbered arguments are options /a, /b, ...
    - all even numbered arguments are values
    - example:
        /P . /p .h,.cpp /p .rs /H false
        P : path option with default value .
        p : file pattern option, default value .*
        H : Hide directories with no match, default true
    Failure to meet those assumptions results in error return 
    from CmdLnParser::parse(args).

    Using code should, on parser error, provide help message
    and terminaate program.
*/
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::vec::Vec;
use std::string::String;

#[derive(Debug)]
pub struct CmdLnParser {
    options: HashMap<char, Vec<String>>,  /*-- /P . /p .h /p .cpp /p .rs --*/
}
impl Default for CmdLnParser {
    fn default() -> Self {
        CmdLnParser::new()
    }
}
impl CmdLnParser {
    pub fn new() -> Self {
        Self {
            options: HashMap::<char, Vec<String>>::new(),
        }
    }
    /*-- used for testing --*/
    pub fn split_item_options(opts: &Vec<String>) -> Vec<String> {
        let mut exp = std::vec::Vec::<&str>::new();
        for i in 0..opts.len() {
            let s = &opts[i];
            let mut temp: Vec<&str> = s.split(',').collect();
            exp.append(&mut temp);
        }
        let mut v = Vec::<String>::new();
        for item in exp {
            v.push(item.to_string());
        }
        v
    }
    pub fn parse(&mut self, args: &Vec<String>) -> Result<char, &str> {
        if args.len() == 1 {
            return Err("no cmdln args");
        }
        let mut i = 1;
        loop /*-- over cmdln args --*/ {
            /*-- expects "/c" where c is option char --*/
            let char1 = args[i].chars()     // returns iterator over string
                        .next()             // one iter step returns Option<char>
                        .ok_or_else(|| "parse error")?;  // unwraps option if Some(ch)

            let key = args[i].chars()
                      .nth(1)               // next char
                      .ok_or_else(|| "parse error")?;
            if (char1 == '/') && (i < args.len() - 1) {
                let val = &args[i+1];
                /*-- expand any comma-separated items --*/
                let new_opt: Vec<String> = 
                        val.split(',')      // returns iterator over splits
                        .map(|s| s.trim().to_string())  // trim outer whitespace
                        .collect();         // collect into vector
                for item in new_opt {
                    self.add_opt(key, &item);
                }
                /*-- skip over value just processed --*/
                i += 2;
                if i >= args.len() {
                    /*-- finished arg list --*/
                    break;
                }
            }
            else {
                /*-- parsing error - incorrect cmdln structure --*/
                return Err("parse error");
            }
        }
        /*-- has correct cmdln structure --*/
        Ok('a')
    }
    pub fn show_cmdln_args() {
        let args: Vec<String> = std::env::args().collect();
        let args = &args[1..];
        print!("\n  {:?}","Command line arguments:\n    ");
        for arg in args {
            print!("{} ", arg);
        }    
    }
    pub fn add_opt(&mut self, key:char, value:&str) -> Option<String> {

        match self.options.entry(key) {
            Entry::Occupied(mut entry) => {
                entry.get_mut().push(value.to_string());
                Some(value.to_string())
            }
            Entry::Vacant(entry) => {
                entry.insert(vec!(value.to_string()));
                Some(value.to_string())
            }
        }
    // https://doc.rust-lang.org/stable/std/collections/hash_map/enum.Entry.html
    }
    /*-- returns Option containing ref of key's value --*/
    pub fn values(&self, key:char) -> Option<&Vec<String>> {
        if self.options.contains_key(&key) {
            Some(&self.options[&key])
        }
        else {
            None
        }
    }
    /*-- shows all values associated with key --*/
    pub fn show_values(&self, key:char) {
        if let Some(val) = self.values(key) {
            print!("\n  {:?}", val);
        }
        else {
            print!("\n  key {:?} does not exist", key);
        }
    }
    /*-- slightly larger message than show_values --*/
    pub fn show_values_long(&self, key:char) {
        if let Some(val) = self.values(key) {
            print!("\n  value of key {} is {:?}", key, val);
        }
        else {
            print!("\n  key {:?} does not exist", key);
        }
    }
    /*-- returns reference to parser's options HashMap --*/
    pub fn options(&self) -> &HashMap<char, Vec<String>> {
        &self.options
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
