use std::collections::HashMap;
use std::fs;

fn main() {
    let mut args = std::env::args().skip(1);
    let key = args.next().unwrap();
    let value = args.next().unwrap();

    println!("{}\t{}\n", key, value);

    let mut db = Database::new().expect("Creating DB failed");

    db.insert(key.clone(), value.clone());
    db.insert(key.to_uppercase(), value.clone());
    db.flush().unwrap();
}

struct Database {
    map: HashMap<String, String>,
    flush: bool,
}

impl Database {
    fn new() -> Result<Self, std::io::Error> {
        let mut map = HashMap::new();
        let contents = fs::read_to_string("kv.db")?;

        for line in contents.lines() {
            let (key, value) = line.split_once('\t').expect("Corrupt DB");
            map.insert(key.to_string(), value.to_string());
        }

        Ok(Database { map, flush: false })
    }

    fn insert(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    fn flush(mut self) -> std::io::Result<()> {
        self.flush = true;
        do_flush(&self)
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        if !self.flush {
            let _ = do_flush(self);
        }
    }
}

fn do_flush(db: &Database) -> std::io::Result<()> {
    let mut contents = String::new();

    for (key, value) in &db.map {
        contents.push_str(key);
        contents.push('\t');
        contents.push_str(value);
        contents.push('\n');
    }

    std::fs::write("kv.db", contents)
}
