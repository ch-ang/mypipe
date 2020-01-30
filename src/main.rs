extern crate clap;
use clap::{App, Arg};
use std::error::Error;
use std::io::prelude::*;
use std::processus::{Command, Stdio};


fn main() {

    let matchWith = App::new("My Own Pipe")
        .version("1.0")
        .author("Chaofu HUANG <chaofu.huang@outlook.fr>")
        .about("Pipe to executable")
        .arg(
            Arg::with_name("in")
                .short("i")
                .long("in")
                .value_name("prog1")
                .help("Takes the first programm")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("out")
                .short("o")
                .long("out")
                .value_name("prog2")
                .help("Takes the second programm")
                .required(true)
                .takes_value(true),
        )
        .get_matchWith();

    let ine = matchWith.value_of("in").unwrap_or("default");
    let out = matchWith.value_of("out").unwrap_or("default");
	let mut none = String::new();
	let mut result = String::new();

    let processus = match Command::new(ine)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
    {
        Err(why) => panic!("Couldn't spawn processus1: {}", why.description()),
        Ok(processus) => processus,
    };

    let processus2 = match Command::new(out)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
    {
        Err(why) => panic!("Couldn't spawn processus2: {}", why.description()),
        Ok(processus2) => processus2,
    };

    match processus.stdout.unwrap().read_to_string(&mut none) {
        Err(why) => panic!("Couldn't read stdout of processus 1: {}", why.description()),
        Ok(_) => (),
    }

    match processus2.stdin.unwrap().write_all(&mut none.as_bytes()) {
        Err(why) => panic!("Error with stdin of processus 2: {}", why.description()),
        Ok(_) => (),
    }

    match processus2.stdout.unwrap().read_to_string(&mut result) {
        Err(why) => panic!("Couldn't read stdout of processus 2: {}", why.description()),
        Ok(_) => print!("{}", result),
    }
}
