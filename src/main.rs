#![forbid(unsafe_code)]

use crate::file::{get_names};
use crate::functions::{generate_initials, generate_curriculum_csv, generate_period_schedule_csv};
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

	let first_name_list: Vec<String> = get_names("first_names.txt");
    let middle_name_list: Vec<String> = get_names("middle_names.txt");
    let last_name_list: Vec<String> = get_names("last_names.txt");

    let mut curriculum_data: Vec<Vec<i32>> = vec![];
    for i in 0..curriculum_count {
		curriculum_data.push(vec![
			i,
			random_number(1, student_count + 1),
			random_number(1, subject_count + 1),
			random_number(1, 9)
		]);
    }
	generate_curriculum_csv("Curriculum.csv", vec!["ID", "StudentID", "SubjectID", "NumberOfLessonsPerWeek"], curriculum_data);

    let mut period_schedule_data: Vec<PeriodSchedule> = vec![];
    for i in 0..period_schedule_count {
        period_schedule_data.push(PeriodSchedule {
            day_of_week: random_day(false),
            number_of_periods: random_number(1, 6)
        })
    }
	generate_period_schedule_csv("PeriodSchedule.csv", vec!["ID", "DayOfWeek", "NumberOfPeriods"], period_schedule_data);

	// room_fields = [
	// 	"id", # int
	// 	"name", # str
	// 	"maximumClassSize", # int
	// 	"subjectsTaught", # ?
	// 	"teachers" # ?
	// ]
	// room_data = []
	// for i in range(0, room_count + 1):
	// 	room_data.append([
	// 		i + 1,
	// 		random_room(),
	// 		random.randint(20, 32),
	// 		# subjectsTaught
	// 		# teachers
	// 	])
	// generate_csv("Room.csv", ["id", "name", "maximumClassSize", "subjectsTaught", "teachers"], room_data)
    let mut room_data: Vec<Room> = vec![];
    for i in 0..room_count {
        room_data.push(Room {
            name: random_room().parse().unwrap(),
            maximum_class_size: random_number(15, 31)
        })
    }
    //generate_csv('Room.csv', ['name', 'maximumClassSize'], $room_data);

	// student_fields = [
	// 	"id", # int quoted e.g. "1"
	// 	"firstName", # str
	// 	"middleNames", # str
	// 	"surname", # str
	// 	"initials" # str
	// ]
	// student_data = []
	// for i in range(0, student_count + 1):
	// 	first_name = random_name(first_names)
	// 	middle_name = random_name(middle_names)
	// 	last_name = random_name(last_names)
	// 	student_data.append([
	// 		str(i + 1),
	// 		first_name,
	// 		middle_name,
	// 		last_name,
	// 		generate_initials(first_name, middle_name, last_name)
	// 	])
	// generate_csv("Student.csv", ["id", "firstName", "middleName", "surname", "initials"], student_data)
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

	// subject_fields = [
	// 	"id", # int
	// 	"subjectName", # str
	// 	"subjectYear", # int
	// 	"set", # int quoted e.g "3"
	// 	"maximumClassSize", # int
	// 	"teachers", # ?
	// 	"roomsTaught" # list[int] quoted e.g. "2, 3, 4, 5"
	// ]
	// subject_data = []
	// for i in range(0, subject_count + 1):
	// 	subject_data.append([
	// 		i + 1,
	// 		# subjectName
	// 		random.randint(1, 5),
	// 		str(random.randint(1, 8)),
	// 		random.randint(20, 32),
	// 		# teachers
	// 		# roomsTaught list(int) quoted e.g. "2, 3, 4, 5"
	// 	])
	// generate_csv("Subject.csv", ["id", "subjectName", "subjectYear", "set", "maximumClassSize", "teachers", "roomsTaught"], subject_data)
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

	// teacher_fields = [
	// 	"id", # int
	// 	"firstName", # str
	// 	"middleNames", # str
	// 	"surname", # str
	// 	"initials", # str
	// 	"teacherType", # int
	// 	"subjectsTaught", # list[int] quoted e.g. "1, 2, 3"
	// 	"roomsTaught" # list[int] quoted e.g. "2, 3, 4, 5"
	// ]
	// teacher_data = []
	// for i in range(0, teacher_count + 1):
	// 	first_name = random_name(first_names)
	// 	middle_name = random_name(middle_names)
	// 	last_name = random_name(last_names)
	// 	teacher_data.append([
	// 		i + 1,
	// 		first_name,
	// 		middle_name,
	// 		last_name,
	// 		generate_initials(first_name, middle_name, last_name),
	// 		random.randint(1, teacher_type_count + 1),
	// 		", ".join(map(str, generate_random_length_random_list())),
	// 		", ".join(map(str, generate_random_length_random_list()))
	// 	])
	// generate_csv("Teacher.csv", ["id", "firstName", "middleName", "surname", "initials", "teacherTypeID", "subjectTaughtIDs", "roomTaughtIDs"], teacher_data)
    //generate_csv('Teacher.csv', ['firstName', 'middleName', 'surname', 'initials', 'teacherTypeID', 'subjectTaughtIDs', 'roomTaughtIDs'], $teacher_data);

	// teacher_type_fields = [
	// 	"id", # int
	// 	"name", # str
	// 	"displayName" # str
	// ]
	// teacher_type_data = []
	// for i in range(0, teacher_type_count + 1):
	// 	teacher_type_data.append([
	// 		i + 1,
	// 		random_teacher_type("name"),
	// 		random_teacher_type("displayName")
	// 	])
	// generate_csv("TeacherType.csv", ["id", "name", "displayName"], teacher_type_data)
    //generate_csv('TeacherType.csv', ['name', 'displayName'], $teacher_type_data);
}