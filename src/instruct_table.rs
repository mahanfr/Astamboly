#[repr(u8)]
pub enum Regsiter {
    A = 0,
    C,
    D,
    B,
    SP,
    BP,
    SI,
    DI,
}

pub fn get_instruct_bytes(instruct: &str) -> u32 {
    match instruct {
        "mov" => 0xb8,
        "syscall" => 0x0f05,
        _ => unimplemented!(),
    }
}

pub fn mov(reg1: Regsiter, imm: u32) -> Vec<u8> {
    let mut mov_bytes = get_instruct_bytes("mov");
    mov_bytes += reg1 as u32;
    let mut res = Vec::new();
    if mov_bytes < u8::MAX as u32 {
        res.append(&mut (mov_bytes as u8).to_le_bytes().into());
    } else if mov_bytes < u16::MAX as u32 {
        res.append(&mut (mov_bytes as u16).to_le_bytes().into());
    } else {
        res.append(&mut mov_bytes.to_le_bytes().into());
    }
    res.append(&mut imm.to_le_bytes().into());
    res
}

pub fn syscall() -> Vec<u8> {
    get_instruct_bytes("syscall").to_le_bytes().into()
}
