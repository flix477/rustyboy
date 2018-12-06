mod joypad;
use cartridge::Cartridge;
use std::error::Error;
use config::Config;
use processor::interrupt::Interrupt;
use processor::interrupt::InterruptHandler;
use self::joypad::Joypad;
use bus::{Bus, Readable, Writable};
use video::Video;

pub struct Hardware {
    cartridge: Cartridge,
    interrupt_handler: InterruptHandler,
    joypad: Joypad,
    video: Video,
    internal_ram: [u8; 8000],
    high_ram: [u8; 127]
}

impl Hardware {
    pub fn new(config: Config) -> Result<Hardware, Box<dyn Error>> {
        Ok(Hardware {
            cartridge: config.cartridge,
            interrupt_handler: InterruptHandler::new(),
            joypad: Joypad::new(),
            video: Video::new(),
            internal_ram: [0; 8000],
            high_ram: [0; 127]
        })
    }
}

impl Readable for Hardware {
    fn read(&self, address: u16) -> u8 {
        match address {
            0...0x7FFF |
            0xA000...0xBFFF => self.cartridge.read(address),

            0xFF40...0xFF4B |
            0x8000...0x9FFF => self.video.read(address), // lcdc|video ram,

            0xC000...0xCFFF => {
                let address = address - 0xC000;
                self.internal_ram[address as usize]
            }, // 8kb internal ram
            0xD000...0xDFFF => 0, // echo ^^

            0xFE00...0xFEFF => 0, // sprite attrib
            0xFF00 => self.joypad.read(address), // joypad info
            0xFF01 => unimplemented!(), // serial transfer data
            0xFF02 => unimplemented!(), // sio control
            0xFF04 => unimplemented!(), // divider register
            0xFF05 => unimplemented!(), // timer counter
            0xFF06 => unimplemented!(), // timer modulo
            0xFF07 => unimplemented!(), // timer control

            0xFF0F |
            0xFFFF => self.interrupt_handler.read(address), // interrupt

            0xFF10 => unimplemented!(), // sound mode 1 register, sweep
            0xFF11 => unimplemented!(), // sound mode 1 register, sound length
            0xFF12 => unimplemented!(), // sound mode 1 register, envelope
            0xFF13 => unimplemented!(), // sound mode 1 register, frequency low
            0xFF14 => unimplemented!(), // sound mode 1 register, frequency high
            0xFF16 => unimplemented!(), // sound mode 2 register, sound length
            0xFF17 => unimplemented!(), // sound mode 2 register, envelope
            0xFF18 => unimplemented!(), // sound mode 2 register, frequency low
            0xFF19 => unimplemented!(), // sound mode 2 register, frequency high
            0xFF1A => unimplemented!(), // sound mode 3 register, on/off
            0xFF1B => unimplemented!(), // sound mode 3 register, sound length
            0xFF1C => unimplemented!(), // sound mode 3 register, output level
            0xFF1D => unimplemented!(), // sound mode 3 register, frequency low
            0xFF1E => unimplemented!(), // sound mode 3 register, frequency high
            0xFF20 => unimplemented!(), // sound mode 4 register, sound length
            0xFF21 => unimplemented!(), // sound mode 4 register, envelope
            0xFF22 => unimplemented!(), // sound mode 4 register, polynomial counter
            0xFF23 => unimplemented!(), // sound mode 4 register, counter/consecutive
            0xFF24 => unimplemented!(), // channel control - on/off - volume
            0xFF25 => unimplemented!(), // sound output terminal selection
            0xFF26 => unimplemented!(), // sound on/off
            0xFF30...0xFF3F => unimplemented!(), // waveform ram

            0xFF80...0xFFFE => {
                let address = address - 0xFF80;
                self.high_ram[address as usize]
            }, // high ram
            _ => 0 // empty
        }
    }
}

impl Writable for Hardware {
    fn write(&mut self, address: u16, value: u8) {
        match address {
            0...0x7FFF |
            0xA000...0xBFFF => self.cartridge.write(address, value),
            0xFF40...0xFF4B |
            0x8000...0x9FFF => self.video.write(address, value), // lcdc|video ram,

            0xC000...0xCFFF => {
                let address = address - 0xC000;
                self.internal_ram[address as usize] = value;
            }, // 8kb internal ram
            0xD000...0xDFFF => unimplemented!(), // echo ^^

            0xFE00...0xFEFF => unimplemented!(), // sprite attrib
            0xFF00 => self.joypad.write(address, value), // joypad
            0xFF0F |
            0xFFFF => self.interrupt_handler.write(address, value), // interrupt enable (IE register)
            0xFF00...0xFF7F => unimplemented!(), // i/o ports
            0xFF80...0xFFFE => {
                let address = address - 0xFF80;
                self.high_ram[address as usize] = value;
            }, // high ram
            _ => {} // empty
        }
    }
}

impl Bus for Hardware {
    fn fetch_interrupt(&mut self) -> Option<Interrupt> {
        self.interrupt_handler.fetch_interrupt()
    }

    fn toggle_interrupts(&mut self, value: bool) {
        self.interrupt_handler.toggle_interrupts(value);
    }
}