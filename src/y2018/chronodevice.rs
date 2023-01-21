use crate::SimpleError;

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

    pub fn from_str(s: &str) -> Result<Self, SimpleError> {
        let op = match s {
            "addr" => Self::AddRegister,
            "addi" => Self::AddImmediate,
            "mulr" => Self::MultiplyRegister,
            "muli" => Self::MultiplyImmediate,
            "banr" => Self::AndRegister,
            "bani" => Self::AndImmediate,
            "borr" => Self::OrRegister,
            "bori" => Self::OrImmediate,
            "setr" => Self::SetRegister,
            "seti" => Self::SetImmediate,
            "gtir" => Self::GreaterThanImmediateRegister,
            "gtri" => Self::GreaterThanRegisterImmediate,
            "gtrr" => Self::GreaterThanRegisterRegister,
            "eqir" => Self::EqualImmediateRegister,
            "eqri" => Self::EqualRegisterImmediate,
            "eqrr" => Self::EqualRegisterRegister,
            _ => return Err(SimpleError::new(format!("invalid chrono device operation: {s}")))
        };

        Ok(op)
    }

    pub fn execute(&self, registers: &[u64], a: u64, b: u64) -> u64 {
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
            Self::GreaterThanImmediateRegister => u64::from(a > registers[b_us]),
            Self::GreaterThanRegisterImmediate => u64::from(registers[a_us] > b),
            Self::GreaterThanRegisterRegister => u64::from(registers[a_us] > registers[b_us]),
            Self::EqualImmediateRegister => u64::from(a == registers[b_us]),
            Self::EqualRegisterImmediate => u64::from(registers[a_us] == b),
            Self::EqualRegisterRegister => u64::from(registers[a_us] == registers[b_us]),
        }
    }

    pub fn can_produce(&self, before: &[u64], after: &[u64], a: u64, b: u64, c: usize) -> bool {
        after[c] == self.execute(before, a, b)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ChronoInstruction {
    pub op: ChronoOperation,
    pub a: u64,
    pub b: u64,
    pub c: usize,
}

impl ChronoInstruction {
    pub fn from_line(line: &str) -> Result<Self, SimpleError> {
        let split: Vec<_> = line.split(' ').collect();
        if split.len() != 4 {
            return Err(SimpleError::new(format!("invalid line format, expected 3 spaces: {line}")));
        }

        let op = ChronoOperation::from_str(split[0])?;
        let a = split[1].parse()?;
        let b = split[2].parse()?;
        let c = split[3].parse()?;

        Ok(ChronoInstruction { op, a, b, c, })
    }
}
