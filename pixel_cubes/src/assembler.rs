// inspiration from Viola :)

/* System parameters. */
const INSTRUCTION_LENGTH: usize = 8;

/* Operation codes. */
const ADDSUB_OP: OperationCode = 0b000;
const ADDI_OP: OperationCode   = 0b001;
const JMP_OP: OperationCode    = 0b010;
const JEQ_OP: OperationCode    = 0b011;
const PRNT_OP: OperationCode   = 0b100;
const INPUT_OP: OperationCode  = 0b101;
const EXIT_OP: OperationCode   = 0b110;
// one more

/* Registry addresses. */
const R0: Registry = 0b00; // always zero
const R1: Registry = 0b01;
const R2: Registry = 0b10;
const R3: Registry = 0b11;

fn parse_row(row: u8) -> Result<(InstructionType, OperationCode), String> {
    let op: usize = row >> 5;
    println!("op: {:b}", op);
    match op {
        ADDSUB_OP => Ok(InstructionType::Arithmetic, ADDSUB_OP),
        ADDI_OP   => Ok(InstructionType::Arithmetic, ADDI_OP),
        JMP_OP    => Ok(InstructionType::Jump, JMP_OP),
        JEQ_OP    => Ok(InstructionType::Jump, JEQ_OP),
        PRNT_OP   => Ok(InstructionType::)
    }
}

