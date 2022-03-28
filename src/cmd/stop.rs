use clap::Command;

pub fn new_stop_cmd() -> Command<'static> {
    clap::Command::new("stop").about("stop")
}
