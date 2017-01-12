extern crate clap;
extern crate toml;
extern crate rustc_serialize;

use std::collections::HashMap;
use std::env;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::Read;
use std::process;
use clap::{Arg, App};

#[derive(RustcDecodable, Debug)]
struct Command {
    cmd: String,
    args: Option<Vec<String>>,
    desc: Option<String>,
}

impl Command {
    fn execute(&self) {
        let mut cmd = process::Command::new(&self.cmd);
        if self.args.is_some() {
            cmd.args(self.args.as_ref().unwrap());
        }

        let ch = cmd.spawn();
        match ch {
            Ok(mut c) => {
                c.wait();
            }
            Err(e) => println!("err {}", e),
        };
    }
}

struct CommandStore {
    commands: HashMap<String, Command>,
}

impl CommandStore {
    fn load(toml: &str) -> Option<CommandStore> {
        match toml::decode_str(toml) {
            Some(c) => Some(CommandStore { commands: c }),
            None => None,
        }
    }

    fn get(&self, name: &str) -> Option<&Command> {
        self.commands.get(name)
    }
}

impl fmt::Display for CommandStore {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let empty = "";
        let message = self.commands
                          .iter()
                          .map(|(k, v)| {
                              let desc = match v.desc {
                                  Some(ref d) => d,
                                  None => empty,
                              };
                              format!("{}\t{}", k, desc)
                          })
                          .collect::<Vec<String>>()
                          .join("\n");
        write!(f, "{}", message)
    }
}


fn main() {
    let matches = App::new("buri")
                      .version("0.1")
                      .author("meganehouser <sleepy.st818@gmail.com>")
                      .about("Command launcher")
                      .arg(Arg::with_name("INPUT")
                               .help("Input name")
                               .required(false)
                               .index(1))
                      .get_matches();

    let mut home = env::home_dir().expect("getting home dir error");
    home.push("buri.toml");
    let mut buri_file = if home.exists() {
        File::open(home).expect("open error")
    } else {
        File::create(home).expect("create error")
    };

    let mut toml = String::new();
    buri_file.read_to_string(&mut toml);
    let cmd_store = CommandStore::load(&toml).unwrap();

    match matches.value_of("INPUT") {
        Some(c) => {
            cmd_store.get(c).unwrap().execute();
        }
        None => println!("{}", cmd_store),
    };
}
