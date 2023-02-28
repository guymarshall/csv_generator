#![forbid(unsafe_code)]

use crate::file::{
	get_names
};
use crate::functions::{
	generate_initials,
	generate_curriculum_csv,
	generate_period_schedule_csv,
	generate_room_csv,
	add_quotes,
	generate_student_csv,
	generate_teacher_type_csv,
	vector_to_string_with_quotes,
	generate_subject_csv,
	generate_teacher_csv
};
use crate::random::{
	random_name,
	day_from_i32,
	random_number,
	random_room,
	random_teacher_type,
	random_length_random_vector,
	random_subject_name
};

mod file;
mod random;
mod functions;
mod user_input;

fn main() {
    println!("CSV Generator - Enter counts for the following prompts to generate your .CSV file.");
    let curriculum_count: i32 = user_input::input("Curriculum Count:");
    let period_schedule_count: i32 = user_input::input("Period Schedule Count:");
    let room_count: i32 = user_input::input("Room Count:");
    let student_count: i32 = user_input::input("Student Count:");
    let subject_count: i32 = user_input::input("Subject Count:");
    let teacher_count: i32 = user_input::input("Teacher Count:");
    let teacher_type_count: i32 = user_input::input("Teacher Type Count:");

	let first_name_list: Vec<String> = get_names("first_names.txt");
    let middle_name_list: Vec<String> = get_names("middle_names.txt");
    let last_name_list: Vec<String> = get_names("last_names.txt");

    let mut curriculum_data: Vec<Vec<i32>> = vec![];
    for i in 0..curriculum_count {
		curriculum_data.push(vec![
			i + 1,
			random_number(1, student_count + 1),
			random_number(1, subject_count + 1),
			random_number(1, 9)
		]);
    }
	generate_curriculum_csv(
		"Curriculum.csv",
		vec!["ID", "StudentID", "SubjectID", "NumberOfLessonsPerWeek"],
		curriculum_data
	);

	let mut period_schedule_data: Vec<Vec<(i32, String, i32)>> = vec![];
    for i in 0..period_schedule_count {
        period_schedule_data.push(vec![(
			i + 1,
            day_from_i32(i),
            random_number(1, 6)
		)])
    }
	generate_period_schedule_csv(
		"PeriodSchedule.csv",
		vec!["ID", "DayOfWeek", "NumberOfPeriods"],
		period_schedule_data
	);

	let mut room_data: Vec<Vec<(i32, &str, i32, String, String)>> = vec![];
    for i in 0..room_count {
		room_data.push(vec![(
			i + 1,
			random_room(),
            random_number(15, 31),
			vector_to_string_with_quotes(&random_length_random_vector()),
			vector_to_string_with_quotes(&random_length_random_vector())
		)]);
    }
	generate_room_csv(
		"Room.csv",
		vec!["ID", "Name", "MaximumClassSize", "SubjectsTaught", "Teachers"],
		room_data
	);

	let mut student_data: Vec<Vec<(String, String, String, String, String)>> = vec![];
	for i in 0..student_count {
		let first_name: String = random_name(&first_name_list);
		let middle_name: String = random_name(&middle_name_list);
		let last_name: String = random_name(&last_name_list);

		let first_name_for_initials: String = first_name.clone();
		let middle_name_for_initials: String = middle_name.clone();
		let last_name_for_initials: String = last_name.clone();

		student_data.push(vec![(
			add_quotes(&(i + 1).to_string()),
			first_name,
			middle_name,
			last_name,
			generate_initials(first_name_for_initials, middle_name_for_initials, last_name_for_initials)
		)]);
	}

    generate_student_csv(
		"Student.csv",
		vec!["ID", "FirstName", "MiddleNames", "Surname", "Initials"],
		student_data
	);

	let mut subject_data: Vec<Vec<(i32, String, i32, String, i32, String, String)>> = vec![];
    for i in 0..subject_count {
        subject_data.push(vec![(
            i + 1,
			random_subject_name(),
            random_number(7, 13),
            add_quotes(random_number(1, 8).to_string().as_str()),
            random_number(15, 31),
			vector_to_string_with_quotes(&random_length_random_vector()),
            vector_to_string_with_quotes(&random_length_random_vector())
		)]);
    }
    generate_subject_csv(
		"Subject.csv",
		vec!["ID", "SubjectName", "SubjectYear", "Set", "MaximumClassSize", "Teachers", "RoomsTaught"],
		subject_data
	);

	let mut teacher_data: Vec<Vec<(i32, String, String, String, String, i32, String, String)>> = vec![];
	for i in 0..teacher_count {
		let first_name: String = random_name(&first_name_list);
		let middle_name: String = random_name(&middle_name_list);
		let last_name: String = random_name(&last_name_list);

		let first_name_for_initials: String = first_name.clone();
		let middle_name_for_initials: String = middle_name.clone();
		let last_name_for_initials: String = last_name.clone();

		teacher_data.push(vec![(
			i + 1,
			first_name,
			middle_name,
			last_name,
			generate_initials(first_name_for_initials, middle_name_for_initials, last_name_for_initials),
			random_number(1, teacher_type_count + 1),
			vector_to_string_with_quotes(&random_length_random_vector()),
			vector_to_string_with_quotes(&random_length_random_vector())
		)]);
	}
	generate_teacher_csv(
		"Teacher.csv",
		vec!["ID", "FirstName", "MiddleName", "Surname", "Initials", "TeacherTypeID", "SubjectTaughtIDs", "RoomTaughtIDs"],
		teacher_data
	);

	let mut teacher_type_data: Vec<Vec<(i32, String, String, String, String)>> = vec![];
    for i in 0..teacher_type_count {
		teacher_type_data.push(vec![(
			i + 1,
			random_teacher_type("name"),
            random_teacher_type("displayName"),
			vector_to_string_with_quotes(&random_length_random_vector()),
			vector_to_string_with_quotes(&random_length_random_vector())
		)]);
    }
	generate_teacher_type_csv(
		"TeacherType.csv",
		vec!["ID", "Name", "DisplayName", "SubjectsTaught", "Teachers"],
		teacher_type_data
	);
}