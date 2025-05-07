use colprint::colprint;
use std::fmt::{self, Display, Formatter};

// Example struct with custom Display implementation
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    bio: String,
}

impl Display for Person {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Name: {}\nAge: {}\nBio: {}", self.name, self.age, self.bio)
    }
}

// Another example struct
#[derive(Debug)]
struct Stats {
    title: String,
    values: Vec<f64>,
}

impl Display for Stats {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.title)?;
        for value in &self.values {
            writeln!(f, "{:.2}", value)?;
        }
        Ok(())
    }
}

fn main() {
    // Create example data
    let person = Person {
        name: "Alice Johnson".to_string(),
        age: 30,
        bio: "Software Engineer\nLoves hiking and rock climbing\nBased in Seattle".to_string(),
    };

    let stats = Stats {
        title: "Quarterly Revenue".to_string(),
        values: vec![1234.56, 2345.67, 3456.78, 4567.89],
    };

    // Demonstrate different formatting options
    println!("Example 1: Basic display formatting with auto width");
    colprint!("{}|{}", stats, person);

    println!("\nExample 2: Display formatting with pipe separator");
    colprint!("{} | {}", stats, person);

    println!("\nExample 3: Debug formatting with pipe separator");
    colprint!("{:?} | {:?}", stats, person);

    println!("\nExample 4: Pretty debug formatting with border separator");
    colprint!("{:#?:50} | {:#?:50}", stats, person);

    println!("\nExample 5: Mixed formatting with custom separator");
    colprint!("{} => {:?}", person, stats);

    // More complex example with three columns
    #[derive(Debug)]
    struct Task {
        name: String,
        status: String,
        priority: u8,
    }

    impl Display for Task {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "Task: {}\nStatus: {}\nPriority: {}", self.name, self.status, self.priority)
        }
    }

    let task = Task {
        name: "Complete API implementation".to_string(),
        status: "In Progress".to_string(),
        priority: 1,
    };

    println!("\nExample 6: Three columns with different separators");
    colprint!("{} -> {:?} => {:#?}", person, stats, task);
}
