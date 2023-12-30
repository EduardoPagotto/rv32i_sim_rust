//mod device;
use crate::Memory;
use std::fs::{self, File};
use std::io::Read;

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum MemoryAccessWidth {
    Byte = 0x01,     // 1 byte 8 bits
    HalfWord = 0x02, // 2 bytes 16 bits (little endian)
    Word = 0x04,     // 4 bytes 32 bits (little endian)
    DWord = 0x08,    // 8 bytes 64 bits (little endian)
}

pub struct Bus {
    pub bank: Vec<Memory>,
}

impl Bus {
    pub fn new() -> Self {
        let m: Vec<Memory> = Vec::with_capacity(8);
        Self { bank: m }
    }

    pub fn add(&mut self, memory: Memory) -> usize {
        self.bank.push(memory);
        self.bank.len() - 1
    }

    pub fn load(&self, address: u32, width: MemoryAccessWidth, signed: bool) -> Option<u32> {
        let size = width as u32;

        for m in &self.bank {
            let result = m.read(address, size, signed);

            match result {
                Some(_) => {
                    return result;
                }
                None => {
                    continue;
                }
            }
        }

        None
    }

    pub fn store(&mut self, address: u32, value: u32, width: MemoryAccessWidth) -> bool {
        let size = width as u32;

        for m in &mut self.bank {
            if m.write(address, size, value) {
                return true;
            }
        }

        false
    }

    pub fn load_file(&mut self, filename: &String, indice: usize) {
        let mut f = File::open(&filename).expect("no file found");
        let metadata = fs::metadata(&filename).expect("unable to read metadata");
        let mut buffer = vec![0; metadata.len() as usize];
        f.read(&mut buffer).expect("buffer overflow");

        for i in 0..buffer.len() {
            self.bank[indice].mem[i] = buffer[i];
        }
    }
}
