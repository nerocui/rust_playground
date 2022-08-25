use std::fmt;

pub struct Student {
    name: String,
    age: u8,
    grade: u8,
    class: String,
    id: u32,
}

impl Student {
    pub fn new(name: String, age: u8, grade: u8, class: String, id: u32) -> Student {
        Student {
            name: name,
            age: age,
            grade: grade,
            class: class,
            id: id,
        }
    }
}

impl fmt::Display for Student {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "Name: {}\nAge: {}\nGrade: {}\nClass: {}\nID: {}", self.name, self.age, self.grade, self.class, self.id)
    }
}
