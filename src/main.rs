#![forbid(unsafe_code)]

use std::process::exit;

use crate::file::{get_first_names, get_last_names, get_middle_names};
use crate::functions::generate_initials;
use crate::random::{random_name, random_day, random_number, random_room};

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

    const CSV_FIELDS: &[(&str, &[&str])] = &[
		("curriculum", &["id", "studentID", "subjectID", "numberOfLessonsPerWeek"]),
		("period_schedule", &["id", "dayOfWeek", "numberOfPeriods"]),
		("room", &["id", "name", "maximumClassSize", "subjectsTaught", "teachers"]),
		("student", &["id", "firstName", "middleNames", "surname", "initials"]),
		("subject", &["id", "subjectName", "subjectYear", "set", "maximumClassSize", "teachers", "roomsTaught"]),
		("teacher", &["id", "firstName", "middleNames", "surname", "initials", "teacherType", "subjectsTaught", "roomsTaught"]),
		("teacher_type", &["id", "name", "displayName"]),
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
        let first_name: String = random_name(&first_name_list);
        let middle_name: String = random_name(&middle_name_list);
        let last_name: String = random_name(&last_name_list);

        student_data.push(Student {
            first_name: first_name.clone(),
            middle_name: middle_name.clone(),
            last_name: last_name.clone(),
            initials: generate_initials(&first_name, &middle_name, &last_name)
        })
    }
    //generate_csv('Student.csv', ['firstName', 'middleNames', 'surname', 'initials'], $student_data);

    struct Subject {
        subject_name: String,
        subject_year: i32,
        set: i32,
        maximum_class_size: i32,
        rooms_taught: i32
    }
    let mut subject_data: Vec<Subject> = vec![];
    for i in 0..subject_count {
        subject_data.push(Subject {
            subject_name: random_name(&middle_name_list),
            subject_year: random_number(7, 13),
            set: random_number(1, 8),
            maximum_class_size: random_number(15, 31),
            rooms_taught: random_number(1, 8)
        })
    }
    //generate_csv('Subject.csv', ['subjectName', 'subjectYear', 'set', 'maximumClassSize', 'roomsTaught'], $subject_data);


    //generate_csv('Teacher.csv', ['firstName', 'middleName', 'surname', 'initials', 'teacherTypeID', 'subjectTaughtIDs', 'roomTaughtIDs'], $teacher_data);


    //generate_csv('TeacherType.csv', ['name', 'displayName'], $teacher_type_data);
}

// WARNING - THERE IS A BUG WITH THE GENERATION OF CSV FILES, FOR EXAMPLE ENTERING 1000 STUDENTS CREATES 999 INSTEAD OF 1000 (I FORGOT THE TITLE ROW FOR THE CSV COUNTS)

/*
import random
from functions import generate_csv, generate_initials, generate_random_length_random_list, get_count, get_names, random_name, random_day, random_room, random_teacher_type


csv_fields = {
	"curriculum": [
		"id", # int
		"studentID", # int
		"subjectID", # int
		"numberOfLessonsPerWeek" # int
	],
	"period_schedule": [
		"id", # int
		"dayOfWeek", # ?str?
		"numberOfPeriods" # int
	],
	"room": [
		"id", # int
		"name", # str
		"maximumClassSize", # int
		"subjectsTaught", # ?
		"teachers" # ?
	],
	"student": [
		"id", # int quoted e.g. "1"
		"firstName", # str
		"middleNames", # str
		"surname", # str
		"initials" # str
	],
	"subject": [
		"id", # int
		"subjectName", # str
		"subjectYear", # int
		"set", # int quoted e.g "3"
		"maximumClassSize", # int
		"teachers", # ?
		"roomsTaught" # list[int] quoted e.g. "2, 3, 4, 5"
	],
	"teacher": [
		"id", # int
		"firstName", # str
		"middleNames", # str
		"surname", # str
		"initials", # str
		"teacherType", # int
		"subjectsTaught", # list[int] quoted e.g. "1, 2, 3"
		"roomsTaught" # list[int] quoted e.g. "2, 3, 4, 5"
	],
	"teacher_type": [
		"id", # int
		"name", # str
		"displayName" # str
	],
}

def main():
	print("CSV Generator - Enter counts for the following prompts to generate your .CSV file.")
	curriculum_count = get_count("Curriculum Count: ")
	period_schedule_count = get_count("Period Schedule Count: ")
	room_count = get_count("Room Count: ")
	student_count = get_count("Student Count: ")
	subject_count = get_count("Subject Count: ")
	teacher_count = get_count("Teacher Count: ")
	teacher_type_count = get_count("Teacher Type Count: ")

	first_names = get_names("first_names.txt")
	middle_names = get_names("middle_names.txt")
	last_names = get_names("last_names.txt")

	curriculum_fields = [
		"id", # int
		"studentID", # int
		"subjectID", # int
		"numberOfLessonsPerWeek" # int
	]
	curriculum_data = []
	for i in range(0, curriculum_count):
		curriculum_data.append([
			i + 1,
			random.randint(1, student_count + 1),
			random.randint(1, subject_count + 1),
			random.randint(1, 10)
		])
	generate_csv("Curriculum.csv", ["id", "studentID", "subjectID", "numberOfLessonsPerWeek"], curriculum_data)

	period_schedule_fields = [
		"id", # int
		"dayOfWeek", # ?str?
		"numberOfPeriods" # int
	]
	period_schedule_data = []
	for i in range(0, period_schedule_count):
		period_schedule_data.append([
			i + 1,
			random_day(),
			random.randint(1, 10)
		])
	generate_csv("PeriodSchedule.csv", ["id", "dayOfWeek", "numberOfPeriods"], period_schedule_data)

	room_fields = [
		"id", # int
		"name", # str
		"maximumClassSize", # int
		"subjectsTaught", # ?
		"teachers" # ?
	]
	room_data = []
	for i in range(0, room_count):
		room_data.append([
			i + 1,
			random_room(),
			random.randint(20, 32),
			# subjectsTaught
			# teachers
		])
	generate_csv("Room.csv", ["id", "name", "maximumClassSize", "subjectsTaught", "teachers"], room_data)

	student_fields = [
		"id", # int quoted e.g. "1"
		"firstName", # str
		"middleNames", # str
		"surname", # str
		"initials" # str
	]
	student_data = []
	for i in range(0, student_count):
		first_name = random_name(first_names)
		middle_name = random_name(middle_names)
		last_name = random_name(last_names)
		student_data.append([
			str(i + 1),
			first_name,
			middle_name,
			last_name,
			generate_initials(first_name, middle_name, last_name)
		])
	generate_csv("Student.csv", ["id", "firstName", "middleName", "surname", "initials"], student_data)

	subject_fields = [
		"id", # int
		"subjectName", # str
		"subjectYear", # int
		"set", # int quoted e.g "3"
		"maximumClassSize", # int
		"teachers", # ?
		"roomsTaught" # list[int] quoted e.g. "2, 3, 4, 5"
	]
	subject_data = []
	for i in range(0, subject_count):
		subject_data.append([
			i + 1,
			# subjectName
			random.randint(1, 5),
			str(random.randint(1, 8)),
			random.randint(20, 32),
			# teachers
			# roomsTaught list(int) quoted e.g. "2, 3, 4, 5"
		])
	generate_csv("Subject.csv", ["id", "subjectName", "subjectYear", "set", "maximumClassSize", "teachers", "roomsTaught"], subject_data)

	teacher_fields = [
		"id", # int
		"firstName", # str
		"middleNames", # str
		"surname", # str
		"initials", # str
		"teacherType", # int
		"subjectsTaught", # list[int] quoted e.g. "1, 2, 3"
		"roomsTaught" # list[int] quoted e.g. "2, 3, 4, 5"
	]
	teacher_data = []
	for i in range(0, teacher_count):
		first_name = random_name(first_names)
		middle_name = random_name(middle_names)
		last_name = random_name(last_names)
		teacher_data.append([
			i + 1,
			first_name,
			middle_name,
			last_name,
			generate_initials(first_name, middle_name, last_name),
			random.randint(1, teacher_type_count + 1),
			", ".join(map(str, generate_random_length_random_list())),
			", ".join(map(str, generate_random_length_random_list()))
		])
	generate_csv("Teacher.csv", ["id", "firstName", "middleName", "surname", "initials", "teacherTypeID", "subjectTaughtIDs", "roomTaughtIDs"], teacher_data)

	teacher_type_fields = [
		"id", # int
		"name", # str
		"displayName" # str
	]
	teacher_type_data = []
	for i in range(0, teacher_type_count):
		teacher_type_data.append([
			i + 1,
			random_teacher_type("name"),
			random_teacher_type("displayName")
		])
	generate_csv("TeacherType.csv", ["id", "name", "displayName"], teacher_type_data)
*/