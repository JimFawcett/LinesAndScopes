/////////////////////////////////////////////////////////////
// LinesAndScopes::file_anal::lib.rs                       //
//   - find '{', "fn ", and number of lines                //
// Jim Fawcett, https://JimFawcett.github.io, 26 Oct 2020  //
/////////////////////////////////////////////////////////////

use std::path::Path;
use std::fs::{OpenOptions};
use std::io::*;

#[derive(Debug, Copy, Clone, Default)]
pub struct FileAnal {
    scope_count: usize,
    line_count: usize,
    fn_count: usize
}

impl FileAnal {
    pub fn new() -> Self {
        Self {
            scope_count: 0,
            line_count: 0,
            fn_count: 0
        }
    }
    pub fn get_scopes(line:&str) -> usize {
        let mut local_scope_count = 0;
        for ch in line.chars() {
            if ch == '{' {
                local_scope_count += 1;
            }
        }
        local_scope_count
    }
    pub fn do_anal(&mut self, p:&Path) -> bool {

        let rslt = OpenOptions::new().read(true).open(p);
        if rslt.is_err() {
            return false;
        }
        let file = rslt.unwrap();
        let mut buf_rdr = BufReader::new(file);
        loop {
            let mut line = String::new();
            let rslt = buf_rdr.read_line(&mut line);
            self.line_count += 1;
            if rslt.is_err() { break; }
            let len = rslt.unwrap();
            if len == 0 { break; }
            if line.contains("fn ") {
                self.fn_count += 1;
            }
            self.scope_count += Self::get_scopes(&line);
        }
        true
    }
    pub fn lines(&self) -> usize {
        self.line_count
    }
    pub fn scopes(&self) -> usize {
        self.scope_count
    }
    pub fn functions(&self) -> usize {
        self.fn_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn make_test_file(name:&str) -> bool {
        let rslt = std::fs::File::create(name);
        if rslt.is_ok() {
            let mut test_file = rslt.unwrap();            
            let _rslt = test_file.write_all(b"fn foo() {{}}\n");
            let _rslt = test_file.write_all(b"fn bar() {}\n");
            return true;
        }
        false
    }
    #[test]
    fn test_lines() {
        let _rslt = make_test_file("test1.txt");
        let mut file_anal = FileAnal::new();
        let _ = file_anal.do_anal(Path::new("test1.txt"));
        assert_eq!(file_anal.line_count,3);
    }
    #[test]
    fn test_scopes() {
        let _rslt = make_test_file("test2.txt");
        let mut file_anal = FileAnal::new();
        let _ = file_anal.do_anal(Path::new("test2.txt"));
        assert_eq!(file_anal.scope_count,3);
    }
    #[test]
    fn test_functions() {
        let _rslt = make_test_file("test3.txt");
        let mut file_anal = FileAnal::new();
        let _ = file_anal.do_anal(Path::new("test3.txt"));
        assert_eq!(file_anal.fn_count,2);
    }
}
