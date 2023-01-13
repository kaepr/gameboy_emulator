use crate::{
    bus::Memory,
    cpu::CPU,
    utils::{reset_bit, set_bit, word_to_bytes, BitPosCheck},
};

pub struct Interrupts;

pub trait Interruptable {
    fn create_interrut_request(&mut self);
    fn reset_interrupt_request(&mut self);
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum InterruptType {
    VBLANK = 0,
    LCDSTAT = 1,
    TIMER = 2,
    SERIAL = 3,
    JOYPAD = 4,
}

impl Interrupts {
    const INTERRUPT_ENABLE_ADDRESS: u16 = 0xFFFF;
    const INTERRUPT_FLAG_ADDRESS: u16 = 0xFF0F;

    pub fn new() -> Self {
        Interrupts {}
    }

    pub fn pending_interrupt(cpu: &mut CPU) -> bool {
        let (ie_reg, ie_flag) = Self::get_interrupt_registers(cpu);
        ie_reg & ie_flag > 0
    }

    pub fn has_interrupt(cpu: &mut CPU) -> bool {
        cpu.ime & Self::pending_interrupt(cpu)
    }

    /// Checks the interrupt_request flags for all I/O
    /// and sets the addresses in interrutps registers
    pub fn process_interrupt_request(cpu: &mut CPU) {
        let (_ie_reg, mut ie_flag) = Self::get_interrupt_registers(cpu);

        if cpu.bus.timer.request_interrupt {
            ie_flag = set_bit(ie_flag, InterruptType::TIMER as usize);
            cpu.bus.timer.reset_interrupt_request();
        }

        cpu.bus.write(Self::INTERRUPT_FLAG_ADDRESS, ie_flag);
    }

    fn interrupt_type_addr(ie_reg: u8, ie_flag: u8) -> (InterruptType, u16) {
        if Self::check_interrupt_flag(ie_reg, ie_flag, InterruptType::VBLANK) {
            return (InterruptType::VBLANK, 0x0040);
        }

        if Self::check_interrupt_flag(ie_reg, ie_flag, InterruptType::LCDSTAT) {
            return (InterruptType::LCDSTAT, 0x0048);
        }

        if Self::check_interrupt_flag(ie_reg, ie_flag, InterruptType::TIMER) {
            return (InterruptType::TIMER, 0x0050);
        }

        if Self::check_interrupt_flag(ie_reg, ie_flag, InterruptType::SERIAL) {
            return (InterruptType::SERIAL, 0x0058);
        }

        if Self::check_interrupt_flag(ie_reg, ie_flag, InterruptType::JOYPAD) {
            return (InterruptType::JOYPAD, 0x0060);
        }

        panic!("there was no interrupt requested and enabled");
    }

    fn check_interrupt_flag(ie_reg: u8, ie_flag: u8, it_type: InterruptType) -> bool {
        ie_reg.is_bit_set(it_type as usize) && ie_flag.is_bit_set(it_type as usize)
    }

    pub fn handle_interrupt(cpu: &mut CPU) {
        cpu.tick();
        cpu.tick();

        let (ie_reg, ie_flag) = Self::get_interrupt_registers(cpu);
        let (pc_high, pc_low) = word_to_bytes(cpu.registers.pc);

        let (it_type, it_address) = Self::interrupt_type_addr(ie_reg, ie_flag);

        cpu.registers.sp = cpu.registers.sp.wrapping_sub(1);
        cpu.write_byte(cpu.registers.sp, pc_high);
        cpu.registers.sp = cpu.registers.sp.wrapping_sub(1);
        cpu.write_byte(cpu.registers.sp, pc_low);
        cpu.registers.pc = it_address;
        cpu.tick();

        let it_type_reset = reset_bit(ie_flag, it_type as usize);
        cpu.bus.write(0xFF0F, it_type_reset);
    }

    fn get_interrupt_registers(cpu: &CPU) -> (u8, u8) {
        let ie_reg = cpu.bus.read(Self::INTERRUPT_ENABLE_ADDRESS);
        let ie_flag = cpu.bus.read(Self::INTERRUPT_FLAG_ADDRESS);

        (ie_reg, ie_flag)
    }
}
