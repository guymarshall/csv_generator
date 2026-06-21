use std::fs;

use clap::Parser;

use crate::file::file_to_vector;
use crate::functions::{
    add_quotes, generate_initials, generate_period_schedule_csv, generate_room_csv,
    generate_staff_code, generate_student_csv, generate_subject_csv, generate_teacher_csv,
    generate_teacher_type_csv, vector_to_unique_string_with_quotes,
};
use crate::random::{
    day_from_i32, random_length_random_vector, random_name, random_number, random_room,
    random_subject_name,
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
}

// TODO: make structs and headers correct

struct PeriodSchedule {
    id: i32,
    day_of_week: String,
    number_of_periods: i32,
}

struct Student {
    id: String,
    name: String,
    initials: String,
    username: String,
    year_group: i32,
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
    staff_code: String,
    name: String,
    initials: String,
    teacher_type_id: i32,
    subject_taught_ids: String,
}

struct Room {
    id: i32,
    name: String,
    maximum_class_size: i32,
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

    let first_name_list: Vec<String> = file_to_vector("first_names.txt");
    let middle_name_list: Vec<String> = file_to_vector("middle_names.txt");
    let last_name_list: Vec<String> = file_to_vector("last_names.txt");
    let teacher_type_name_list: Vec<String> = file_to_vector("teacher_type_names.txt");
    let teacher_type_count: usize = teacher_type_name_list.len();
    let teacher_type_display_name_list: Vec<String> =
        file_to_vector("teacher_type_display_names.txt");

    assert!(teacher_type_count == teacher_type_display_name_list.len());

    fs::create_dir_all("output").unwrap();

    let period_schedule_data: Vec<PeriodSchedule> = (0..period_schedule_count)
        .map(|i: i32| PeriodSchedule {
            id: i + 1,
            day_of_week: day_from_i32(i % 7),
            number_of_periods: random_number(1, 6),
        })
        .collect();
    generate_period_schedule_csv(
        "output/PeriodSchedule.csv",
        vec!["ID", "DayOfWeek", "NumberOfPeriods"],
        period_schedule_data,
    );

    let student_data: Vec<Student> = (0..student_count)
        .map(|i: i32| {
            let first_name: String = random_name(&first_name_list);
            let middle_name: String = random_name(&middle_name_list);
            let last_name: String = random_name(&last_name_list);

            let first_name_for_initials: String = first_name.clone();
            let middle_name_for_initials: String = middle_name.clone();
            let last_name_for_initials: String = last_name.clone();

            let initials: String = generate_initials(
                first_name_for_initials,
                middle_name_for_initials,
                last_name_for_initials,
            );
            let id: String = (i + 1).to_string();

            Student {
                id: add_quotes(id.clone()),
                name: add_quotes(format! {"{first_name} {middle_name} {last_name}"}),
                initials: add_quotes(initials.clone()),
                username: add_quotes(format!("{initials}{id}")),
                year_group: random_number(7, 13),
            }
        })
        .collect();
    generate_student_csv(
        "output/Student.csv",
        vec!["ID", "Name", "Initials", "Username", "YearGroup"],
        student_data,
    );

    let subject_data: Vec<Subject> = (0..subject_count)
        .map(|i: i32| Subject {
            id: i + 1,
            subject_name: add_quotes(random_subject_name()),
            subject_year: random_number(7, 13),
            set: add_quotes(random_number(1, 8).to_string()),
            maximum_class_size: random_number(15, 31),
            rooms_taught: vector_to_unique_string_with_quotes(&random_length_random_vector()),
        })
        .collect();
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

    let teacher_data: Vec<Teacher> = (0..teacher_count)
        .map(|i: i32| {
            let first_name: String = random_name(&first_name_list);
            let middle_name: String = random_name(&middle_name_list);
            let last_name: String = random_name(&last_name_list);

            let first_name_for_initials: String = first_name.clone();
            let middle_name_for_initials: String = middle_name.clone();
            let last_name_for_initials: String = last_name.clone();

            Teacher {
                id: i + 1,
                staff_code: add_quotes(generate_staff_code(
                    first_name_for_initials.clone(),
                    last_name_for_initials.clone(),
                )),
                name: add_quotes(format!("{first_name} {middle_name} {last_name}")),
                initials: add_quotes(generate_initials(
                    first_name_for_initials,
                    middle_name_for_initials,
                    last_name_for_initials,
                )),
                teacher_type_id: random_number(1, teacher_type_count as i32),
                subject_taught_ids: vector_to_unique_string_with_quotes(
                    &random_length_random_vector(),
                ),
            }
        })
        .collect();
    generate_teacher_csv(
        "output/Teacher.csv",
        vec![
            "ID",
            "StaffCode",
            "Name",
            "Initials",
            "TeacherTypeID",
            "SubjectTaughtIDs",
        ],
        teacher_data,
    );

    let room_data: Vec<Room> = (0..room_count)
        .map(|i: i32| Room {
            id: i + 1,
            name: add_quotes(random_room()),
            maximum_class_size: random_number(15, 31),
        })
        .collect();
    generate_room_csv(
        "output/Room.csv",
        vec!["ID", "Name", "MaximumClassSize"],
        room_data,
    );

    let teacher_type_data: Vec<TeacherType> = (0..teacher_type_count)
        .map(|i: usize| TeacherType {
            id: (i + 1) as i32,
            name: add_quotes(teacher_type_name_list.get(i).unwrap().to_string()),
            display_name: add_quotes(teacher_type_display_name_list.get(i).unwrap().to_string()),
        })
        .collect();
    generate_teacher_type_csv(
        "output/TeacherType.csv",
        vec!["ID", "Name", "DisplayName"],
        teacher_type_data,
    );
}
