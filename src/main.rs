mod classroom;
mod student;
use classroom::{Classroom, CommandError};

fn main() {
    let mut classroom = Classroom::new();

    classroom.execute("add Alice");
    classroom.execute("add Bob");

    classroom.execute("score Alice 80");
    classroom.execute("score Alice 90");

    classroom.execute("status Bob leave");
    classroom.execute("score Bob 100");

    classroom.execute("show Alice");

    classroom.execute("list");

    classroom.execute("remove Bob");

    classroom.execute("list 123");

    classroom.execute("");
    classroom.execute("hello");
    classroom.execute("add");
    classroom.execute("score Alice");
    classroom.execute("status Alice something");
}
