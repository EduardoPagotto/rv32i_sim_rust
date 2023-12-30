pub const DEV_OPENED: u8 = 0b00000001;
pub const DEV_CHANGED: u8 = 0b00000010;
pub const DEV_RW: u8 = 0b00000100;

pub struct Memory {
    pub start: u32,
    pub top: u32,
    status: u8,
    pub mem: Vec<u8>,
}

impl Memory {
    pub fn new(init: u32, size: usize, status: u8) -> Self {
        let mut m: Vec<u8> = Vec::with_capacity(size);

        for _ in 0..size {
            m.push(0xfe);
        }

        Self {
            start: init,
            top: init + size as u32,
            status: status,
            mem: m,
        }
    }

    pub fn is_open(&self) -> bool {
        (self.status & DEV_OPENED) != 0
    }

    pub fn is_writetable(&self) -> bool {
        (self.status & (DEV_RW | DEV_OPENED)) == 0x5
    }

    pub fn valid_write(&self, address: u32, size: u32) -> bool {
        return (self.is_writetable() && (address >= self.start) && ((address + size) <= self.top));
    }

    pub fn valid_range(&self, address: u32, size: u32) -> bool {
        self.is_open() && (address >= self.start) && (address < self.top + size)
    }

    pub fn read(&self, address: u32, size: u32, signed: bool) -> Option<u32> {
        // in range ?
        if self.valid_range(address, size as u32) {
            // init 0 and base addr
            let mut result: u32 = 0;
            let addr_base = address - self.start;

            for i in 0..size {
                let offset = (addr_base + i) as usize;
                result = result | (self.mem[offset] as u32) << (8 * i);
            }

            // TODO: implementar sinal do ultimo digito
            if signed {
                let last_val = (addr_base + size - 1) as usize;
                if (self.mem[last_val] >> 7) == 1 {
                    //ultimo digito e sinal ?

                    let mut i = 4;
                    while i > size {
                        result = result | 0xff << (8 * (i - 1));
                        i -= 1;
                    }
                }
            }

            // TODO: ver como fica o iterator
            //self.status &= !DEV_CHANGED;

            return Some(result);
        }

        None
    }

    pub fn write(&mut self, address: u32, size: u32, value: u32) -> bool {
        // in range ?
        if self.valid_write(address, size as u32) {
            // init 0 and base addr
            //let mut result: u32 = 0;
            let addr_base = address - self.start;

            for i in 0..size {
                let offset = (addr_base + i) as usize;

                let r = (value >> (8 * i) & 0xff) as u8;

                self.mem[offset] = r;

                //result = result | (self.mem[offset] as u32) << (8 * i);
            }

            self.status |= DEV_CHANGED; // set changed

            return true;
        }

        false
    }

    pub fn get_status(&self) -> u8 {
        self.status
    }

    pub fn open(&mut self) {
        self.status |= DEV_OPENED;
    }
    pub fn close(&mut self) {
        self.status &= !DEV_OPENED;
    }

    pub fn is_rw(&self) -> bool {
        (self.status & DEV_RW) != 0
    }

    pub fn set_rw(&mut self, enable: bool) {
        match enable {
            true => self.status |= DEV_RW,
            false => self.status &= !DEV_RW,
        }
    }
}
