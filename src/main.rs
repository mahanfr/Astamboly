use std::{fs::File, io::Write, error::Error};


struct Section {
    name: String,
    stype: u32,
    flags: u64,
    addr: u64,
    offset: u64,
    link: u32,
    info: u32,
    align: u64,
    ent_size: u64,
    data: Vec<u8>,
}

struct Elf64 {
    file_path: String,
    etype: u16,
    entry: u64,
    phoff: u64,
    shoff: u64,
    sections: Vec<Section>,
}
impl Elf64 {
    pub fn new(file_path: String) -> Self {
        Self {
            file_path,
            etype: 0x01,
            entry: 0,
            phoff: 0,
            shoff: 64,
            sections: Vec::new(),
        }
    }

    fn write_header(&self,file: &mut File) -> Result<(),Box <dyn Error>> {
        let e_ident: [u8;16] = [0x7f,0x45,0x4c,0x46,0x02,0x01,0x01,0,0,0,0,0,0,0,0,0];
        // idntifier for elf file
        file.write_all(&e_ident.to_vec())?;
        // type of the file
        file.write_all(&self.etype.to_le_bytes())?;
        // platform of instructions
        file.write_all(&[0x3E,0x00])?;
        // current version
        file.write_all(&1u32.to_le_bytes())?;
        // address of entry
        file.write_all(&self.entry.to_le_bytes())?;
        // program headers offset
        file.write_all(&self.phoff.to_le_bytes())?;
        // section headers offset
        file.write_all(&self.shoff.to_le_bytes())?;
        // Flags
        file.write_all(&0u32.to_le_bytes())?;
        // Header size
        file.write_all(&64u16.to_le_bytes())?;
        // size of program header
        file.write_all(&0u16.to_le_bytes())?;
        // number of program headers
        file.write_all(&0u16.to_le_bytes())?;
        // size of section header
        file.write_all(&64u16.to_le_bytes())?;
        // number of sections
        file.write_all(&(self.sections.len() as u16 + 4u16).to_le_bytes())?;
        // index of section that holds names
        file.write_all(&(self.sections.len() as u16 + 1u16).to_le_bytes())?;
        Ok(())
    }

    pub fn flush(&self) -> Result<(),Box <dyn Error>>{
        let mut file = File::create(self.file_path.as_str())?;
        self.write_header(&mut file)?;
        Ok(())
    }
}


fn main() {
    let elf_file = Elf64::new("./astamboly".to_string());
    elf_file.flush();
    println!("Hello, world!");
}
