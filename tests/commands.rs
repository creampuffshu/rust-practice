use hello_rust::classroom::{Classroom, CommandError};
use hello_rust::student::Status;

fn classroom_with_alice() -> Classroom {
    let mut classroom = Classroom::new();

    classroom.execute("add Alice").unwrap();
    classroom.execute("score Alice 80").unwrap();
    classroom.execute("score Alice 90").unwrap();

    classroom
}

fn assert_alice_unchanged(classroom: &Classroom) {
    assert_eq!(classroom.student_count(), 1);
    assert_eq!(classroom.active_student_count(), 1);

    let alice = classroom.find_student("Alice").unwrap();
    assert_eq!(alice.name(), "Alice");
    assert_eq!(alice.scores(), &[80, 90]);
    assert_eq!(alice.status(), Status::Active);
}

#[test]
fn invalid_argument_counts_are_rejected_without_changing_state() {
    let commands = [
        "",
        "   ",
        "add",
        "add Alice Bob",
        "score Alice",
        "score Alice 80 extra",
        "status Alice",
        "status Alice leave extra",
        "show",
        "show Alice extra",
        "remove",
        "remove Alice extra",
        "list extra",
    ];

    for command in commands {
        let mut classroom = classroom_with_alice();

        let result = classroom.execute(command);

        assert_eq!(
            result,
            Err(CommandError::WrongArgsNum),
            "command: {command:?}"
        );
        assert_alice_unchanged(&classroom);
    }
}

#[test]
fn invalid_commands_return_expected_errors_without_changing_state() {
    let cases = [
        ("unknown", CommandError::NonExistentCommand),
        ("add Alice", CommandError::DuplicatedStudent),
        ("score Alice abc", CommandError::FailParse),
        ("score Alice 2147483648", CommandError::FailParse),
        ("score Nobody 80", CommandError::NotFindStudent),
        ("status Alice unknown", CommandError::WrongStatus),
        ("status Nobody active", CommandError::NotFindStudent),
        ("remove Nobody", CommandError::NotFindStudent),
        ("show Nobody", CommandError::NotFindStudent),
    ];

    for (command, expected_error) in cases {
        let mut classroom = classroom_with_alice();

        let result = classroom.execute(command);

        assert_eq!(result, Err(expected_error), "command: {command:?}");
        assert_alice_unchanged(&classroom);
    }
}

#[test]
fn inactive_student_score_command_preserves_existing_scores() {
    for (status_command, expected_status) in [
        ("status Alice leave", Status::Leave),
        ("status Alice graduated", Status::Graduated),
    ] {
        let mut classroom = classroom_with_alice();
        classroom.execute(status_command).unwrap();

        let result = classroom.execute("score Alice 100");

        assert_eq!(result, Err(CommandError::CannotAddScore));
        assert_eq!(classroom.student_count(), 1);
        assert_eq!(classroom.active_student_count(), 0);

        let alice = classroom.find_student("Alice").unwrap();
        assert_eq!(alice.name(), "Alice");
        assert_eq!(alice.scores(), &[80, 90]);
        assert_eq!(alice.status(), expected_status);
    }
}

#[test]
fn add_command_accepts_extra_whitespace() {
    let mut classroom = Classroom::new();

    let result = classroom.execute("  add   Alice  ");

    assert_eq!(result, Ok(()));
    assert_eq!(classroom.student_count(), 1);
    assert_eq!(
        classroom.find_student("Alice").unwrap().name(),
        "Alice"
    );
}

#[test]
fn show_and_list_succeed_without_changing_state() {
    let mut classroom = classroom_with_alice();

    assert_eq!(classroom.execute("show Alice"), Ok(()));
    assert_alice_unchanged(&classroom);

    assert_eq!(classroom.execute("list"), Ok(()));
    assert_alice_unchanged(&classroom);
}

#[test]
fn list_succeeds_for_empty_classroom() {
    let mut classroom = Classroom::new();

    assert_eq!(classroom.execute("list"), Ok(()));
    assert_eq!(classroom.student_count(), 0);
}