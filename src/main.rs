fn main() {
    println!("Hello, world!");
}

// FUNCTIONS
/*
// <?php
//
// function get_random_name(string $filename)
// {
//     $contents = file_get_contents($filename);
//     $first_names = explode("\n", $contents);
//
//     $key = array_rand($first_names);
//     $first_name = $first_names[$key];
//     return $first_name;
// }
//
// function random_number(int $min, int $max, bool $return_string = false)
// {
//     $random_number = mt_rand($min, $max);
//
//     return $return_string ? (string) $random_number : $random_number;
// }
//
// function generate_csv(string $filename, array $field_headings, array $data)
// {
//     $file = fopen($filename, 'w');
//
//     fputcsv($file, $field_headings);
//
//     foreach ($data as $row) {
//         fputcsv($file, $row);
//     }
//
//     fclose($file);
// }
//
// function random_day(bool $include_weekend = false)
// {
//     $days_of_week = ['Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday'];
//
//     if ($include_weekend)
//     {
//         $days_of_week[] = 'Saturday';
//         $days_of_week[] = 'Sunday';
//     }
//
//     $max_random_index = $include_weekend ? 4 : 6;
//
//     return $days_of_week[mt_rand(0, $max_random_index)];
// }
//
// function random_room()
// {
//     $rooms = ['Ma1', 'Ma2', 'Ma3', 'Ma4', 'Ma5', 'Ma6', 'Ma7', 'Ma8', 'Ma9', 'DT1', 'DT2', 'DT3', 'DT4', 'DT5', 'IT1', 'IT2', 'IT3', 'La1', 'La2', 'La3', 'La4', 'La5', 'History1', 'History2', 'History3', 'Geography1', 'Geography2', 'Geography3', 'Sc1', 'Sc2', 'Sc3', 'Sc4', 'Sc5', 'Sc6', 'Sc7', 'Sc8', 'Eng1', 'Eng2', 'Eng3', 'Eng4', 'Eng5', 'Eng6', 'Eng7', 'Eng8', 'Music1', 'Music2', 'Drama1', 'Drama2', 'PE'];
//
//     return $rooms[mt_rand(0, count($rooms))];
// }
//
// function generate_initials(string $first_name = '', string $middle_name = '', string $last_name = '')
// {
//     return $first_name[0].$middle_name[0].$last_name[0];
// }
//
// function generate_random_length_random_array()
// {
//     $output = [];
//
//     for ($i = 0; $i < mt_rand(1, 10); $i++)
//     {
//         $output[] = mt_rand(1, 10);
//     }
//
//     return $output;
// }
//
// function random_teacher_type(string $type)
// {
//     $names = ['Teacher', 'Cover Teacher', 'Trainee Teacher', 'Head of Department'];
//     $display_names = ['Teacher', 'Cover', 'Trainee', 'Head'];
//
//     if ($type == 'name')
//     {
//         return $names[mt_rand(0, count($names))];
//     }
//     else if ($type == 'displayName')
//     {
//         return $display_names[mt_rand(0, count($display_names))];
//     }
// }
*/

// APP
/*
// <!DOCTYPE html>
// <html lang="en">
// <head>
//     <meta charset="UTF-8">
//     <meta http-equiv="X-UA-Compatible" content="IE=edge">
//     <meta name="viewport" content="width=device-width, initial-scale=1.0">
//     <title>CSV Generator</title>
// </head>
// <body>
//     <form action="index.php" method="get">
//         <label for="curriculumCount">Curriculum Count:</label>
//         <input type="number" name="curriculumCount" id="curriculumCount">
//         <br>
//         <label for="periodScheduleCount">PeriodSchedule Count:</label>
//         <input type="number" name="periodScheduleCount" id="periodScheduleCount">
//         <br>
//         <label for="roomCount">Room Count:</label>
//         <input type="number" name="roomCount" id="roomCount">
//         <br>
//         <label for="studentCount">Student Count:</label>
//         <input type="number" name="studentCount" id="studentCount">
//         <br>
//         <label for="subjectCount">Subject Count:</label>
//         <input type="number" name="subjectCount" id="subjectCount">
//         <br>
//         <label for="teacherCount">Teacher Count:</label>
//         <input type="number" name="teacherCount" id="teacherCount">
//         <br>
//         <label for="teacherTypeCount">Teacher Type Count:</label>
//         <input type="number" name="teacherTypeCount" id="teacherTypeCount">
//         <br>
//         <input type="submit" value="Generate">
//     </form>
// </body>
// </html>

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