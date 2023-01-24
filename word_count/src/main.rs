use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args()
        .skip(1)
        .next()
        .unwrap_or(String::from("test.txt"));
    let contents = std::fs::read_to_string(path)?;
    let counter = Counter::new(&contents);

    loop {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer)?;
        let word = buffer.trim();
        println!("count: {}", counter.get_count(word));
        buffer.clear();
    }
}

struct Counter<'a> {
    words: HashMap<&'a str, usize>,
}

impl<'a> Counter<'a> {
    fn new(file_contents: &'a str) -> Counter<'a> {
        let mut words = HashMap::<&str, usize>::new();
        for word in file_contents.split(" ") {
            *words.entry(word).or_default() += 1;
        }

        Self { words }
    }

    fn get_count(&self, word: &str) -> usize {
        *self.words.get(word).unwrap_or(&0)
    }
}
