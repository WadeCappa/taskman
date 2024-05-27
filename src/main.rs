use clap::{arg, Arg, ArgAction, ArgMatches, Command};
use std::string::String;
use std::fmt::Debug;
use std::option::Option;
use chrono::{DateTime, FixedOffset, Local};
use std::fs::{OpenOptions, File};
use std::io::prelude::*;
use dirs::home_dir;

const DATETIME_FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.f";
const APP_DATA: &str = ".taskman/tasks.csv";

#[derive(Debug)]
enum Completed {
    YES,
    NO
}

impl std::fmt::Display for Completed {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let value: char = match self {
            Completed::YES => 'y',
            Completed::NO => 'n'
        };
        write!(f, "{}", value)
    }
}

#[derive(Debug)]
struct Task {
    name: String,
    cost: u32,
    priority: u32,
    completed: Completed,
    description: Option<String>,
    date_created: DateTime::<FixedOffset>,
    deadline: Option<DateTime::<FixedOffset>>,
}

impl Task {
    fn as_string(&self) -> String {
        return format!("{0},{1},{2},{3},{4},{5},{6}", 
            self.completed,
            self.name, 
            match &self.description {
                Some(desc) => desc, 
                None => "" 
            },
            self.cost, 
            self.priority, 
            self.date_created.to_rfc3339(), 
            match self.deadline {
                Some(date) => date.to_rfc3339(),
                None => String::from("")
            }
        );
    }

    fn write(&self, file: &mut File) {
        match writeln!(file, "{0}", self.as_string()) { 
            Ok(_value) => println!("added new task {:?}", self),
            Err(error) => eprintln!("failed to write new task, {error}")
        };
    }
}

fn write_task(task: Task) {
    match home_dir() {
        Some(mut path) => {
            path.push(APP_DATA);
            match OpenOptions::new().write(true).append(true).open(path.as_path()) {
                Ok(mut file) => {
                    task.write(&mut file);
                }
                Err(error) => {
                    let path_as_str = path.as_path().to_str().unwrap();
                    eprintln!("file write error, {error}, {path_as_str}");
                }
            }
        },
        None => panic!("cannot find home directory"),
    };
}

fn get_string_arg(args: &ArgMatches, arg_name: &str) -> String {
    return String::from(args.get_one::<String>(arg_name).unwrap());
}

fn get_u32_arg(args: &ArgMatches, arg_name: &str) -> u32 { 
    return *args.get_one::<u32>(arg_name).unwrap(); 
}

fn append_midnight_if_no_time(date_maybe_time: String) -> String {
    if !date_maybe_time.contains("T") { 
        return date_maybe_time + "T23:59:59";
    } else {
        return date_maybe_time;
    }
}

fn append_timezone_if_not_present(date_maybe_timezone: String) -> String {
    if !date_maybe_timezone.contains("+") { 
        let datetime: String = DateTime::<FixedOffset>::from(Local::now()).to_rfc3339();
        let current_time_offset: &str = &datetime[datetime.len() - 6..];
        return format!("{date_maybe_timezone}{current_time_offset}");
    } else {
        return date_maybe_timezone;
    }

}

fn get_datetime(args: &ArgMatches, arg_name: &str) -> DateTime::<FixedOffset> {
    let datetime = append_timezone_if_not_present(
        append_midnight_if_no_time(
            get_string_arg(args, arg_name)
        )
    );

    match DateTime::parse_from_rfc3339(&datetime) {
        Ok(value) => return value,
        Err(error) => {
            eprintln!("encountered an error trying to parse {datetime}, {error}");
            panic!("could not unwrap user timestamp");
        }
    }; 
}

fn get_optional<T>(
    args: &ArgMatches, 
    arg_name: &str, 
    arg_getter: &dyn Fn(&ArgMatches, &str) -> T 
) -> Option<T> {
    return match args.contains_id(arg_name) {
        true => Some(arg_getter(args, arg_name)),
        false => None
    }
}

fn build_task(args: &ArgMatches) -> Task {
    let name = get_string_arg(args, "name");
    let cost = get_u32_arg(args, "cost");
    let priority = get_u32_arg(args, "priority");
    let deadline = get_optional::<DateTime::<FixedOffset>>(
        args,
        "deadline",
        &get_datetime
    );

    let description = get_optional::<String>(args, "description", &get_string_arg);

    return Task {
        name: name,
        cost: cost,
        description: description,
        completed: Completed::NO,
        priority: priority,
        date_created: DateTime::from(Local::now()),
        deadline: deadline 
    };
}

fn add(args: &ArgMatches) {
    let task: Task = build_task(args);
    write_task(task);
}

fn delete(args: &ArgMatches) {
}

fn complete(args: &ArgMatches) {
}

fn show(args: &ArgMatches) {
}

fn main() {
    let cmd = clap::Command::new("cmd")
        .subcommand_required(true)
        .subcommand( Command::new("add")
            .about("add a new task")
            .arg(arg!(-n --name "name of the new task")
                .required(true)
                .action(ArgAction::Set))
            .arg(arg!(-e --description "description of this task")
                .action(ArgAction::Set))
            .arg(arg!(-c --cost "the expected time in minutes that this task will take")
                .action(ArgAction::Set)
                .required(true)
                .value_parser(clap::value_parser!(u32)))
            .arg(arg!(-p --priority 
                    "an arbitrary number representing the importance of completing this task")
                .action(ArgAction::Set)
                .required(true)
                .value_parser(clap::value_parser!(u32)))
            .arg(arg!(-d --deadline
                    "when this task needs to be completed, in format 
                    yyyy-dd-mmThh::mm::ss in military time (always assumes local timezone)"
                )
                .action(ArgAction::Set))
            )
        .subcommand(Command::new("delete").about(""))
        .subcommand(Command::new("complete").about(""))
        .subcommand(Command::new("show").about("display tasks")
            .arg(arg!(-n --number "number of tasks to show")
                .default_value("5")
                .value_parser(clap::value_parser!(u32))
                .action(ArgAction::Set))
            .arg(arg!(-a --all "show all tasks")
                .value_parser(clap::value_parser!(u32))
                .action(ArgAction::SetTrue))
            );

    let matches = cmd.get_matches();
    match matches.subcommand() {
        Some(("add", matches)) => add(matches), 
        Some(("delete", matches)) => delete(matches), 
        Some(("complete", matches)) => complete(matches), 
        Some(("show", matches)) => show(matches), 
        _ => unreachable!("clap should ensure that we don't get here"),
    };
}
