use clap::Command;

pub fn new_start_cmd() -> Command<'static> {
    clap::Command::new("start").about("start")
}
