#!/usr/bin/env bash

curriculum_count=1000
period_schedule_count=60
room_count=50
student_count=1000
subject_count=20
teacher_count=70
teacher_type_count=5
cargo run --release -- \
    --curriculum_count "$curriculum_count" \
    --period_schedule_count "$period_schedule_count" \
    --room_count "$room_count" \
    --student_count "$student_count" \
    --subject_count "$subject_count" \
    --teacher_count "$teacher_count" \
    --teacher_type_count "$teacher_type_count"