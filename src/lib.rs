// TODO: create flags type: for easy decoding, encoding of flags
// TODO: create separate memory type: for safe, convinient access of memory
use std::fmt;

enum Instruction {
    NOP,
    CLS,
    VBLNK,
    BGC,
    SPR,
    DRW0,
    DRW1,
    RND,
    FLIP0,
    FLIP1,
    FLIP2,
    FLIP3,
    SND0,
    SND1,
    SND2,
    SND3,
    SNP,
    SNG,
    JMP0,
    JMC,
    JX,
    JME,
    CALL0,
    RET,
    JMP1,
    CX,
    CALL1,
    LDI0,
    LDI1,
    LDM0,
    LDM1,
    MOV,
    STM0,
    STM1,
    ADDI,
    ADD0,
    ADD1,
    SUBI,
    SUB0,
    SUB1,
    CMPI,
    CMP,
    ANDI,
    AND0,
    AND1,
    TSTI,
    TST,
    ORI,
    OR0,
    OR1,
    XORI,
    XOR0,
    XOR1,
    MULI,
    MUL0,
    MUL1,
    DIVI,
    DIV0,
    DIV1,
    MODI,
    MOD0,
    MOD1,
    REMI,
    REM0,
    REM1,
    SHL0,
    SHR0,
    SAL0,
    SAR0,
    SHL1,
    SHR1,
    SAL1,
    SAR1,
    PUSH,
    POP,
    PUSHALL,
    POPALL,
    PUSHF,
    POPF,
    PAL0,
    PAL1,
    NOTI,
    NOT0,
    NOT1,
    NEGI,
    NEG0,
    NEG1,
}

impl Instruction {
    fn decode(opcode: u32) -> Result<Instruction, &'static str> {
        use Instruction::*;

        let b3 = (opcode & 0xFF000000 >> 24) as u8;

        match b3 {
            0x00 => Ok(NOP),
            0x01 => Ok(CLS),
            0x02 => Ok(VBLNK),
            0x03 => Ok(BGC),
            0x04 => Ok(SPR),
            0x05 => Ok(DRW0),
            0x06 => Ok(DRW1),
            0x07 => Ok(RND),
            // 0x08 => Ok(FLIP0),
            0x09 => Ok(SND0),
            0x0A => Ok(SND1),
            0x0B => Ok(SND2),
            0x0C => Ok(SND3),
            0x0D => Ok(SNP),
            0x0E => Ok(SNG),

            0x10 => Ok(JMP0),
            0x11 => Ok(JMC),
            0x12 => Ok(JX),
            0x13 => Ok(JME),
            0x14 => Ok(CALL0),
            0x15 => Ok(RET),
            0x16 => Ok(JMP1),
            0x17 => Ok(CX),
            0x18 => Ok(CALL1),

            0x20 => Ok(LDI0),
            0x21 => Ok(LDI1),
            0x22 => Ok(LDM0),
            0x23 => Ok(LDM1),
            0x24 => Ok(MOV),

            0x30 => Ok(STM0),
            0x31 => Ok(STM1),

            0x40 => Ok(ADDI),
            0x41 => Ok(ADD0),
            0x42 => Ok(ADD1),

            0x50 => Ok(SUBI),
            0x51 => Ok(SUB0),
            0x52 => Ok(SUB1),
            0x53 => Ok(CMPI),
            0x54 => Ok(CMP),

            0x60 => Ok(ANDI),
            0x61 => Ok(AND0),
            0x62 => Ok(AND1),
            0x63 => Ok(TSTI),
            0x64 => Ok(TST),

            0x70 => Ok(ORI),
            0x71 => Ok(OR0),
            0x72 => Ok(OR1),

            0x80 => Ok(XORI),
            0x81 => Ok(XOR0),
            0x82 => Ok(XOR1),

            0x90 => Ok(MULI),
            0x91 => Ok(MUL0),
            0x92 => Ok(MUL1),

            0xA0 => Ok(DIVI),
            0xA1 => Ok(DIV0),
            0xA2 => Ok(DIV1),
            0xA3 => Ok(MODI),
            0xA4 => Ok(MOD0),
            0xA5 => Ok(MOD1),
            0xA6 => Ok(REMI),
            0xA7 => Ok(REM0),
            0xA8 => Ok(REM1),

            0xB0 => Ok(SHL0),
            0xB1 => Ok(SHR0),
            // 0xB0 => Ok(SAL0),
            0xB2 => Ok(SAR0),
            0xB3 => Ok(SHL1),
            0xB4 => Ok(SHR1),
            // 0xB3 => Ok(SHL1),
            0xB5 => Ok(SAR1),

            0xC0 => Ok(PUSH),
            0xC1 => Ok(POP),
            0xC2 => Ok(PUSHALL),
            0xC3 => Ok(POPALL),
            0xC4 => Ok(PUSHF),
            0xC5 => Ok(POPF),

            0xD0 => Ok(PAL0),
            0xD1 => Ok(PAL1),

            0xE0 => Ok(NOTI),
            0xE1 => Ok(NOT0),
            0xE2 => Ok(NOT1),
            0xE3 => Ok(NEGI),
            0xE4 => Ok(NEG0),
            0xE5 => Ok(NEG1),

            _ => Err(""),
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

#[derive(Default)]
struct Cpu {
    m: Vec<u8>,

    r: [u16; 16],
    pc: u16,
    sp: u16,
    f: u8,

    bg: u8,
    spritew: u8,
    spriteh: u8,
    hflip: bool,
    vlip: bool,
}

// fetch-decode-execute

impl Cpu {
    fn new() -> Cpu {
        Cpu {
            ..Default::default()
        }
    }

    fn step(&self) {
        let opcode = 0 as u32;
        let instruction = Instruction::decode(opcode);

        // Decompose the instruction into bytes.
        let (b3, b2, b1, b0) = (
            (opcode & 0xFF000000 >> 0x18) as u8,
            (opcode & 0x00FF0000 >> 0x10) as u8,
            (opcode & 0x0000FF00 >> 0x08) as u8,
            (opcode & 0x000000FF) as u8,
        );

        // Decompose the instruction into nibbles.
        let (n7, n6, n5, n4, n3, n2, n1, n0) = (
            b0 & 0x0F,
            b0 & 0xF0,
            b1 & 0x0F,
            b1 & 0xF0,
            b2 & 0x0F,
            b2 & 0xF0,
            b3 & 0x0F,
            b3 & 0xF0,
        );

        // Decode the instruction.
        match instruction {
            NOP => {}
            _ => {}
        };
    }

    fn execute(&self, instruction: &Instruction) {}
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_cpu() {
        let cpu = Cpu::new();
        cpu.step();
    }
}
