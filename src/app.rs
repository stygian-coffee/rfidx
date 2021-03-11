use std::io::Result;

use walkdir::{DirEntry, WalkDir};

pub struct App {
    file_index: Vec<DirEntry>,
}

impl App {
    pub fn new() -> Self {
        Self { file_index: vec![] }
    }

    pub fn run(&mut self) -> Result<()> {
        self.generate_index()?;

        println!("{:?}", self.file_index);

        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf)?;

        let filtered: Vec<&DirEntry> = self
            .file_index
            .iter()
            .filter(|d| d.file_name().to_str().unwrap().ends_with(".rs"))
            .collect();

        println!("{:?}", filtered);

        Ok(())
    }

    fn generate_index(&mut self) -> Result<()> {
        let walkdir = WalkDir::new(".");
        for f in walkdir.into_iter() {
            self.file_index.push(f?);
        }

        Ok(())
    }
}
