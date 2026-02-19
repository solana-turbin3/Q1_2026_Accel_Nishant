use std::{
    collections::VecDeque,
    env,
    fs::{self, File},
    io::{Read, Write},
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

use borsh::{BorshDeserialize, BorshSerialize};

const STORAGE_FILE: &str = "todos.bin";

/// =======================================================
/// 1️⃣ Todo Struct
/// =======================================================

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
struct Todo {
    id: u64,
    description: String,
    created_at: u64,
}

/// =======================================================
/// 2️⃣ Generic Queue
/// =======================================================

pub struct Queue<T> {
    items: VecDeque<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Self {
            items: VecDeque::new(),
        }
    }

    pub fn enqueue(&mut self, item: T) {
        self.items.push_back(item);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.items.pop_front()
    }

    pub fn peek(&self) -> Option<&T> {
        self.items.front()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

/// =======================================================
/// 3️⃣ Persistence
/// =======================================================

impl<T> Queue<T>
where
    T: BorshSerialize + BorshDeserialize,
{
    pub fn save_to_file(&self) {
        let bytes = borsh::to_vec(self).expect("Serialization failed");
        std::fs::write(STORAGE_FILE, bytes).expect("Write failed");
    }

    pub fn load_from_file() -> Self {
        if !std::path::Path::new(STORAGE_FILE).exists() {
            return Self::new();
        }

        let bytes = std::fs::read(STORAGE_FILE).expect("Read failed");

        if bytes.is_empty() {
            return Self::new();
        }

        borsh::from_slice(&bytes).expect("Deserialization failed")
    }
}

/// Enable Borsh for Queue<T>
impl<T> BorshSerialize for Queue<T>
where
    T: BorshSerialize,
{
    fn serialize<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        self.items.serialize(writer)
    }

}

impl<T> BorshDeserialize for Queue<T>
where
    T: BorshDeserialize,
{
    fn deserialize_reader<R: Read>(reader: &mut R) -> std::io::Result<Self> {
        let items = VecDeque::<T>::deserialize_reader(reader)?;
        Ok(Self { items })
    }
}

/// =======================================================
/// 4️⃣ CLI Logic
/// =======================================================

fn main() {
    let mut queue: Queue<Todo> = Queue::load_from_file();

    let args: Vec<String> = env::args().collect();

    println!("Args: {:?}", args);
    if args.len() < 2 {
        println!("Usage:");
        println!("todo add \"Task description\"");
        println!("todo list");
        println!("todo done");
        return;
    }

    match args[1].as_str() {
        "add" => {
            if args.len() < 3 {
                println!("Provide a task description.");
                return;
            }

            let description = args[2].clone();
            let id = generate_id();
            let created_at = current_timestamp();

            let todo = Todo {
                id,
                description,
                created_at,
            };

            queue.enqueue(todo);
            queue.save_to_file();

            println!("Task added.");
        }

        "list" => {
            if queue.is_empty() {
                println!("No tasks.");
                return;
            }

            for todo in queue.items.iter() {
                println!(
                    "[{}] {} (created at {})",
                    todo.id, todo.description, todo.created_at
                );
            }
        }

        "done" => {
            match queue.dequeue() {
                Some(todo) => {
                    println!("Completed: {}", todo.description);
                    queue.save_to_file();
                }
                None => println!("No tasks to complete."),
            }
        }

        _ => {
            println!("Unknown command.");
        }
    }
}

/// =======================================================
/// 5️⃣ Helpers
/// =======================================================

fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn generate_id() -> u64 {
    current_timestamp()
}