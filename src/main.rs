extern crate clap;
extern crate toml;
extern crate rustc_serialize;

use std::collections::HashMap;
use std::env;
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::process;
use clap::{Arg, App};

#[derive(RustcDecodable, Debug)]
struct Command {
    cmd: String,
    spawn: Option<bool>,
    args: Option<Vec<String>>,
    desc: Option<String>,
}

impl Command {
    fn execute(&self) {
        let is_spawn = self.spawn.unwrap_or(false);
        let mut args = Vec::<String>::new();

        let mut cmd = if is_spawn {
            let cmd = process::Command::new("cmd");
            args.push("/c".to_string());
            args.push(self.cmd.to_owned());
            cmd
        } else {
            let cmd = process::Command::new(&self.cmd);
            cmd
        };

        if self.args.is_some() {
            args.extend_from_slice(&self.args.as_ref().unwrap().as_slice());
        }

        cmd.args(args);

        let ch = cmd.spawn();
        if let Ok(mut c) = ch {
            if !is_spawn {
                c.wait();
            }
        } else {
            let e = ch.err().unwrap();
            println!("err {}", e);
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
                      .version("0.2")
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
