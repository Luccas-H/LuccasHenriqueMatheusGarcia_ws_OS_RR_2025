use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand::{thread_rng, Rng};

// Estrutura que representa a mesa com os garfos
struct Table {
    forks: Vec<Arc<Mutex<()>>>,
}

impl Table {
    /// Cria uma nova mesa com a quantidade de garfos especificada
    fn new(num_forks: usize) -> Self {
        let forks = (0..num_forks)
            .map(|_| Arc::new(Mutex::new(())))
            .collect();
        Table { forks }
    }
}

// Estrutura que representa um filósofo
struct Philosopher {
    id: usize,
    table: Arc<Table>,
}

impl Philosopher {
    /// Cria um novo filósofo com um id e referência à mesa
    fn new(id: usize, table: Arc<Table>) -> Self {
        Philosopher { id, table }
    }

    /// Cria uma thread para executar as ações do filósofo
    fn spawn(self) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            self.exist();
        })
    }

    /// Loop principal: o filósofo alterna entre pensar e comer
    fn exist(&self) {
        loop {
            self.think();
            self.eat();
        }
    }

    /// Simula o filósofo pensando por um tempo aleatório
    fn think(&self) {
        println!("Filósofo {} pensando..", self.id);
        let ms = thread_rng().gen_range(0..5000);
        thread::sleep(Duration::from_millis(ms));
    }

    /// Simula o filósofo tentando pegar os garfos, comer e depois liberar os garfos
    fn eat(&self) {
        let left = self.id;
        let right = (self.id + 1) % self.table.forks.len();

        let (first, second) = if left < right { (left, right) } else { (right, left) };

        println!(
            "Filósofo {} tentando pegar os garfos {} e {}...",
            self.id, first, second
        );

        let _first_fork = self.table.forks[first].lock().unwrap();
        println!("Filósofo {} pegou o garfo {}", self.id, first);

        let _second_fork = self.table.forks[second].lock().unwrap();
        println!("Filósofo {} pegou o garfo {}", self.id, second);

        println!("Filósofo {} está comendo...", self.id);
        let ms = thread_rng().gen_range(0..10000);
        thread::sleep(Duration::from_millis(ms));

        println!(
            "Filósofo {} devolveu os garfos {} e {}.",
            self.id, first, second
        );
    }
}

/// Função principal: inicializa a mesa e os filósofos, e inicia as threads
fn main() {
    println!("O Jantar dos Filósofos\n");

    let table = Arc::new(Table::new(5));
    let mut handles = Vec::new();

    for i in 0..5 {
        let philosopher = Philosopher::new(i, Arc::clone(&table));
        handles.push(philosopher.spawn());
    }

    for handle in handles {
        let _ = handle.join();
    }
}
