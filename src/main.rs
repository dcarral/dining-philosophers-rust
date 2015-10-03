use std::thread;
use std::sync::Mutex;

struct Table {
    forks: Vec<Mutex<()>>,
}

struct Philosopher {
    name: String,
}

impl Philosopher {
    fn new(name: &str) -> Philosopher {
        Philosopher {
            name: name.to_string(),
        }
    }

    fn eat(&self) {
        println!("{} is eating.", self.name);

        thread::sleep_ms(1000);

        println!("{} is done eating.", self.name);
    }
}

fn main() {
    let philosophers = vec![
        Philosopher::new("Judith Butler"),
        Philosopher::new("Gilles Deleuze"),
        Philosopher::new("Karl Marx"),
        Philosopher::new("John Locke"),
        Philosopher::new("Michel Foucault"),
    ];

    let handles: Vec<_> = philosophers.into_iter().map( |philosopher| {
        thread::spawn(move || {
            philosopher.eat();
        })
    }).collect();

    for handle in handles {
        handle.join().unwrap();
    }
}
