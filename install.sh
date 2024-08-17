#!/usr/bin/env bash

home_dir="${HOME}/.taskman"
task_file="${home_dir}/tasks.csv"
archive_file="${home_dir}/archive.csv"

mkdir -p $home_dir 
touch $task_file
touch $archive_file
