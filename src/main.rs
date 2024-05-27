use clap::{arg, Arg, ArgAction, ArgMatches, Command};
use std::string::String;
use std::fmt::Debug;
use std::option::Option;
use chrono::{DateTime, FixedOffset, Local};
use std::fs::{OpenOptions, File};
use std::io::prelude::*;
use std::path::PathBuf;
use dirs::home_dir;

const DATETIME_FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.f";
const APP_DATA: &str = ".taskman/tasks.csv";

#[derive(Debug)]
struct Task {
    name: String,
    cost: u32,
    priority: u32,
    date_created: DateTime::<FixedOffset>,
    deadline: Option<DateTime::<FixedOffset>>,
}

impl Task {
    fn as_string(&self) -> String {
        return format!("{0},{1},{2},{3},{4}", 
            self.name, 
            self.cost, 
            self.priority, 
            self.date_created, 
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

fn get_string_arg(args: &ArgMatches, arg_name: &str) -> String {
    return match args.get_one::<String>(arg_name) {
        Some(value) => String::from(value),
        None => panic!("expected to find a value")
    };
}

fn get_u32_arg(args: &ArgMatches, arg_name: &str) -> u32 { 
    return match args.get_one::<u32>(arg_name) {
        Some(value) => *value,
        None => panic!("expected to find a value")
    };
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

    return Task {
        name: name,
        cost: cost,
        priority: priority,
        date_created: DateTime::from(Local::now()),
        deadline: deadline 
    };
}

fn add(args: &ArgMatches) {
    let task: Task = build_task(args);
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
            .about("")
            .arg(Arg::new("name")
                .short('n')
                .long("name")
                .action(ArgAction::Set)
                .value_name("name")
                .required(true)
                .help("name of the new task")
                .value_parser(clap::builder::NonEmptyStringValueParser::new()))
            .arg(arg!(-c --cost "the expected time in minutes that this task will take")
                .required(true)
                .value_parser(clap::value_parser!(u32))
                .action(ArgAction::Set))
            .arg(arg!(-p --priority 
                    "an arbitrary number representing the importance of completing this task"
                )
                .required(true)
                .value_parser(clap::value_parser!(u32))
                .action(ArgAction::Set))
            .arg(arg!(-d --deadline
                    "when this task needs to be completed, in format 
                    yyyy-dd-mmThh::mm::ss in military time (always assumes local timezone)"
                )
                .required(false)
                .value_parser(clap::value_parser!(String))
                .action(ArgAction::Set))
            )
        .subcommand(Command::new("delete").about(""))
        .subcommand(Command::new("complete").about(""))
        .subcommand(Command::new("show").about("")
            .arg(arg!(-n --number "number of tasks to show")
                .default_value("5")
                .value_parser(clap::value_parser!(u32))
                .action(ArgAction::Set)
        ));

    let matches = cmd.get_matches();
    match matches.subcommand() {
        Some(("add", matches)) => add(matches), 
        Some(("delete", matches)) => delete(matches), 
        Some(("complete", matches)) => complete(matches), 
        Some(("show", matches)) => show(matches), 
        _ => unreachable!("clap should ensure that we don't get here"),
    };
}
