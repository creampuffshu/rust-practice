mod classroom;
mod student;
mod summary;
use classroom::{Classroom, CommandError};

use crate::summary::Summary;

fn main() {
    let mut classroom = Classroom::new();

    execute(&mut classroom, "add Alice");
    execute(&mut classroom, "add Bob");

    execute(&mut classroom, "score Alice 80");
    execute(&mut classroom, "score Alice 90");

    execute(&mut classroom, "status Bob leave");
    execute(&mut classroom, "score Bob 100");

    execute(&mut classroom, "show Alice");

    execute(&mut classroom, "list");

    execute(&mut classroom, "remove Bob");
    execute(&mut classroom, "list 123");
    execute(&mut classroom, "");
    execute(&mut classroom, "hello");
    execute(&mut classroom, "add");
    execute(&mut classroom, "score Alice");
    execute(&mut classroom, "status Alice something");

    println!("{}", classroom.summary());
}

fn execute(classroom: &mut Classroom, command: &str) {
    if let Err(err) = classroom.execute(command) {
        println!("{err:?}");
    }
}
