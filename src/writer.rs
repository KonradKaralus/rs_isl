use std::{fs, path::Path};

pub struct Writer {
    path: String,
    idx: usize,
}

impl Writer {
    pub fn new() -> Self {
        let path = "output".to_string();
        let _ = fs::create_dir(Path::new(&path));

        Self { path, idx: 0 }
    }

    pub fn write(&mut self, data: Vec<Vec<f64>>) {
        // println!("write called");
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
