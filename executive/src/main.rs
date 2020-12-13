/////////////////////////////////////////////////////////////
// LinesAndScopes::executive - simple code analysis        //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 13 Dec 2020  //
/////////////////////////////////////////////////////////////
/* 
   LinesAndScopes evaluates:
   - number of lines in each file analyzed including
     comments and whitespace
   - number of functions (Rust only) in file
   - number of scopes for entire file

   It expects a command line like:
     /P .. /p rs,h,cpp
*/
#![allow(dead_code)]
#[allow(unused_imports)]
use cmdln_parser::*;
use dir_nav::*;
use file_anal::*;
use std::path::{Path};

/*--------------------------------------------------------- 
   Generic parameter for analyzer, providing application
   specific handline of files and paths.
*/
#[derive(Debug, Default)]
struct Appl {
    dir: String,
    sum_lines: usize,
    sum_functions: usize,
    sum_scopes: usize,
}
impl Appl {
    pub fn new() -> Self {
        Self {
            dir: String::default(),
            sum_lines: 0,
            sum_functions: 0,
            sum_scopes: 0
        }
    }
    pub fn sum_lines(&self) -> usize { self.sum_lines }
    pub fn sum_functions(&self) -> usize { self.sum_functions }
    pub fn sum_scopes(&self) -> usize { self.sum_scopes }
}
impl DirEvent for Appl {
    fn do_dir(&mut self, dir:&str) {
        print!("\n  {}", dir);
        self.dir = dir.to_string();
    }
    fn do_file(&mut self, file: &str) {
        print!("\n    {}", file);
        let mut anl = FileAnal::new();
        
        /*-- join current dir with file --*/
        let prefix = Path::new(&self.dir);
        let fqp = prefix.join(&Path::new(file));

        let _ = anl.do_anal(&fqp);
        print!("\n    lines:     {}", anl.lines());
        print!("\n    functions: {}", anl.functions());
        print!("\n    scopes:    {}", anl.scopes());
        self.sum_lines += anl.lines();
        self.sum_functions += anl.functions();
        self.sum_scopes += anl.scopes();
    }
}

/*---------------------------------------------------------
   Application Executive
*/
#[derive(Debug)]
struct Exec {
    clp: CmdLnParser,
    anl: FileAnal,
}
impl Exec {
    fn new() -> Self {
        Self {
            clp: CmdLnParser::new(),
            anl: FileAnal::new()
        }
    }
    fn cmdln_parse(&mut self) -> Result<char,&str> {
        let args: Vec<String> = std::env::args().collect();
        let parser = &mut self.clp;
        parser.parse(&args)?;
        Ok('a')
    }
    fn show_help(&self) {
        print!("\n  FileAnalyzer expects on command line:");
        print!("\n    /P .            -- path to search");
        print!("\n    /p .rs,.h,.cpp  -- file patterns");
        print!("\n    [/h true]       -- display this message\n");
    }
    fn show_cmdln_parse(&self) {
        if self.clp.options().contains_key(&'h') {
            self.show_help();
            print!("\n  quitting\n\n");
            std::process::exit(0);
        }
        print!("\n  Options parsed from cmdln: char:Vec<String>):;");
        for key in self.clp.options().keys() {
            print!("\n    {}:{:?}", key, self.clp.options()[key]);
        }
    }
    fn analyze(&self) {
        let start = &self.clp.options()[&'P'][0];
        let spath = std::path::Path::new(start);
        let mut dn = DirNav::<Appl>::new();
        let patterns = &self.clp.options()[&'p'];
        for pat in patterns {
            dn.add_pat(pat);
        }
        let _ = dn.visit(spath);
        let app = dn.get_app();
        println!();
        print!("\n  Total lines:     {}", app.sum_lines());
        print!("\n  Total functions: {}", app.sum_functions());
        print!("\n  Total scopes:    {}", app.sum_scopes());    
    }
}
fn main() {

    print!("\n  LinesAndScopes: ver 1.0");
    print!("\n --------------------------");
    let mut exec = Exec::new();
    let rslt = exec.cmdln_parse();
    if rslt.is_err() {
        print!("\n  cmdln parse error - quitting\n");
        return;
    }
    exec.show_cmdln_parse();
    println!();
    exec.analyze();

    print!("\n\n  That's all Folks\n\n");
}
