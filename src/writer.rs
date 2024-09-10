use std::{fs, path::Path};

pub struct Writer {
    path: String,
    idx: usize,
}

impl Writer {
    pub fn new() -> Self {
        let path = "output/raw".to_string();
        let _ = fs::create_dir(Path::new(&path));

        let files = fs::read_dir(&path).unwrap();

        for file in files {
            let _ = fs::remove_file(file.unwrap().path());
        }

        Self { path, idx: 0 }
    }

    pub fn write(&mut self, data: Vec<Vec<f64>>) {
        let mut out = "".to_string();
        for line in data {
            for n in line {
                out += &(n.to_string() + ",");
            }
            out += "\n";
        }

        fs::write(self.path.clone() + &format!("/file{}", self.idx), out).unwrap();
        self.idx += 1;
    }
}
