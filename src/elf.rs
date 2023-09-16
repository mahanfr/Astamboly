use std::{fs::File, io::Write, error::Error};

struct Section {
    name: String,
    stype: u32,
    flags: u64,
    addr: u64,
    offset: u64,
    size: u64,
    link: u32,
    info: u32,
    align: u64,
    ent_size: u64,
    data: Vec<u8>,
}

impl Default for Section {
    fn default() -> Self {
        Self {
            name: String::new(),
            stype: 0,
            flags: 0,
            addr: 0,
            offset: 0,
            size: 0,
            link: 0,
            info: 0,
            align: 0,
            ent_size: 0,
            data: Vec::new(),
        }
    }
}

struct Elf64 {
    file_path: String,
    etype: u16,
    entry: u64,
    phoff: u64,
    shoff: u64,
    sections: Vec<Section>,
    shstr_table: Vec<String>,
}
impl Elf64 {
    pub fn new(file_path: String) -> Self {
        let shstr_table = vec! [
            ".shstrtab".to_string(),
            ".symtab".to_string(),
            ".strtab".to_string(),
        ];
        Self {
            file_path,
            etype: 0x01,
            entry: 0,
            phoff: 0,
            shoff: 64,
            sections: Vec::new(),
            shstr_table,
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
    
    fn get_header_bytes(&self,sec: &Section) -> Vec<u8> {
        let mut bytes = Vec::new();
        if !sec.name.is_empty() {
            //TODO: Set name tables return index
        } else {
            bytes.append(&mut 0u32.to_le_bytes().to_vec());
        }
        bytes.append(&mut sec.stype.to_le_bytes().to_vec());
        bytes.append(&mut sec.flags.to_le_bytes().to_vec());
        bytes.append(&mut sec.addr.to_le_bytes().to_vec());
        bytes.append(&mut sec.offset.to_le_bytes().to_vec());
        bytes.append(&mut sec.data.len().to_le_bytes().to_vec());
        bytes.append(&mut sec.link.to_le_bytes().to_vec());
        bytes.append(&mut sec.info.to_le_bytes().to_vec());
        bytes.append(&mut sec.align.to_le_bytes().to_vec());
        bytes.append(&mut sec.ent_size.to_le_bytes().to_vec());

        bytes
    }

    pub fn write_section_data(&self, file: &mut File) -> Result<(),Box<dyn Error>> {
        for sec in self.sections.iter() {
            file.write_all(sec.data.as_slice())?;
        }
        Ok(())
    }
    
    pub fn write_section_header_bytes(&self,file: &mut File) -> Result<(),Box<dyn Error>> {
        // Empty Section Header
        file.write_all(self.get_header_bytes(&Section::default()).as_slice())?;
        for sec in self.sections.iter() {
            file.write_all(&self.get_header_bytes(sec))?;
        }
        Ok(())
    }

    pub fn add_symtab_section(&mut self) -> Result<(), Box<dyn Error>> {
        let data_offset : u64 = self.sections.iter().map(|sec| sec.data.len() as u64).sum();
        let data = Vec::<u8>::new();
        let sec = Section {
            name: ".symtab".to_string(),
            stype: 0x02,
            offset: 64 + (64 * data_offset),
            link: 4,
            info: 3,
            size: data.len() as u64,
            align: 8,
            ..Default::default()
        };
        self.sections.push(sec);
        Ok(())
    }

    pub fn add_shstrtab_section(&mut self) -> usize {
        let sec = Section {
            name: ".shstrtab".to_string(),
            stype: 0x03,
            align: 1,
            ..Default::default()
        };
        self.sections.push(sec);
        self.sections.len() - 1
    }

    pub fn add_strtab_section(&mut self) -> Result<(), Box<dyn Error>> {
        let sec = Section {
            name: ".strtab".to_string(),
            stype: 0x03,
            align: 1,
            ..Default::default()
        };
        self.sections.push(sec);
        Ok(())
    }
    
    fn set_shstrtab_data(&mut self, index: usize) {
        let mut shstrtab_data = Vec::<u8>::new();
        for sec in self.sections.iter() {
            shstrtab_data.push('\0' as u8);
            shstrtab_data.append(&mut sec.name.as_bytes().to_vec());
        }
        shstrtab_data.push('\0' as u8);
        self.sections[index].size = shstrtab_data.len() as u64;
        let frame = 2 << shstrtab_data.len().ilog2() as usize;
        shstrtab_data.resize(frame,'\0' as u8);
    }

    pub fn flush(&mut self) -> Result<(),Box <dyn Error>>{
        let mut file = File::create(self.file_path.as_str())?;
        self.write_header(&mut file)?;
        let shstrtab_index = self.add_shstrtab_section();
        self.add_symtab_section()?;
        self.add_strtab_section()?;

        // Add Section Data
        self.write_section_header_bytes(&mut file)?;
        self.write_section_data(&mut file)?;
        Ok(())
    }
}

