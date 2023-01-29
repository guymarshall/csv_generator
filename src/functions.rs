fn generate_initials(first_name: &str, middle_name: &str, last_name: &str) -> String {
    let mut result: String = String::new();

    if !first_name.is_empty() {
        result.push(first_name.chars().next().unwrap());
    }
    if !middle_name.is_empty() {
        result.push(middle_name.chars().next().unwrap());
    }
    if !last_name.is_empty() {
        result.push(last_name.chars().next().unwrap());
    }

    result
}

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
// function generate_initials(string $first_name = '', string $middle_name = '', string $last_name = '')
// {
//     return $first_name[0].$middle_name[0].$last_name[0];
// }
*/