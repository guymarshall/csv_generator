csv_fields = {
	"teacher": [
		"id",
		"firstName",
		"middleNames",
		"surname",
		"initials",
		"teacherType",
		"subjectsTaught",
		"roomsTaught"
	],
	"teacher_type": [
		"id",
		"name",
		"displayName"
	],
	"subject": [
		"id",
		"subjectName",
		"subjectYear",
		"set",
		"maximumClassSize",
		"teachers",
		"roomsTaught"
	],
	"room": [
		"id",
		"name",
		"maximumClassSize",
		"subjectsTaught",
		"teachers"
	],
	"student": [
		"id",
		"firstName",
		"middleNames",
		"surname",
		"initials"
	],
	"curriculum": [
		"id",
		"studentID",
		"subjectID",
		"numberOfLessonsPerWeek"
	],
	"period_schedule": [
		"id",
		"dayOfWeek",
		"numberOfPeriods"
	]
}

from file import file_to_list

first_names = file_to_list("first-names.txt")

for name in first_names:
	print(name)