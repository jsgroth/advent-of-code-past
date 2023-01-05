#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum ChronoOperation {
    AddRegister,
    AddImmediate,
    MultiplyRegister,
    MultiplyImmediate,
    AndRegister,
    AndImmediate,
    OrRegister,
    OrImmediate,
    SetRegister,
    SetImmediate,
    GreaterThanImmediateRegister,
    GreaterThanRegisterImmediate,
    GreaterThanRegisterRegister,
    EqualImmediateRegister,
    EqualRegisterImmediate,
    EqualRegisterRegister,
}

impl ChronoOperation {
    pub const ALL: [Self; 16] = [
        Self::AddRegister,
        Self::AddImmediate,
        Self::MultiplyRegister,
        Self::MultiplyImmediate,
        Self::AndRegister,
        Self::AndImmediate,
        Self::OrRegister,
        Self::OrImmediate,
        Self::SetRegister,
        Self::SetImmediate,
        Self::GreaterThanImmediateRegister,
        Self::GreaterThanRegisterImmediate,
        Self::GreaterThanRegisterRegister,
        Self::EqualImmediateRegister,
        Self::EqualRegisterImmediate,
        Self::EqualRegisterRegister,
    ];

    pub fn execute(&self, registers: &[u32; 4], a: u32, b: u32) -> u32 {
        let a_us = a as usize;
        let b_us = b as usize;
        match self {
            Self::AddRegister => registers[a_us] + registers[b_us],
            Self::AddImmediate => registers[a_us] + b,
            Self::MultiplyRegister => registers[a_us] * registers[b_us],
            Self::MultiplyImmediate => registers[a_us] * b,
            Self::AndRegister => registers[a_us] & registers[b_us],
            Self::AndImmediate => registers[a_us] & b,
            Self::OrRegister => registers[a_us] | registers[b_us],
            Self::OrImmediate => registers[a_us] | b,
            Self::SetRegister => registers[a_us],
            Self::SetImmediate => a,
            Self::GreaterThanImmediateRegister => if a > registers[b_us] { 1 } else { 0 },
            Self::GreaterThanRegisterImmediate => if registers[a_us] > b { 1 } else { 0 },
            Self::GreaterThanRegisterRegister => if registers[a_us] > registers[b_us] { 1 } else { 0 },
            Self::EqualImmediateRegister => if a == registers[b_us] { 1 } else { 0 },
            Self::EqualRegisterImmediate => if registers[a_us] == b { 1 } else { 0 },
            Self::EqualRegisterRegister => if registers[a_us] == registers[b_us] { 1 } else { 0 },
        }
    }

    pub fn can_produce(&self, before: &[u32; 4], after: &[u32; 4], a: u32, b: u32, c: usize) -> bool {
        after[c] == self.execute(before, a, b)
    }
}