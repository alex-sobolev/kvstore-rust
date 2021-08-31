use std::collections::HashMap;
use std::fs;

fn main() {
    let mut args = std::env::args().skip(1);
    let key = args.next().unwrap();
    let value = args.next().unwrap();
    let contents = format!("{}\t{}\n", key, value);

    fs::write("kv.db", contents).unwrap();

    let db = Database::new();
}

struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Self, std::io::Error> {
        let mut map = HashMap::new();
        let contents = fs::read_to_string("kv.db")?;

        for line in contents.lines() {
            let (key, value) = line.split_once('\t').expect("Corrupt DB");
            map.insert(key.to_string(), value.to_string());
        }

        Ok(Database { map })
    }
}
