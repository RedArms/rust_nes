use std::iter::FlatMap;

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

#[repr(u8)]
#[derive(Clone, Copy)]
enum FLAGS {
    CARRY     = 0,
    ZERO      = 1,
    INTERRUPT = 2,
    DECIMAL   = 3,
    B         = 4,
    ONE       = 5,
    OVERFLOW  = 6,
    NEGATIVE  = 7
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
    fn set_flag(&mut self,up:bool,flag:FLAGS) {
        if up {
            self.flags = self.flags | 1 << (flag as u8);
        }
        else {
            self.flags = self.flags & (0xF ^ (1 << flag as u8));
        }
    }
    fn get_xth_bit(x:u8,element:u8) -> bool {
        return ((element>>x) & 0xFE) > 0
    }

    fn check_zero_carry(&mut self,x:u8) {
        if x == 0 {
            self.set_flag(true, FLAGS::ZERO)
        }
        if Self::get_xth_bit(7, x) {
            self.set_flag(true, FLAGS::NEGATIVE)

        }
    }

    fn execute(&mut self) {
        let opcode = self.ram[self.pc as usize];

        match opcode {

            //CLEAR FLAGS
            //CLC
            0x18=>{
                self.set_flag(false, FLAGS::CARRY);
                self.pc +=1;
            },
            //CLD
            0xD8=>{
                self.set_flag(false, FLAGS::DECIMAL);
                self.pc +=1;
            },
            //CLI
            0x58=>{
                self.set_flag(false, FLAGS::INTERRUPT);
                self.pc +=1;
            },
            //CLV
            0xB8=>{
                self.set_flag(false, FLAGS::OVERFLOW);
                self.pc +=1;
            }

            //JMP absolute
            0x4C=>{
                self.pc = self.fetch_next_two_byte_as_u16()
            },
            //NOP
            0xEA=>{
                self.pc +=1;
            },
            

            //INC ZP
            0xE6=>{
                self.ram[self.fetch_next_byte() as usize] 
                = self.ram[self.fetch_next_byte() as usize] + 1;
                self.check_zero_carry(self.ram[self.fetch_next_byte() as usize]);
            },
            //INC ZPX
            0xF6=>{
                self.ram[(self.fetch_next_byte() + self.x) as usize] 
                = self.ram[(self.fetch_next_byte() + self.x) as usize] + 1;
                self.check_zero_carry(self.ram[(self.fetch_next_byte() + self.x) as usize]);
            },
            //INC ABS
            0xEE=>{
                self.ram[self.fetch_next_two_byte_as_u16() as usize]
                 = self.ram[self.fetch_next_two_byte_as_u16() as usize] + 1;
                self.check_zero_carry(self.ram[self.fetch_next_two_byte_as_u16() as usize]);
            }, 
            //INC ABS
            0xFE=>{
                self.ram[(self.fetch_next_two_byte_as_u16() + self.x as u16) as usize] 
                = self.ram[(self.fetch_next_two_byte_as_u16() + self.x as u16) as usize] + 1;
                self.check_zero_carry(self.ram[(self.fetch_next_two_byte_as_u16() + self.x as u16) as usize]);
            },                       
            //INX
            0xE8=>{
                self.x = self.x + 1;
                self.check_zero_carry(self.x);
            },
            //INY
            0xC8=>{
                self.y = self.y + 1;
                self.check_zero_carry(self.y);
            }
            _=>{print!("Unknow opcode : {:#04x}", opcode);}
        }
    }


    
}