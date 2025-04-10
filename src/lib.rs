use std::process::Command;
use anyhow::{Result, anyhow};
use binaryfile::BinaryReader;
use regex::Regex;
use std::env;
use std::ffi::OsString;

//const DEFAULT_TEPRA_PATH:OsString = OsString::from(r#"c:\Program Files (x86)\KING JIM\TEPRA SPC10\SPC10.exe"#);

macro_rules! cwd {
    () => {
        std::env::current_dir().unwrap().to_string_lossy().to_string()
    };
}


pub struct TEPRA {
    //pub tepra_path: String,
    pub tepra_path: OsString,
    pub tpe_path: String,
    pub csv_path: String,
    pub tmp_path: String,
    pub print_count: u32,
    pub required_tape_size: u32,
}


impl TEPRA {

    pub fn new(tepra_path: Option<&str>) -> Self {
        let default_tepra_path:OsString = OsString::from(r#"c:\Program Files (x86)\KING JIM\TEPRA SPC10\SPC10.exe"#);
        let tepra_path = tepra_path.unwrap_or(DEFAULT_TEPRA_PATH);

        let tmp = format!("{}\\tepesize.txt", std::env::var("TEMP").unwrap_or_else(|_| "c:\\".to_string()));
        Self {
            tepra_path : tepra_path.to_string().into(),
            tpe_path : "".to_string(),
            csv_path: "".to_string(),
            tmp_path: tmp,
            print_count: 1,
            required_tape_size: 0,
        }
    }

    pub fn tpe(mut self, tpe_path: &str) -> Self {
        self.tpe_path = tpe_path.to_string();
        self
    }

    pub fn csv(mut self, csv_path: &str) -> Self {
        self.csv_path = csv_path.to_string();
        self
    }

    pub fn tmp(mut self, tmp_path: &str) -> Self {
        self.tmp_path = tmp_path.to_string();
        self
    }

    pub fn print_count(mut self, print_count: u32) -> Self {
        self.print_count = print_count;
        self
    }

    pub fn tape_size(mut self, tape_size: u32) -> Self {
        self.required_tape_size = tape_size;
        self
    }


    pub fn print(&self) -> Result<()> {

        if self.required_tape_size != 0 {

            let param = OsString::from(format!(r#"{},{},{},/GT {}"#,
                self.tpe_path,
                self.csv_path,
                self.print_count,
                self.tmp_path
                ));


            println!("{:?}", &self.tepra_path);
            println!("{:?}", param);

            let mut ret = Command::new("cmd")
                .arg("/C")
                .arg(self.tepra_path.clone())
                .arg("/p")
                .arg(param)
                //.spawn()?;
                .output()?;
            //let _ = ret.wait()?;
            

            // size.txt utf16 -> utf8
            let bin = BinaryReader::open(&self.tmp_path)?.read();
            let utf16data: Vec<u16> = bin.unwrap()
                .chunks(2).map(|c| u16::from_le_bytes([c[0], c[1]])).collect();

            let utf8data = String::from_utf16_lossy(&utf16data);

            std::fs::remove_file(&self.tmp_path)?;

            // check tape size
            let pattern = format!(" {}mm", self.required_tape_size);
            let re = Regex::new(&pattern).unwrap();
            if re.is_match(&utf8data) == false {
                return Err(anyhow!("not required tape size"));
            }
        }




        let param = format!("{},{},{}",
            self.tpe_path,
            self.csv_path,
            self.print_count
            );

        println!("{:?}", &self.tepra_path);
        println!("{:?}", param);

        let mut ret = Command::new(&self.tepra_path)
            .args(&["/p", &param])
            //.spawn()?;
            .output()?;
        //let _ = ret.wait()?;
        Ok(())
    }
}


#[test]
fn tepra_test() {
    
    let tpe_file = &format!(r#"{}\test.tpe"#, cwd!());
    let csv_file = &format!(r#"{}\test.csv"#, cwd!());

    let tepra = TEPRA::new(None)
        .tpe(tpe_file)
        .csv(csv_file)
        .print_count(1)
        .tape_size(6);

    match tepra.print() {
        Ok(_) => println!("print ok"),
        Err(e) => println!("*** {:?} ***", e.to_string())
    }
}

