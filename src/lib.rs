use std::ffi::OsString;
use std::process::Command;
use anyhow::{anyhow, Result};
use binaryfile::BinaryReader;
use regex::Regex;
use std::fs;
use std::path::PathBuf;


#[derive(Default, Debug)]
pub struct TEPRA {
    pub tepra_path: OsString,
    pub tpe_path: OsString,
    pub csv_path: OsString,
    pub size_path: OsString,
    pub num_print: u32,
}



impl TEPRA {
    pub fn new() -> TEPRA {
        TEPRA {
            tepra_path: OsString::from(r#"c:\Program Files (x86)\KING JIM\TEPRA SPC10\SPC10.exe"#),
            num_print: 1,
            ..Default::default()     
        }
    }

    pub fn tepra(mut self, path:OsString) -> Self {
        self.tepra_path = path;
        self
    }

    pub fn tpe(mut self, path:OsString) -> Self {
        self.tpe_path = path;
        self
    }

    pub fn csv(mut self, path:OsString) -> Self {
        self.csv_path = path;
        self
    }

    pub fn size_file(mut self, path:OsString) -> Self {
        self.size_path = path;
        self
    }

    pub fn number_of(mut self, num_of_print:u32) -> Self {
        self.num_print = num_of_print;
        self
    }
    

    pub fn print(&self) -> Result<()> {

        println!("start print");
        
        let param = format!(r#"{},{},{}"#, 
            self.tpe_path.to_string_lossy(),
            self.csv_path.to_string_lossy(), 
            self.num_print 
        );
        println!("pass1");
        println!("{:?}", &param);
        
        let mut child = Command::new(&self.tepra_path)
                .arg("/p")
                .arg(param)
                .spawn()?;

        let _ = child.wait()?;

        Ok(())

    }


    pub fn check(&self, require_size: u32) -> Result<()> {

        let path_buf: PathBuf = PathBuf::from(self.size_path.clone());
        if path_buf.exists() {
            match fs::remove_file(&path_buf) {
                Ok(_) => {},
                Err(e) => return Err(anyhow!("do not erase tepra size file.")),
            }
        }
        
        let param = format!(r#"{},{},{},/GT {}"#, 
            self.tpe_path.to_string_lossy(),
            self.csv_path.to_string_lossy(), 
            self.num_print,
            self.size_path.to_string_lossy()
        );

        let mut child = Command::new(&self.tepra_path)
                .arg("/p")
                .arg(param)
                .spawn()?;

        let _ = child.wait()?;

        // decode
        let size_path = &self.size_path.to_string_lossy();
        let bin = BinaryReader::open(&size_path)?.read();
        let utf16data: Vec<u16> = bin.unwrap()
            .chunks(2).map(|c| u16::from_le_bytes([c[0], c[1]])).collect();
        let utf8data = String::from_utf16_lossy(&utf16data);

        // size check
        let pattern = format!(" {}mm", require_size);
        let re = Regex::new(&pattern).unwrap();
        if re.is_match(&utf8data) == false {
            return Err(anyhow!("not required tape size"));
        }

        Ok(())
    }

}


#[test]
fn tepra_test() {

    let tepra = TEPRA::new()
        .tpe(OsString::from(r#"c:\work\tepra\label6mm.tpe"#))
        .csv(OsString::from(r#"c:\work\tepra\dnum.csv"#))
        .size_file(OsString::from(r#"c:\work\tepra\tapesize.txt"#))
        .number_of(1);



    let res = match tepra.check(6) {

        Ok(_) => {
            println!("ok!");
            tepra.print().unwrap();
        },

        Err(e) => println!("err: {}", e)
    };

}
