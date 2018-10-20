#![warn(deprecated)]
#![warn(dead_code)]

mod process;
mod maps;

extern crate libc;
extern crate elf;
extern crate nix;
extern crate hex;
extern crate failure;

//use std::mem;
use std::env;
use std::str;
use std::thread;
use std::time::Duration;

fn main() {
    let pid_str: String = env::args().nth(1).unwrap();
    let pid_int: i32 = pid_str.parse().unwrap();
    let pid = nix::unistd::Pid::from_raw(pid_int);

    let p = process::Process{pid};

    let symbol_name: &str = "__environ";

    let map = maps::get_maps(pid_int).unwrap();
    let filename = map.filename().clone().unwrap().to_string();
    let offset = maps::get_elf_symbol_value(filename, symbol_name).unwrap();
    let addr_env_ptr_ptr: u64 = map.start() as u64 + offset as u64;

    p.attach().unwrap();
    thread::sleep(Duration::from_millis(50));
    p.setoptions().unwrap();
    thread::sleep(Duration::from_millis(50));

    let env_ptr_ptr = p.peek(addr_env_ptr_ptr).unwrap();
    let env_ptrs = p.get_env_ptrs(env_ptr_ptr).unwrap();

    for env_ptr in env_ptrs {
        let result = p.get_env_val(env_ptr);
        println!("{}", result.unwrap());
    }
}
