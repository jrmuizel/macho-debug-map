extern crate goblin;

use goblin::{error, Object};
use goblin::mach::*;
use std::path::Path;
use std::env;
use std::fs::File;
use std::io::Read;

fn run() -> error::Result<()> {
    for (i, arg) in env::args().enumerate() {
        if i == 1 {
            let path = Path::new(arg.as_str());
            let mut fd = File::open(path)?;
            let mut buffer = Vec::new();
            fd.read_to_end(&mut buffer)?;
            match Object::parse(&buffer)? {
                Object::Elf(elf) => {
                    println!("elf: {:#?}", &elf);
                },
                Object::PE(pe) => {
                    println!("pe: {:#?}", &pe);
                },
                Object::Mach(mach) => {
                    match mach {
                        Mach::Binary(mach) => {
                            //println!("mach: {:#?}", &mach);
                            for s in mach.symbols() {
                                let s = s.unwrap();
                                if s.1.n_type & symbols::N_STAB != 0 {
                                    if s.1.n_type == symbols::N_OSO {
                                        println!("OSO {}", s.0);
                                    } else {
                                        // the symbols in the OSO file are looked up and then remapped by name to
                                        // the
                                        println!("stab {} {:x} {} {}", s.0, s.1.n_value, s.1.n_desc, s.1.n_sect);
                                    }
                                    if s.1.n_type == symbols::N_SO {
                                        if s.0 == "" {
                                            // this is the end of the OSO list
                                        }
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                },
                Object::Archive(archive) => {
                    println!("archive: {:#?}", &archive);
                },
                Object::Unknown(magic) => { println!("unknown magic: {:#x}", magic) }
            }
        }
    }
    Ok(())
}

fn main() {
    run();
}
