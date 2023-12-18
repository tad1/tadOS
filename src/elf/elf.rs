use embedded_sdmmc::{File, VolumeManager};

use crate::{fs::TestClock, bsp::EMMCController, info};



#[repr(C)]
struct ELF64Header {
    e_ident: [u8; 16],
    e_type: u16,
    e_machine: u16,
    e_version: u32,
    e_entry: u64,
    e_phoff: u64,
    e_shoff: u64,
    e_flags: u32,
    e_ehsize: u16,
    e_phentsize: u16,
    e_phnum: u16,
    e_shentsize: u16,
    e_shnum: u16,
    e_shstrndx: u16
}

#[repr(C)]
struct ELF64ProgramHeader {
    p_type: u32,
    p_flags: u32,
    p_offset: u64,
    p_vaddr: u64,
    p_paddr: u64,
    p_filesz: u64,
    p_memsz: u64,
    p_align: u64
}


pub unsafe fn load_elf(vol_mgr: &mut embedded_sdmmc::VolumeManager<&EMMCController, TestClock>, file: File) -> Option<fn()>{
    // TODO: remove responsiblity of reading headers
    const HEADER_SIZE: usize = core::mem::size_of::<ELF64Header>();
    let mut buffer: [u8; HEADER_SIZE] = [0; HEADER_SIZE];

    let _ = vol_mgr.file_seek_from_start(file, 0);
    let _ = vol_mgr.read(file, &mut buffer);

    let header: ELF64Header = unsafe { core::mem::transmute(buffer) };

    if !header.e_ident.starts_with(&[0x7F, 0x45, 0x4c, 0x46]) {
        return None;
    }

    let program_header_offset = header.e_phoff;
    
    
    const PROGRAM_HEADER_SIZE: usize = core::mem::size_of::<ELF64ProgramHeader>();
    let mut buffer: [u8; PROGRAM_HEADER_SIZE] = [0; PROGRAM_HEADER_SIZE];
    let _ = vol_mgr.file_seek_from_start(file, program_header_offset as u32);
    let _ = vol_mgr.read(file, &mut buffer);
    
    let program_header: ELF64ProgramHeader = unsafe { core::mem::transmute(buffer)};
    
    let entry_addr = header.e_entry;
    let offset: u64 = program_header.p_offset;
    let bytes_to_read: u64 = program_header.p_filesz;
    let memory_addr: *mut u8 = program_header.p_vaddr as *mut u8;

    let mut bytes_readed: usize = 0;

    let _ = vol_mgr.file_seek_from_start(file, offset as u32);
    let mut buff = [0; 32];
    let mut memory_addr_index: isize = 0;
    while bytes_readed < bytes_to_read as usize {
        // read
        let mut n_bytes = vol_mgr.read(file, &mut buff).unwrap();
        bytes_readed = bytes_readed + n_bytes;

        // write
        for b in &buff[0..n_bytes]{
            core::ptr::write_volatile(memory_addr.offset(memory_addr_index), *b);
            memory_addr_index = memory_addr_index + 1;

        }
    }
    return Some(core::mem::transmute(entry_addr));
    
}