use clap::{arg, ArgAction, ArgMatches, Command, Arg};
use std::any::Any;
use std::string::String;
use std::option::Option;
use tabled::{builder::Builder, settings::Style};
use chrono::{DateTime, FixedOffset, Local};

use crate::task::task::Task;
use crate::comparible_task::comparible_task::ComparibleTask;
use crate::show_rule::show_rule::ShowRule;

mod task;
mod comparible_task;
mod db;
mod show_rule;

fn get_string_arg(args: &ArgMatches, arg_name: &str) -> String {
    return String::from(args.get_one::<String>(arg_name).unwrap());
}

fn get_num_arg<T: Any + Clone + Send + Sync + Copy>(args: &ArgMatches, arg_name: &str) -> T {
    return *args.get_one::<T>(arg_name).unwrap(); 
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
    let cost = get_num_arg::<u32>(args, "cost");
    let priority = get_num_arg::<u32>(args, "priority");
    let deadline = get_optional::<DateTime::<FixedOffset>>(
        args,
        "deadline",
        &get_datetime
    );

    let desc = get_optional::<String>(args, "description", &get_string_arg);
    let date_created = DateTime::from(Local::now());

    return Task::new(name, desc, cost, priority, date_created, None, deadline);
}

fn add(args: &ArgMatches) {
    let task: Task = build_task(args);
    crate::db::db::write_task(task);
}

fn delete(args: &ArgMatches) {
}

fn complete(args: &ArgMatches) {
    let id = get_num_arg::<usize>(args, "taskId");
    crate::db::db::mark_complete(id);
}

fn show(args: &ArgMatches) {
    let total = match args.get_flag("all") {
        true => usize::MAX,
        false => get_num_arg::<usize>(args, "number")
    };

    let show_completed = args.get_flag("completed");
    let tasks: Vec::<ComparibleTask> = crate::db::db::get_tasks(total, show_completed);

    let verbose: bool = args.get_flag("verbose");
    let mut builder = Builder::default();

    let show_rule = ShowRule::from(verbose, show_completed);
    ComparibleTask::add_tasks_to_table(tasks, &mut builder, &show_rule);

    let mut table = builder.build();
    table.with(Style::ascii_rounded());

    println!("{table}");
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
            .arg(arg!(-c --cost "the expected time in *minutes* that this task will take")
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
                    yyyy-mm-ddThh::mm::ss in military time (always assumes local timezone)"
                )
                .action(ArgAction::Set))
            )
        .subcommand(Command::new("delete").about(""))
        .subcommand(Command::new("complete")
            .about("mark a task completed by index")
            .arg(Arg::new("taskId")
                .help("the id of the task to mark complete, where the task id is in the left-most column")
                .action(ArgAction::Set)
                .required(true)
                .value_parser(clap::value_parser!(usize))))
        .subcommand(Command::new("show").about("display tasks")
            .arg(arg!(-n --number "number of tasks to show")
                .default_value("5")
                .value_parser(clap::value_parser!(usize))
                .action(ArgAction::Set))
            .arg(arg!(-a --all "show all tasks")
                .action(ArgAction::SetTrue))
            .arg(arg!(-c --completed "show completed tasks")
                .action(ArgAction::SetTrue))
            .arg(arg!(-v --verbose "show all task information")
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
