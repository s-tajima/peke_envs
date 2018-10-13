extern crate elf;
extern crate proc_maps;
extern crate regex;

use self::regex::Regex;

pub fn get_elf_symbol_value(filename: String, symbol_name: &str) -> Result<u64, String> {
    let file = elf::File::open_path(filename).ok().ok_or("open_path error")?;

    for s in &file.sections {
        let f: Vec<u64> = file.get_symbols(&s).ok().ok_or("get_symbols error")?
            .iter()
            .filter(|sym| sym.name == symbol_name )
            .map(|sym| sym.value )
            .collect();

        if f.first().is_none() { continue }
        return Ok(*f.first().unwrap())
    }
    Err("No symbol found")?
}

pub fn get_maps(pid: i32) -> Result<proc_maps::MapRange, String> {
    let maps = proc_maps::get_process_maps(pid as proc_maps::Pid)
        .map_err(|e| format!("Failed to get_maps: {} (pid:{})", e, pid ))?;

    let re = Regex::new(r"(libc\.so|libc\.so\.\d+|libc-\d+\.\d+\.so)$")
        .map_err(|e| format!("Failed to build regex: {}", e))?;

    let maps: Vec<proc_maps::MapRange> = maps.iter()
        .filter(|m| m.filename().is_some() )
        .filter(|m| re.is_match(&m.filename().clone().unwrap()) )
        .filter(|m| m.is_exec() && m.is_read() )
        .cloned()
        .collect();

    Ok(maps.first().unwrap().clone())
}

