#!/usr/bin/env bash

home_dir="${HOME}/.taskman"
task_file="${home_dir}/tasks.csv"
archive_file="${home_dir}/archive.csv"
id_sequence_file="${home_dir}/id_sequence.txt"

mkdir -p $home_dir 
touch $task_file
touch $archive_file
touch $id_sequence_file
python3 setup_sequence.py $id_sequence_file
