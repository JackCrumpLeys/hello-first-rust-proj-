use std::{fs, io};
use std::io::Write;
use std::path::PathBuf;
use hello::*;
use rand::Rng;
use rusty_leveldb::{DB, DBIterator, LdbIterator, Options};

mod ex_b;
mod ex_c;
mod ex_d;
mod ex_e;
mod ex_f;
mod ex_g;
mod level_db_dumper;

// constants:
const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    // //speed_test
    // let speed:Duration = speed_test();
    // println!("speed test took: {}", speed.as_millis());

    // hello world test
    let name = "jack";

    hello(name);
    blank_ln();

    let mut rng = rand::thread_rng();

    let i: i32 = rng.gen();
    let f: f64 = rng.gen();
    let roll = rng.gen_range(1..7);
    println!("i:{}, f:{}, roll:{}", i, f, roll);
    blank_ln();

    let x = if name == "jack" {
        4
    } else {
        5
    };
    let (y, word) = loop {
        break (2, "hello")
    };

    println!("x:{}, y:{}, word:{}", x, y, word);
    blank_ln();

    // missiles!
    let mut missiles = STARTING_MISSILES;
    let ready = READY_AMOUNT;
    println!("Firing {} of my {} missiles...", ready,missiles);
    missiles -= ready;

    println!("{} missiles left!", missiles);

    let mut opt = rusty_leveldb::in_memory();
    opt.reuse_logs = false;
    opt.reuse_manifest = false;
    opt.compression_type = rusty_leveldb::CompressionType::CompressionNone;
    let db_dir = PathBuf::from("./leveldb");
    let abs_path = fs::canonicalize(&db_dir);
    println!("{:?}", abs_path);
    let mut db = DB::open(abs_path.unwrap(), opt).unwrap();

    db.put(b"Hello", b"World").unwrap();
    assert_eq!(b"World", db.get(b"Hello").unwrap().as_slice());

    let mut it = db.new_iter().unwrap();
    let (mut k, mut v) = (vec![], vec![]);
    while it.advance() {
        it.current(&mut k, &mut v);
        print!("{}", std::str::from_utf8(&k).unwrap());
        print!("{}",(" => "));
        print!("{}",std::str::from_utf8(&v).unwrap());
        print!("{}",("\n"));
    }
    // loop {
    //     let i = iter.next().unwrap();
    //     println!("{:?}", i);
    //     if !iter.valid() {break}
    // }

    blank_ln();
    ex_b::main();
    blank_ln();
    ex_c::main();
    blank_ln();
    ex_d::main();
    blank_ln();
    ex_e::main();
    blank_ln();
    ex_f::main();
    blank_ln();
    ex_g::main();
    blank_ln();



    println!("attempting to execute script!");
    let script_result = script_stuff::run_main::<rhai_script_host::RhaiScriptHost>();
    println!("exit status: {}", if script_result.is_ok() {"OK"} else {"ERROR"});
}

fn hello(name:&str) {
    println!("Hello, {}", name);
}
