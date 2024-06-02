use clap::ArgMatches;

pub fn get_task_number(sub_matches: &ArgMatches) -> Vec<usize> {
    // Get all task numbers the user has provide as value to the subcommand
    // For each task number, we subtract 1 from, as from the program perspective the first item starts at index 0
    sub_matches.get_many::<usize>("TASK_NUMBER").unwrap_or_default().map(|v| *v-1).collect::<Vec<_>>()
}