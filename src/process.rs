extern crate nix;
extern crate libc;

use nix::sys::ptrace;
use std::mem::transmute;
use std::ptr;
use std::str;
use std::time::Duration;
use std::thread;

pub struct Process {
    pub pid: nix::unistd::Pid 
}

impl Process {
    pub fn attach(&self) -> Result<(), String> {
        ptrace::attach(self.pid).map_err(|e| format!("Failed to ptrace attach {} ({})", self.pid, e))
    }
    
    pub fn detach(&self) -> Result<(), String> {
        ptrace::detach(self.pid).map_err(|e| format!("Failed to ptrace detach {} ({})", self.pid, e))
    }
    
    pub fn setoptions(&self) -> Result<(), String> {
        ptrace::setoptions(self.pid, nix::sys::ptrace::Options::PTRACE_O_TRACESYSGOOD).map_err(|e| format!("Failed to ptrace setoptions {} ({})", self.pid, e))
    }
    
    pub fn syscall(&self) -> Result<(), String> {
        ptrace::syscall(self.pid).map_err(|e| format!("failed to ptrace syscall {} ({})", self.pid, e))
    }
    
    pub fn peek(&self, addr: u64) -> Result<u64, String> {
        unsafe { ptrace::ptrace(
            ptrace::Request::PTRACE_PEEKDATA,
            self.pid,
            addr as *mut libc::c_void,
            ptr::null_mut(),
        )}
        .map(|i| i as u64) 
        .map_err(|e| format!("failed to ptrace peek {} ({})", self.pid, e))
    }

    pub fn poke(&self, addr: u64, data: u64) -> Result<u64, String> {
        unsafe { ptrace::ptrace(
            ptrace::Request::PTRACE_POKEDATA,
            self.pid,
            addr as *mut libc::c_void,
            data as *mut libc::c_void,
        )}
        .map(|i| i as u64) 
        .map_err(|e| format!("failed to ptrace poke {} ({})", self.pid, e))
    }


    pub fn get_env_ptrs(&self, ptr: u64) -> Result<Vec<u64>, String> {
        let mut vec: Vec<u64> = Vec::new();
        let mut cursor = ptr;
    
        loop { 
            let val = self.peek(cursor as u64)?;
            cursor += 8;
    
            if val == 0 { break };
    
            vec.push(val);
        }
    
        Ok(vec)
    }
    
    pub fn get_env_val(&self, ptr: u64) -> Result<String, String> {
        let mut val: String = "".to_string();
        let mut cursor = ptr;
    
        loop { 
            let raw = self.peek(cursor as u64)?.swap_bytes();
            let bytes: [u8; 8] = unsafe { transmute(raw.to_be()) };
    
            let index = bytes.iter().position(|&x| x == 0x00);
    
            match index {
                None => { 
                    val = format!("{}{}", val, str::from_utf8(&bytes).unwrap());
                },
                Some(i) => { 
                    val = format!("{}{}", val, str::from_utf8(&bytes[0..i]).unwrap());
                    break;
                }, 
            };
            cursor += 8;
        }
    
        Ok(val)
    }

}

impl Drop for Process {
    fn drop(&mut self) {
        self.syscall().unwrap();
        thread::sleep(Duration::from_millis(200));
        self.detach().unwrap();
    }
}

