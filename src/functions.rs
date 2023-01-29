/*
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