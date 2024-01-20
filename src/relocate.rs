use crate::{mnemonic::Mnemonic, instructions::{Instr, Opr}};

pub fn relocate(instr_list: &mut Vec<Instr>) {
    todo!();
    // let mut bytes_sum = 0;
    // let mut set = Vec::<(String, usize, usize, Instr)>::new();
    // for (index, item) in self.instructs.iter_mut().enumerate() {
    //     if item.instr.mnem == Mnemonic::Lable {
    //         let Oprs::One(Opr::Rel(key)) = item.instr.oprs.clone() else {
    //             unreachable!();
    //         };
    //         self.rel_map.entry(key).and_modify(|x| *x = Some(bytes_sum));
    //         continue;
    //     }
    //     if item.relocatable == Relocatable::Loc {
    //         let Oprs::One(Opr::Rel(key)) = item.instr.oprs.clone() else {
    //             unreachable!();
    //         };
    //         let Some(Some(target)) = self.rel_map.get(&key) else {
    //             panic!("Unknown Target!");
    //         };
    //         let loc: i32 = *target as i32 - bytes_sum as i32;
    //         let new_bytes = match item.instr.mnem {
    //             Mnemonic::Call => {
    //                 assemble_instr(&Instr::new1(item.instr.mnem, Opr::Imm32(loc as i64)))
    //             }
    //             _ => assemble_instr(&Instr::new1(item.instr.mnem, loc)),
    //         };
    //         bytes_sum += new_bytes.len();
    //         set.push((key, index, bytes_sum, item.instr.clone()));
    //         // item.bytes = new_bytes;
    //     } else {
    //         bytes_sum += item.bytes.len();
    //     }
    // }
    // for item in set.iter() {
    //     let Some(Some(target)) = self.rel_map.get(&item.0) else {
    //         panic!("Unknown Target!");
    //     };
    //     let loc: i32 = *target as i32 - item.2 as i32;
    //     let new_bytes = match item.3.mnem {
    //         Mnemonic::Call => assemble_instr(&Instr::new1(item.3.mnem, Opr::Imm32(loc as i64))),
    //         _ => assemble_instr(&Instr::new1(item.3.mnem, loc)),
    //     };
    //     self.instructs[item.1].bytes = new_bytes;
    // }
}

pub fn text_section_bytes(instr_list: &mut Vec<Instr>) -> Vec<u8> {
    relocate(instr_list);
    let mut bytes = Vec::new();
    let mut bc = 0;
    for item in instr_list.iter() {
        todo!();
        // println!(
        //     "\x1b[92m{bc:3X}\x1b[0m: {:02X?} \x1b[93m{}\x1b[0m",
        //     item.bytes, item.instr
        //     );
        // bc += item.bytes.len();
        // bytes.extend(item.bytes.clone());
    }
    bytes
}

pub fn instr2(mnemonic: Mnemonic, opr1: impl Into<Opr>, opr2: impl Into<Opr>) -> Instr {
    Instr::new2(mnemonic, opr1, opr2)
}

pub fn instr1(mnemonic: Mnemonic, opr1: impl Into<Opr>) -> Instr{
    Instr::new1(mnemonic, opr1)
}

pub fn instr0(mnemonic: Mnemonic) -> Instr{
    Instr::new0(mnemonic)
}
