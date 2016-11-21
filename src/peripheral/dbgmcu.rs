# [ doc = "Debug support" ]
# [ repr ( C ) ]
pub struct Dbgmcu {
    # [ doc = "0x00 - MCU Device ID Code Register" ]
    pub idcode: Idcode,
    # [ doc = "0x04 - Debug MCU Configuration Register" ]
    pub cr: Cr,
    # [ doc = "0x08 - APB Low Freeze Register" ]
    pub apblfz: Apblfz,
    # [ doc = "0x0c - APB High Freeze Register" ]
    pub apbhfz: Apbhfz,
}

# [ repr ( C ) ]
pub struct Idcode {
    register: ::volatile_register::RO<u32>,
}

impl Idcode {
    pub fn read(&self) -> IdcodeR {
        IdcodeR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct IdcodeR {
    bits: u32,
}

impl IdcodeR {
    # [ doc = "Bits 0:11 - Device Identifier" ]
    pub fn dev_id(&self) -> u16 {
        const MASK: u32 = 4095;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 12:15 - Division Identifier" ]
    pub fn div_id(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:31 - Revision Identifier" ]
    pub fn rev_id(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct IdcodeW {
    bits: u32,
}

impl IdcodeW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        IdcodeW { bits: 0u32 }
    }
    # [ doc = "Bits 0:11 - Device Identifier" ]
    pub fn dev_id(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 4095;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 12:15 - Division Identifier" ]
    pub fn div_id(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:31 - Revision Identifier" ]
    pub fn rev_id(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Cr {
    register: ::volatile_register::RW<u32>,
}

impl Cr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&CrR, &'w mut CrW) -> &'w mut CrW
    {
        let bits = self.register.read();
        let r = CrR { bits: bits };
        let mut w = CrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> CrR {
        CrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut CrW) -> &mut CrW
    {
        let mut w = CrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CrR {
    bits: u32,
}

impl CrR {
    # [ doc = "Bit 1 - Debug Stop Mode" ]
    pub fn dbg_stop(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Debug Standby Mode" ]
    pub fn dbg_standby(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CrW {
    bits: u32,
}

impl CrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        CrW { bits: 0u32 }
    }
    # [ doc = "Bit 1 - Debug Stop Mode" ]
    pub fn dbg_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Debug Standby Mode" ]
    pub fn dbg_standby(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Apblfz {
    register: ::volatile_register::RW<u32>,
}

impl Apblfz {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&ApblfzR, &'w mut ApblfzW) -> &'w mut ApblfzW
    {
        let bits = self.register.read();
        let r = ApblfzR { bits: bits };
        let mut w = ApblfzW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> ApblfzR {
        ApblfzR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ApblfzW) -> &mut ApblfzW
    {
        let mut w = ApblfzW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ApblfzR {
    bits: u32,
}

impl ApblfzR {
    # [ doc = "Bit 0 - Debug Timer 2 stopped when Core is halted" ]
    pub fn dbg_timer2_stop(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Debug Timer 3 stopped when Core is halted" ]
    pub fn dbg_timer3_stop(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Debug Timer 6 stopped when Core is halted" ]
    pub fn dbg_timer6_stop(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Debug Timer 14 stopped when Core is halted" ]
    pub fn dbg_timer14_stop(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Debug RTC stopped when Core is halted" ]
    pub fn dbg_rtc_stop(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - Debug Window Wachdog stopped when Core is halted" ]
    pub fn dbg_wwdg_stop(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - Debug Independent Wachdog stopped when Core is halted" ]
    pub fn dbg_iwdg_stop(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 21 - SMBUS timeout mode stopped when Core is halted" ]
    pub fn i2c1_smbus_timeout(&self) -> bool {
        const OFFSET: u8 = 21u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ApblfzW {
    bits: u32,
}

impl ApblfzW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ApblfzW { bits: 0u32 }
    }
    # [ doc = "Bit 0 - Debug Timer 2 stopped when Core is halted" ]
    pub fn dbg_timer2_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Debug Timer 3 stopped when Core is halted" ]
    pub fn dbg_timer3_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Debug Timer 6 stopped when Core is halted" ]
    pub fn dbg_timer6_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Debug Timer 14 stopped when Core is halted" ]
    pub fn dbg_timer14_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Debug RTC stopped when Core is halted" ]
    pub fn dbg_rtc_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - Debug Window Wachdog stopped when Core is halted" ]
    pub fn dbg_wwdg_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - Debug Independent Wachdog stopped when Core is halted" ]
    pub fn dbg_iwdg_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 21 - SMBUS timeout mode stopped when Core is halted" ]
    pub fn i2c1_smbus_timeout(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Apbhfz {
    register: ::volatile_register::RW<u32>,
}

impl Apbhfz {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&ApbhfzR, &'w mut ApbhfzW) -> &'w mut ApbhfzW
    {
        let bits = self.register.read();
        let r = ApbhfzR { bits: bits };
        let mut w = ApbhfzW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> ApbhfzR {
        ApbhfzR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ApbhfzW) -> &mut ApbhfzW
    {
        let mut w = ApbhfzW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ApbhfzR {
    bits: u32,
}

impl ApbhfzR {
    # [ doc = "Bit 11 - Debug Timer 1 stopped when Core is halted" ]
    pub fn dbg_timer1_stop(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - Debug Timer 15 stopped when Core is halted" ]
    pub fn dbg_timer15_sto(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - Debug Timer 16 stopped when Core is halted" ]
    pub fn dbg_timer16_sto(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 18 - Debug Timer 17 stopped when Core is halted" ]
    pub fn dbg_timer17_sto(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ApbhfzW {
    bits: u32,
}

impl ApbhfzW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ApbhfzW { bits: 0u32 }
    }
    # [ doc = "Bit 11 - Debug Timer 1 stopped when Core is halted" ]
    pub fn dbg_timer1_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 16 - Debug Timer 15 stopped when Core is halted" ]
    pub fn dbg_timer15_sto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - Debug Timer 16 stopped when Core is halted" ]
    pub fn dbg_timer16_sto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 18 - Debug Timer 17 stopped when Core is halted" ]
    pub fn dbg_timer17_sto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}
