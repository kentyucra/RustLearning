use core::num;

pub struct ThreadPool {
    _handles: Vec<std::thread::JoinHandle<()>>,
}

impl ThreadPool {
    pub fn new(num_threads: u8) -> Self {
        let mut handles: Vec<std::thread::JoinHandle<()>> = Vec::new();
        for _ in 0..num_threads {
            let handle = std::thread::spawn(|| {});
            handles.push(handle)
        }
        return Self { _handles: handles };
    }

    pub fn execute<T: Fn(), T2: Fn()>(&self, work: T, work2: T2) {
        println!("pepe");
        work();
        work2()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let pool = ThreadPool::new(10);

        pool.execute(|| println!("Hola PEPE"), || println!("sjsj"));
        pool.execute(|| println!("Hola PEPE"), || println!("asdasd"));
    }
}
