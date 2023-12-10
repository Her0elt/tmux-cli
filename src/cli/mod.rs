use core::panic;
use std::process::Command as OsCommand;
use std::{env::Args, string::ParseError};

use simple_home_dir::home_dir;

use crate::fzf::FzfHandler;
use crate::tmux::TmuxHandler;
extern crate tmux_interface;

#[derive(Debug)]
enum Command {
    Sessionize(String),
}
#[derive(Debug)]
pub struct Cli {
    command: Command,
}

impl Cli {
    pub fn execute_command(&self) {
        match &self.command {
            Command::Sessionize(root_dir) => self.execute_sessionize(root_dir),
        }
    }

    fn execute_sessionize(&self, root_dir: &str) {
        let dirs = self.execute_find(root_dir);
        let dir = FzfHandler::execute_fzf(&dirs);
        if !dir.is_empty() {
            TmuxHandler::execute_tmux_sessionize(dir);
        }
    }

    fn execute_find(&self, root_dir: &str) -> Vec<String> {
        let mut cmd = OsCommand::new("find");
        let dirs = cmd
            .arg(root_dir)
            .arg("-mindepth")
            .arg("1")
            .arg("-maxdepth")
            .arg("1")
            .arg("-type")
            .arg("d")
            .output();
        if dirs.is_err() {
            panic!("Something went wrong");
        }
        let dirs = dirs.unwrap().stdout;
        let dirs = String::from_utf8(dirs).unwrap().clone();
        dirs.split('\n').map(String::from).collect::<Vec<String>>()
    }
}

impl TryFrom<Args> for Cli {
    type Error = ParseError;

    fn try_from(mut args: Args) -> Result<Self, Self::Error> {
        let command = args.nth(1);
        if command.is_none() {
            panic!("You have to provide a command!");
        }
        let command = match command.unwrap().as_str() {
            "sessionize" => {
                let root_dir = args
                    .next()
                    .unwrap_or(String::from(home_dir().unwrap().to_str().unwrap()));
                Command::Sessionize(root_dir)
            }
            _ => panic!("Not a valid command"),
        };
        Ok(Cli { command })
    }
}
