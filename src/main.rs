use std::fs;

use clap::Parser;

use crate::file::file_to_vector;
use crate::functions::{
    add_quotes, generate_initials, generate_period_schedule_csv, generate_room_csv,
    generate_student_csv, generate_subject_csv, generate_teacher_csv, generate_teacher_type_csv,
    vector_to_unique_string_with_quotes,
};
use crate::random::{
    day_from_i32, random_length_random_vector, random_name, random_number, random_room,
    random_subject_name, random_teacher_type,
};

mod file;
mod functions;
mod random;

#[derive(Parser)]
#[command(rename_all = "snake_case")]
struct Cli {
    #[arg(long)]
    period_schedule_count: i32,

    #[arg(long)]
    room_count: i32,

    #[arg(long)]
    student_count: i32,

    #[arg(long)]
    subject_count: i32,

    #[arg(long)]
    teacher_count: i32,

    #[arg(long)]
    teacher_type_count: i32,
}

// TODO: make structs and headers correct

struct PeriodSchedule {
    id: i32,
    day_of_week: String,
    number_of_periods: i32,
}

struct Room {
    id: i32,
    name: String,
    maximum_class_size: i32,
}

struct Student {
    id: String,
    first_name: String,
    middle_names: String,
    surname: String,
    initials: String,
}

struct Subject {
    id: i32,
    subject_name: String,
    subject_year: i32,
    set: String,
    maximum_class_size: i32,
    rooms_taught: String,
}

struct Teacher {
    id: i32,
    first_name: String,
    middle_name: String,
    surname: String,
    initials: String,
    teacher_type_id: i32,
    subject_taught_ids: String,
    room_taught_ids: String,
}

struct TeacherType {
    id: i32,
    name: String,
    display_name: String,
}

fn main() {
    let arguments: Cli = Cli::parse();

    let period_schedule_count: i32 = arguments.period_schedule_count;
    let room_count: i32 = arguments.room_count;
    let student_count: i32 = arguments.student_count;
    let subject_count: i32 = arguments.subject_count;
    let teacher_count: i32 = arguments.teacher_count;
    let teacher_type_count: i32 = arguments.teacher_type_count;

    let first_name_list: Vec<String> = file_to_vector("first_names.txt");
    let middle_name_list: Vec<String> = file_to_vector("middle_names.txt");
    let last_name_list: Vec<String> = file_to_vector("last_names.txt");

    fs::create_dir_all("output").unwrap();

    let mut period_schedule_data: Vec<PeriodSchedule> = vec![];
    (0..period_schedule_count).for_each(|i: i32| {
        period_schedule_data.push(PeriodSchedule {
            id: i + 1,
            day_of_week: day_from_i32(i % 7),
            number_of_periods: random_number(1, 6),
        })
    });
    generate_period_schedule_csv(
        "output/PeriodSchedule.csv",
        vec!["ID", "DayOfWeek", "NumberOfPeriods"],
        period_schedule_data,
    );

    let mut room_data: Vec<Room> = vec![];
    (0..room_count).for_each(|i: i32| {
        room_data.push(Room {
            id: i + 1,
            name: add_quotes(random_room()),
            maximum_class_size: random_number(15, 31),
        });
    });
    generate_room_csv(
        "output/Room.csv",
        vec!["ID", "Name", "MaximumClassSize"],
        room_data,
    );

    let mut student_data: Vec<Student> = vec![];
    (0..student_count).for_each(|i: i32| {
        let first_name: String = random_name(&first_name_list);
        let middle_name: String = random_name(&middle_name_list);
        let last_name: String = random_name(&last_name_list);

        let first_name_for_initials: String = first_name.clone();
        let middle_name_for_initials: String = middle_name.clone();
        let last_name_for_initials: String = last_name.clone();

        student_data.push(Student {
            id: add_quotes((i + 1).to_string()),
            first_name: add_quotes(first_name),
            middle_names: add_quotes(middle_name),
            surname: add_quotes(last_name),
            initials: add_quotes(generate_initials(
                first_name_for_initials,
                middle_name_for_initials,
                last_name_for_initials,
            )),
        });
    });

    generate_student_csv(
        "output/Student.csv",
        vec!["ID", "FirstName", "MiddleNames", "Surname", "Initials"],
        student_data,
    );

    let mut subject_data: Vec<Subject> = vec![];
    (0..subject_count).for_each(|i: i32| {
        subject_data.push(Subject {
            id: i + 1,
            subject_name: add_quotes(random_subject_name()),
            subject_year: random_number(7, 13),
            set: add_quotes(random_number(1, 8).to_string()),
            maximum_class_size: random_number(15, 31),
            rooms_taught: vector_to_unique_string_with_quotes(&random_length_random_vector()),
        });
    });
    generate_subject_csv(
        "output/Subject.csv",
        vec![
            "ID",
            "SubjectName",
            "SubjectYear",
            "Set",
            "MaximumClassSize",
            "RoomsTaught",
        ],
        subject_data,
    );

    let mut teacher_data: Vec<Teacher> = vec![];
    (0..teacher_count).for_each(|i: i32| {
        let first_name: String = random_name(&first_name_list);
        let middle_name: String = random_name(&middle_name_list);
        let last_name: String = random_name(&last_name_list);

        let first_name_for_initials: String = first_name.clone();
        let middle_name_for_initials: String = middle_name.clone();
        let last_name_for_initials: String = last_name.clone();

        teacher_data.push(Teacher {
            id: i + 1,
            first_name: add_quotes(first_name),
            middle_name: add_quotes(middle_name),
            surname: add_quotes(last_name),
            initials: add_quotes(generate_initials(
                first_name_for_initials,
                middle_name_for_initials,
                last_name_for_initials,
            )),
            teacher_type_id: random_number(1, teacher_type_count),
            subject_taught_ids: vector_to_unique_string_with_quotes(&random_length_random_vector()),
            room_taught_ids: vector_to_unique_string_with_quotes(&random_length_random_vector()),
        });
    });
    generate_teacher_csv(
        "output/Teacher.csv",
        vec![
            "ID",
            "FirstName",
            "MiddleName",
            "Surname",
            "Initials",
            "TeacherTypeID",
            "SubjectTaughtIDs",
            "RoomTaughtIDs",
        ],
        teacher_data,
    );

    let mut teacher_type_data: Vec<TeacherType> = vec![];
    (0..teacher_type_count).for_each(|i: i32| {
        teacher_type_data.push(TeacherType {
            id: i + 1,
            name: add_quotes(random_teacher_type("name")),
            display_name: add_quotes(random_teacher_type("displayName")),
        });
    });
    generate_teacher_type_csv(
        "output/TeacherType.csv",
        vec!["ID", "Name", "DisplayName"],
        teacher_type_data,
    );
}
