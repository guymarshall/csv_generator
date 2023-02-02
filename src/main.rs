use crate::file::{get_first_names, get_last_names, get_middle_names};
use crate::functions::generate_initials;
use crate::random::{get_random_name, random_day, random_number, random_room};

mod file;
mod random;
mod functions;
mod user_input;

fn main() {
    println!("CSV Generator - Enter counts for the following prompts to generate your .CSV file.");
    let curriculum_count: i32 = user_input::get_user_input("Curriculum Count:");
    let period_schedule_count: i32 = user_input::get_user_input("Period Schedule Count:");
    let room_count: i32 = user_input::get_user_input("Room Count:");
    let student_count: i32 = user_input::get_user_input("Student Count:");
    let subject_count: i32 = user_input::get_user_input("Subject Count:");
    let teacher_count: i32 = user_input::get_user_input("Teacher Count:");
    let teacher_type_count: i32 = user_input::get_user_input("Teacher Type Count:");

    if curriculum_count < 1
        || period_schedule_count < 1
        || room_count < 1
        || student_count < 1
        || subject_count < 1
        || teacher_count < 1
        || teacher_type_count < 1 {
        panic!("All counts must be more than 0");
    }

    const FILES: &[(&str, &[&str])] = &[
        ("curriculum", &["studentID", "subjectID", "numberOfLessonsPerWeek"]),
        ("period_schedule", &["dayOfWeek", "numberOfPeriods"]),
        ("room", &["name", "maximumClassSize"]),
        ("student", &["firstName", "middleNames", "surname", "initials"]),
        ("subject", &["subjectName", "subjectYear", "set", "maximumClassSize", "roomsTaught"]),
        ("teacher", &["firstName", "middleName", "surname", "initials", "teacherTypeID", "subjectTaughtIDs", "roomTaughtIDs"]),
        ("teacher_type", &["name", "displayName"]),
    ];

    struct Curriculum {
        index: String,
        subject_id: i32,
        number_of_lessons_per_week: i32
    }
    let mut curriculum_data: Vec<Curriculum> = vec![];
    for i in 0..curriculum_count {
        curriculum_data.push(Curriculum {
            index: i.to_string(),
            subject_id: random_number(1, subject_count + 1).to_string().trim().parse().unwrap_or(0),
            number_of_lessons_per_week: random_number(1, 9).to_string().trim().parse().unwrap_or(0)
        });
    }
    //generate_csv('Curriculum.csv', ['studentID', 'subjectID', 'numberOfLessonsPerWeek'], $curriculum_data);

    struct PeriodSchedule {
        day_of_week: String,
        number_of_periods: i32
    }
    let mut period_schedule_data: Vec<PeriodSchedule> = vec![];
    for i in 0..period_schedule_count {
        period_schedule_data.push(PeriodSchedule {
            day_of_week: random_day(false),
            number_of_periods: random_number(1, 6)
        })
    }
    //generate_csv('PeriodSchedule.csv', ['dayOfWeek', 'numberOfPeriods'], $period_schedule_data);

    struct Room {
        name: String,
        maximum_class_size: i32
    }
    let mut room_data: Vec<Room> = vec![];
    for i in 0..room_count {
        room_data.push(Room {
            name: random_room().parse().unwrap(),
            maximum_class_size: random_number(15, 31)
        })
    }
    //generate_csv('Room.csv', ['name', 'maximumClassSize'], $room_data);

    struct Student {
        first_name: String,
        middle_name: String,
        last_name: String,
        initials: String
    }
    let first_name_list: Vec<String> = get_first_names();
    let middle_name_list: Vec<String> = get_middle_names();
    let last_name_list: Vec<String> = get_last_names();

    let mut student_data: Vec<Student> = vec![];
    for i in 0..student_count {
        let first_name: String = get_random_name(&first_name_list);
        let middle_name: String = get_random_name(&middle_name_list);
        let last_name: String = get_random_name(&last_name_list);

        student_data.push(Student {
            first_name: first_name.clone(),
            middle_name: middle_name.clone(),
            last_name: last_name.clone(),
            initials: generate_initials(&first_name, &middle_name, &last_name)
        })
    }
    //generate_csv('Student.csv', ['firstName', 'middleNames', 'surname', 'initials'], $student_data);


    //generate_csv('Subject.csv', ['subjectName', 'subjectYear', 'set', 'maximumClassSize', 'roomsTaught'], $subject_data);


    //generate_csv('Teacher.csv', ['firstName', 'middleName', 'surname', 'initials', 'teacherTypeID', 'subjectTaughtIDs', 'roomTaughtIDs'], $teacher_data);


    //generate_csv('TeacherType.csv', ['name', 'displayName'], $teacher_type_data);
}


// APP
/*
// $subject_data = [];
// for ($i = 0; $i < $_GET['subjectCount']; $i++)
// {
//     $subject_data[] = [
//         get_random_name('middle-names.txt'),
//         random_number(7, 13),
//         random_number(1, 8),
//         random_number(15, 31),
//         random_number(1, 8)
//     ];
// }
// generate_csv('Subject.csv', ['subjectName', 'subjectYear', 'set', 'maximumClassSize', 'roomsTaught'], $subject_data);
//
// $teacher_data = [];
// for ($i = 0; $i < $_GET['teacherCount']; $i++)
// {
//     $first_name = get_random_name('first-names.txt');
//     $middle_name = get_random_name('middle-names.txt');
//     $last_name = get_random_name('middle-names.txt');
//
//     $teacher_data[] = [
//         $first_name,
//         $middle_name,
//         $last_name,
//         generate_initials($first_name, $middle_name, $last_name),
//         random_number(0, 100),
//         generate_random_length_random_array(),
//         generate_random_length_random_array()
//     ];
// }
// generate_csv('Teacher.csv', ['firstName', 'middleName', 'surname', 'initials', 'teacherTypeID', 'subjectTaughtIDs', 'roomTaughtIDs'], $teacher_data);
//
// $teacher_type_data = [];
// for ($i = 0; $i < $_GET['teacherTypeCount']; $i++)
// {
//     $teacher_type_data[] = [
//         random_teacher_type('name'),
//         random_teacher_type('displayName')
//     ];
// }
// generate_csv('TeacherType.csv', ['name', 'displayName'], $teacher_type_data);
*/