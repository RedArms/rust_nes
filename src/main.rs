fn main() {
}

struct CPU {
    ram:[u8;0xFFFF],
    pc:u16,
    a:u8,
    x:u8,
    y:u8,
    flags:u8,
    stackpointer:u8,
}

impl CPU {

    const STACK:u16 = 0x33;

    fn fetch_next_byte(&self) -> u8 {
        return self.ram[self.pc as usize+ 1];
    }
    fn fetch_next_two_byte(&self) -> (u8,u8) {
        return (self.ram[self.pc as usize+ 1],self.ram[self.pc as usize+ 2]);
    }
    fn fetch_next_two_byte_as_u16(&self) -> u16 {
        let ret = (self.ram[self.pc as usize+ 1],self.ram[self.pc as usize+ 2]);
        return ((ret.1 as u16) << 8) | (ret.0 as u16)

    }
    fn fetch_next_three_byte(&self) -> (u8,u8,u8) {
        return (self.ram[self.pc as usize+ 1],self.ram[self.pc as usize+ 2],self.ram[self.pc as usize+ 3]);
    }

    fn execute(&mut self) {
        let opcode = self.ram[self.pc as usize];

        match opcode {

            //JMP absolute
            0x4C=>{
                self.pc = self.fetch_next_two_byte_as_u16()
            },
            //NOP
            0xEA=>{
                self.pc +=1;
            }
            _=>{print!("Unknow opcode : {:#04x}", opcode);}
        }
    }


    
}