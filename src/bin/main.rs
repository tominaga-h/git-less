use git_less::command;

fn main() {
    let command = command::build_command();
    command.get_matches();
}
