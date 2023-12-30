mod hardware;

use hardware::bus::{Bus, MemoryAccessWidth};
use hardware::memory::{Memory, DEV_OPENED, DEV_RW};

fn main() {
    let mut bus = Bus::new();
    let width = MemoryAccessWidth::Word;
    let id_rom: usize = bus.add(Memory::new(0x0000, 0x1000, DEV_OPENED));
    let id_ram: usize = bus.add(Memory::new(0x1000, 0x1000, DEV_OPENED | DEV_RW));

    bus.load_file(&String::from("./pgm.bin"), id_rom);

    bus.store(0x0000, 0x80000000, width);

    let result = bus.load(0x0000, width, true);

    match result {
        Some(val) => {
            println!("V: {:#010x}", val);
        }

        None => {
            println!("Invalido");
        }
    }
}
