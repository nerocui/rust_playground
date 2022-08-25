mod student;

fn main() {
    let student = student::Student::new(
        "John".to_string(), 
        18, 
        1, 
        "A".to_string(), 
        123456
    );
    println!("{}", student);
}
