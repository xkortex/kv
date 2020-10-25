// -*- compile-command: "cargo build" -*-
#[macro_use]
extern crate clap;
extern crate core;

const DBPATH: &str = "/tmp/kv.db";

use clap::{App, Arg};
use std::{env};
use std::time::{Duration, Instant};
use std::io::{stdout, stderr};
use termion::{color};

use kv::*;
use std::borrow::Borrow;

#[derive(serde::Serialize, serde::Deserialize, PartialEq)]
struct SomeType {
    a: i32,
    b: i32
}

fn run() -> Result<(), Error> {
    let stdout = stdout();
    let stderr = stderr();
    let matches = App::new("kv")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Simple KV utility",
        )
        .arg(
            Arg::with_name("KEY")
                .help("Key to set")
                .index(1),
        )
        .arg(
            Arg::with_name("VAL")
                .help("Value to set")
                .index(2),
        )
        .arg(
            Arg::with_name("set")
                .help("set a value")
                .long("set")
                .short("s")
        )
        .arg(
            Arg::with_name("get")
                .help("get a value")
                .long("get")
                .short("g")
        )
        .arg(
            Arg::with_name("delete")
                .help("Permanently deletes a value")
                .short("d")
                .long("del"),
        )
        .arg(
            Arg::with_name("list")
                .help("Prints values")
                .short("l")
                .long("list"),
        )
        .get_matches();



    // Configure the database
    let mut cfg = Config::new(DBPATH);

    // Open the key/value store
    let store = Store::new(cfg)?;


    // A Bucket provides typed access to a section of the key/value store
    let bucket = store.bucket::<String, String>(Some("stest"))?;

    let key = matches.value_of("KEY").unwrap();
    let mval = matches.value_of("VAL");

    if matches.is_present("set") {
        let val = mval.unwrap();
        let now = Instant::now();

        set(bucket, key.into(), val.into());
        eprintln!("{}{}: {} -> {}", color::Fg(color::Yellow), "ok set", key, val );
        eprintln!("{}{}: {}", color::Fg(color::Yellow), "elapsed", now.elapsed().as_micros());

        store.export();
        eprintln!("{}{}: {}", color::Fg(color::Yellow), "elapsed", now.elapsed().as_micros());

        return Ok(());
    }
    if matches.is_present("get") {

        eprintln!("{}{}: {}", color::Fg(color::Yellow), "ok get", key);
        let now = Instant::now();
        let outval = bucket.get(key).unwrap().unwrap();
        eprintln!("{}{}: {}", color::Fg(color::Yellow), "elapsed", now.elapsed().as_micros());
        println!("{}", outval);

        return Ok(());
    }

    eprintln!("{}\nkv -h for help", matches.usage());

    Ok(())
}

 fn main() {
     run().unwrap();
 }

fn set(b: Bucket<String, String>, k: String, v: String) -> bool {
    b.set(k, v);
    return true
}

fn get(b: Bucket<String, String>, k: String) -> String {
    return b.get(k).unwrap().unwrap();

}

