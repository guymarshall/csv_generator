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
}

// APP
/*
// <?php

// require_once 'functions.php';

// // add PHP for generating CSV files here isset($_GET) check

// if (isset($_GET['curriculumCount']) && isset($_GET['periodScheduleCount']) && isset($_GET['roomCount']) && isset($_GET['studentCount']) && isset($_GET['subjectCount']) && isset($_GET['teacherCount']) && isset($_GET['teacherTypeCount']))
// {
//     echo 'data has been entered into all input fields';

//     if ($_GET['curriculumCount'] <= 0 || $_GET['periodScheduleCount'] <= 0 || $_GET['roomCount'] <= 0 || $_GET['studentCount'] <= 0 || $_GET['subjectCount'] <= 0 || $_GET['teacherCount'] <= 0 || $_GET['teacherTypeCount'] <= 0)
//     {
//         exit('All counts must be more than 0');
//     }

//     $files = [
//         'curriculum' => ['studentID', 'subjectID', 'numberOfLessonsPerWeek'],
//         'period_schedule' => ['dayOfWeek', 'numberOfPeriods'],
//         'room' => ['name', 'maximumClassSize'],
//         'student' => ['firstName', 'middleNames', 'surname', 'initials'],
//         'subject' => ['subjectName', 'subjectYear', 'set', 'maximumClassSize', 'roomsTaught'],
//         'teacher' => ['firstName', 'middleName', 'surname', 'initials', 'teacherTypeID', 'subjectTaughtIDs', 'roomTaughtIDs'],
//         'teacher_type' => ['name', 'displayName']
//     ];

//     $curriculum_data = [];
//     for ($i = 0; $i < $_GET['curriculumCount']; $i++)
//     {
//         $curriculum_data[] = [
//             (string) $i,
//             random_number(1, $_GET['subjectCount'] + 1),
//             random_number(1, 9)
//         ];
//     }
//     generate_csv('Curriculum.csv', ['studentID', 'subjectID', 'numberOfLessonsPerWeek'], $curriculum_data);

//     $period_schedule_data = [];
//     for ($i = 0; $i < $_GET['periodScheduleCount']; $i++)
//     {
//         $period_schedule_data[] = [
//             random_day(),
//             random_number(1, 6)
//         ];
//     }
//     generate_csv('PeriodSchedule.csv', ['dayOfWeek', 'numberOfPeriods'], $period_schedule_data);

//     $room_data = [];
//     for ($i = 0; $i < $_GET['roomCount']; $i++)
//     {
//         $room_data[] = [
//             random_room(),
//             random_number(15, 31)
//         ];
//     }
//     generate_csv('Room.csv', ['name', 'maximumClassSize'], $room_data);

//     $student_data = [];
//     for ($i = 0; $i < $_GET['studentCount']; $i++)
//     {
//         $first_name = get_random_name('first-names.txt');
//         $middle_name = get_random_name('middle-names.txt');
//         $last_name = get_random_name('middle-names.txt');

//         $student_data[] = [
//             $first_name,
//             $middle_name,
//             $last_name,
//             generate_initials($first_name, $middle_name, $last_name)
//         ];
//     }
//     generate_csv('Student.csv', ['firstName', 'middleNames', 'surname', 'initials'], $student_data);

//     $subject_data = [];
//     for ($i = 0; $i < $_GET['subjectCount']; $i++)
//     {
//         $subject_data[] = [
//             get_random_name('middle-names.txt'),
//             random_number(7, 13),
//             random_number(1, 8),
//             random_number(15, 31),
//             random_number(1, 8)
//         ];
//     }
//     generate_csv('Subject.csv', ['subjectName', 'subjectYear', 'set', 'maximumClassSize', 'roomsTaught'], $subject_data);

//     $teacher_data = [];
//     for ($i = 0; $i < $_GET['teacherCount']; $i++)
//     {
//         $first_name = get_random_name('first-names.txt');
//         $middle_name = get_random_name('middle-names.txt');
//         $last_name = get_random_name('middle-names.txt');

//         $teacher_data[] = [
//             $first_name,
//             $middle_name,
//             $last_name,
//             generate_initials($first_name, $middle_name, $last_name),
//             random_number(0, 100),
//             generate_random_length_random_array(),
//             generate_random_length_random_array()
//         ];
//     }
//     generate_csv('Teacher.csv', ['firstName', 'middleName', 'surname', 'initials', 'teacherTypeID', 'subjectTaughtIDs', 'roomTaughtIDs'], $teacher_data);

//     $teacher_type_data = [];
//     for ($i = 0; $i < $_GET['teacherTypeCount']; $i++)
//     {
//         $teacher_type_data[] = [
//             random_teacher_type('name'),
//             random_teacher_type('displayName')
//         ];
//     }
//     generate_csv('TeacherType.csv', ['name', 'displayName'], $teacher_type_data);
// }
*/