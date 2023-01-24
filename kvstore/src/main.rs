use std::collections::HashMap;

fn main() {
    let mut arguments = std::env::args().skip(1);

    let key = arguments.next().unwrap();
    let value = arguments.next().unwrap();

    println!("The key is {}", key);
    println!("The value is {}", value);

    let mut database = Database::new().expect("Database::new() crashed");
    database.insert(key.to_uppercase(), value.clone());
    database.insert(key, value);

    database.flush();
}

struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        // Old way to use match patter
        let old_way_contents = match std::fs::read_to_string("kv.db") {
            Ok(c) => c,
            Err(error) => {
                return Err(error);
            }
        };

        // because the previous pattern was so used rust create a new way to do it
        // to stop writing boilerplate code

        let new_way_contents = std::fs::read_to_string("kv.db")?;

        let mut map = HashMap::new();

        for line in new_way_contents.lines() {
            let (key, value) = line.split_once('\t').expect("crash splitting");
            map.insert(key.to_owned(), value.to_owned());
        }

        return Ok(Database { map: map });
    }

    fn insert(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    fn flush(self) -> std::io::Result<()> {
        let mut contents = String::new();
        for (key, value) in self.map {
            let kvpair = format!("{}\t{}\n", key, value);
            contents.push_str(&kvpair);
        }
        std::fs::write("kv.db", contents)
    }
}
