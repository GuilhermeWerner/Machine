pub const NOP: u8 = 0x00; // nop
pub const LDR_B: u8 = 0x01; // ldr b r1 r2
pub const LDR_H: u8 = 0x02; // ldr h r1 r2
pub const LDR_W: u8 = 0x03; // ldr w r1 r2
pub const LDR_D: u8 = 0x04; // ldr d r1 r2
pub const LDR_Q: u8 = 0x05; // ldr q r1 r2 r3
pub const LDA_B: u8 = 0x06; // lda b r1 0x0000000000000000
pub const LDA_H: u8 = 0x07; // lda h r1 0x0000000000000000
pub const LDA_W: u8 = 0x08; // lda w r1 0x0000000000000000
pub const LDA_D: u8 = 0x09; // lda d r1 0x0000000000000000
pub const LDA_Q: u8 = 0x0A; // lda q r1 r2 0x0000000000000000
pub const LDI_B: u8 = 0x0B; // ldi b r1 0x00
pub const LDI_H: u8 = 0x0C; // ldi h r1 0x0000
pub const LDI_W: u8 = 0x0D; // ldi w r1 0x00000000
pub const LDI_D: u8 = 0x0E; // ldi d r1 0x0000000000000000
pub const LDI_Q: u8 = 0x0F; // ldi q r1 r2 0x00000000000000000000000000000000
pub const STR_B: u8 = 0x10; // str b r1 r2
pub const STR_H: u8 = 0x11; // str h r1 r2
pub const STR_W: u8 = 0x12; // str w r1 r2
pub const STR_D: u8 = 0x13; // str d r1 r2
pub const STR_Q: u8 = 0x14; // str q r1 r2 r3
pub const STA_B: u8 = 0x15; // sta b 0x0000000000000000 r1
pub const STA_H: u8 = 0x16; // sta h 0x0000000000000000 r1
pub const STA_W: u8 = 0x17; // sta w 0x0000000000000000 r1
pub const STA_D: u8 = 0x18; // sta d 0x0000000000000000 r1
pub const STA_Q: u8 = 0x19; // sta q 0x0000000000000000 r1 r2
pub const STI_B: u8 = 0x1A; // sti b
pub const STI_H: u8 = 0x1B; // sti h
pub const STI_W: u8 = 0x1C; // sti w
pub const STI_D: u8 = 0x1D; // sti d
pub const STI_Q: u8 = 0x1E; // sti q
pub const INC: u8 = 0x1F; // inc r1
pub const DEC: u8 = 0x20; // dec r1
pub const MOV: u8 = 0x21; // mov r1 r2
pub const SWP: u8 = 0x22; // swp r1 r2
pub const JMP: u8 = 0x23; // jmp 0x00000000
pub const JMZ: u8 = 0x24; // jmz 0x00000000 r1 r2
pub const JEQ: u8 = 0x25; // jeq 0x00000000 r1 r2
pub const JNE: u8 = 0x26; // jne 0x00000000 r1 r2
pub const JLT: u8 = 0x27; // jlt 0x00000000 r1 r2
pub const JLE: u8 = 0x28; // jle 0x00000000 r1 r2
pub const JGT: u8 = 0x29; // jgt 0x00000000 r1 r2
pub const JGE: u8 = 0x2A; // jge 0x00000000 r1 r2
pub const CALL: u8 = 0x2B; // call 0x00000000
pub const RET: u8 = 0x2C; // ret
pub const PUSH_B: u8 = 0x2D; // push b r1
pub const PUSH_H: u8 = 0x2E; // push h r1
pub const PUSH_W: u8 = 0x2F; // push w r1
pub const PUSH_D: u8 = 0x30; // push d r1
pub const PUSH_Q: u8 = 0x31; // push q r1 r2
pub const PEEK_B: u8 = 0x32; // peek b r1
pub const PEEK_H: u8 = 0x33; // peek h r1
pub const PEEK_W: u8 = 0x34; // peek w r1
pub const PEEK_D: u8 = 0x35; // peek d r1
pub const PEEK_Q: u8 = 0x36; // peek q r1 r2
pub const POP_B: u8 = 0x37; // pop b r1
pub const POP_H: u8 = 0x38; // pop h r1
pub const POP_W: u8 = 0x39; // pop w r1
pub const POP_D: u8 = 0x3A; // pop d r1
pub const POP_Q: u8 = 0x3B; // pop q r1 r2
pub const NEG: u8 = 0x3C; // neg r1 r2
pub const ADD_U8: u8 = 0x3D; // add u8 r1 r2 r3
pub const ADD_U16: u8 = 0x3E; // add u16 r1 r2 r3
pub const ADD_U32: u8 = 0x3F; // add u32 r1 r2 r3
pub const ADD_U64: u8 = 0x40; // add u64 r1 r2 r3
pub const ADD_U128: u8 = 0x41; // add u128 r1 r2 r3 r4 r5 r6
pub const ADD_I8: u8 = 0x42; // add i8 r1 r2 r3
pub const ADD_I16: u8 = 0x43; // add i16 r1 r2 r3
pub const ADD_I32: u8 = 0x44; // add i32 r1 r2 r3
pub const ADD_I64: u8 = 0x45; // add i64 r1 r2 r3
pub const ADD_I128: u8 = 0x46; // add i128 r1 r2 r3 r4 r5 r6
pub const ADD_F32: u8 = 0x47; // add f32 r1 r2 r3
pub const ADD_F64: u8 = 0x48; // add f64 r1 r2 r3
pub const SUB_U8: u8 = 0x49; // sub u8 r1 r2 r3
pub const SUB_U16: u8 = 0x4A; // sub u16 r1 r2 r3
pub const SUB_U32: u8 = 0x4B; // sub u32 r1 r2 r3
pub const SUB_U64: u8 = 0x4C; // sub u64 r1 r2 r3
pub const SUB_U128: u8 = 0x4D; // sub u128 r1 r2 r3 r4 r5 r6
pub const SUB_I8: u8 = 0x4E; // sub i8 r1 r2 r3
pub const SUB_I16: u8 = 0x4F; // sub i16 r1 r2 r3
pub const SUB_I32: u8 = 0x50; // sub i32 r1 r2 r3
pub const SUB_I64: u8 = 0x51; // sub i64 r1 r2 r3
pub const SUB_I128: u8 = 0x52; // sub i128 r1 r2 r3 r4 r5 r6
pub const SUB_F32: u8 = 0x53; // sub f32 r1 r2 r3
pub const SUB_F64: u8 = 0x54; // sub f64 r1 r2 r3
pub const MUL_U8: u8 = 0x55; // mul u8 r1 r2 r3
pub const MUL_U16: u8 = 0x56; // mul u16 r1 r2 r3
pub const MUL_U32: u8 = 0x57; // mul u32 r1 r2 r3
pub const MUL_U64: u8 = 0x58; // mul u64 r1 r2 r3
pub const MUL_U128: u8 = 0x59; // mul u128 r1 r2 r3 r4 r5 r6
pub const MUL_I8: u8 = 0x5A; // mul i8 r1 r2 r3
pub const MUL_I16: u8 = 0x5B; // mul i16 r1 r2 r3
pub const MUL_I32: u8 = 0x5C; // mul i32 r1 r2 r3
pub const MUL_I64: u8 = 0x5D; // mul i64 r1 r2 r3
pub const MUL_I128: u8 = 0x5E; // mul i128 r1 r2 r3 r4 r5 r6
pub const MUL_F32: u8 = 0x5F; // mul f32 r1 r2 r3
pub const MUL_F64: u8 = 0x60; // mul f64 r1 r2 r3
pub const DIV_U8: u8 = 0x61; // div u8 r1 r2 r3
pub const DIV_U16: u8 = 0x62; // div u16 r1 r2 r3
pub const DIV_U32: u8 = 0x63; // div u32 r1 r2 r3
pub const DIV_U64: u8 = 0x64; // div u64 r1 r2 r3
pub const DIV_U128: u8 = 0x65; // div u128 r1 r2 r3 r4 r5 r6
pub const DIV_I8: u8 = 0x66; // div i8 r1 r2 r3
pub const DIV_I16: u8 = 0x67; // div i16 r1 r2 r3
pub const DIV_I32: u8 = 0x68; // div i32 r1 r2 r3
pub const DIV_I64: u8 = 0x69; // div i64 r1 r2 r3
pub const DIV_I128: u8 = 0x6A; // div i128 r1 r2 r3 r4 r5 r6
pub const DIV_F32: u8 = 0x6B; // div f32 r1 r2 r3
pub const DIV_F64: u8 = 0x6C; // div f64 r1 r2 r3
pub const REM_U8: u8 = 0x6D; // rem u8 r1 r2 r3
pub const REM_U16: u8 = 0x6E; // rem u16 r1 r2 r3
pub const REM_U32: u8 = 0x6F; // rem u32 r1 r2 r3
pub const REM_U64: u8 = 0x70; // rem u64 r1 r2 r3
pub const REM_U128: u8 = 0x71; // rem u128 r1 r2 r3 r4 r5 r6
pub const REM_I8: u8 = 0x72; // rem i8 r1 r2 r3
pub const REM_I16: u8 = 0x73; // rem i16 r1 r2 r3
pub const REM_I32: u8 = 0x74; // rem i32 r1 r2 r3
pub const REM_I64: u8 = 0x75; // rem i64 r1 r2 r3
pub const REM_I128: u8 = 0x76; // rem i128 r1 r2 r3 r4 r5 r6
pub const REM_F32: u8 = 0x77; // rem f32 r1 r2 r3
pub const REM_F64: u8 = 0x78; // rem f64 r1 r2 r3
pub const AND: u8 = 0x79; // and r1 r2 r3
pub const OR: u8 = 0x7A; // or r1 r2 r3
pub const XOR: u8 = 0x7B; // xor r1 r2 r3
pub const NOT: u8 = 0x7C; // not r1 r2
pub const NAND: u8 = 0x7D; // nand r1 r2 r3
pub const NOR: u8 = 0x7E; // nor r1 r2 r3
pub const XNOR: u8 = 0x7F; // xnor r1 r2 r3
pub const SHL: u8 = 0x80; // shl r1 r2
pub const SHR: u8 = 0x81; // shr r1 r2
pub const EQ: u8 = 0x82; // eq r1 r2 r3
pub const NE: u8 = 0x83; // ne r1 r2 r3
pub const LT: u8 = 0x84; // lt r1 r2 r3
pub const LE: u8 = 0x85; // le r1 r2 r3
pub const GT: u8 = 0x86; // gt r1 r2 r3
pub const GE: u8 = 0x87; // ge r1 r2 r3
pub const HLT: u8 = 0xFF; // hlt
