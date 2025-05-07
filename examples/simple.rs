use colprint::colprint;

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    country: String,
    job: String,
    hobby: String,
}

impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Name: {}\nAge: {}\nCountry: {}\nJob {}\nHobby {}",
            self.name, self.age, self.country, self.job, self.hobby
        )
    }
}

fn main() {
    let bob = Person {
        name: "Bob".to_string(),
        age: 25,
        country: "Canada".to_string(),
        job: "Data Scientist".to_string(),
        hobby: "Photography".to_string(),
    };
    let jessica = Person {
        name: "Jessica".to_string(),
        age: 28,
        country: "USA".to_string(),
        job: "Software Engineer".to_string(),
        hobby: "Hiking".to_string(),
    };

    colprint!("{:#?}\t{:#?}", bob, jessica);
}
