// SPK image assembly — port of nuttx/tools/cxd56/mkspk.c:create_image()

use object::elf::PT_LOAD;
use object::endian::LittleEndian;
use object::read::elf::{ElfFile32, ProgramHeader};
use object::read::{Object, ObjectSymbol};

use crate::MkSpkError;

const VMK: [u8; 16] = [
    0x27, 0xc0, 0xaf, 0x1b, 0x5d, 0xcb, 0xc6, 0xc5, 0x58, 0x22, 0x1c, 0xdd, 0xaf, 0xf3, 0x20, 0x21,
];

fn align16(x: usize) -> usize {
    (x + 15) & !15
}

pub fn build(elf_bytes: &[u8], savename: &str, core: u8) -> Result<Vec<u8>, MkSpkError> {
    if savename.len() > 63 {
        return Err(MkSpkError::NameTooLong);
    }

    let elf = ElfFile32::<LittleEndian>::parse(elf_bytes)
        .map_err(|e| MkSpkError::InvalidElf(e.to_string()))?;

    let entry = elf.entry() as u32;

    // Find __stack symbol value (0 if absent)
    let sp: u32 = elf
        .symbols()
        .find(|s| s.name().ok() == Some("__stack"))
        .map(|s| s.address() as u32)
        .unwrap_or(0);

    // Collect qualifying PT_LOAD program headers (p_memsz > 0)
    let phdrs: Vec<_> = elf
        .elf_program_headers()
        .iter()
        .filter(|ph| ph.p_type(LittleEndian) == PT_LOAD && ph.p_memsz(LittleEndian) > 0)
        .collect();

    if phdrs.is_empty() {
        return Err(MkSpkError::NoLoadablePhdrs);
    }

    let nphs = phdrs.len();
    let psize: usize = phdrs
        .iter()
        .map(|ph| align16(ph.p_filesz(LittleEndian) as usize))
        .sum();

    let snlen = align16(savename.len() + 1);
    // layout: header(32) + savename(snlen) + prog_info(nphs*16) + payloads
    let imgsize = 32 + snlen + nphs * 16 + psize;

    // zero-initialized image (matches C's malloc+memset pattern)
    let mut img = vec![0u8; imgsize + 32]; // +32 headroom for CMAC

    // ---- header (32 bytes, all fields little-endian) ----
    img[0] = 0xef;
    img[1] = b'M';
    img[2] = b'O';
    img[3] = b'D';
    img[4] = core;
    // reserved[11] = [5..16] — already zero
    img[16..20].copy_from_slice(&entry.to_le_bytes());
    img[20..24].copy_from_slice(&sp.to_le_bytes());
    img[24..26].copy_from_slice(&(core as u16).to_le_bytes());
    img[26..28].copy_from_slice(&(nphs as u16).to_le_bytes());
    let phoffs = (32 + snlen) as u16;
    img[28..30].copy_from_slice(&phoffs.to_le_bytes());
    img[30..32].copy_from_slice(&0o777u16.to_le_bytes());

    // ---- savename: copy up to 63 bytes into zero-init region ----
    let copy_len = savename.len().min(63);
    img[32..32 + copy_len].copy_from_slice(&savename.as_bytes()[..copy_len]);

    // ---- prog_info array + payloads ----
    let pi_base = 32 + snlen;
    let mut payload_offset: usize = pi_base + nphs * 16;

    for (i, ph) in phdrs.iter().enumerate() {
        let load_address = ph.p_paddr(LittleEndian);
        let file_off = ph.p_offset(LittleEndian) as usize;
        let file_sz = ph.p_filesz(LittleEndian) as usize;
        let mem_sz = ph.p_memsz(LittleEndian);
        let padded = align16(file_sz);

        let pi_off = pi_base + i * 16;
        img[pi_off..pi_off + 4].copy_from_slice(&load_address.to_le_bytes());
        img[pi_off + 4..pi_off + 8].copy_from_slice(&(payload_offset as u32).to_le_bytes());
        img[pi_off + 8..pi_off + 12].copy_from_slice(&(padded as u32).to_le_bytes());
        img[pi_off + 12..pi_off + 16].copy_from_slice(&mem_sz.to_le_bytes());

        // copy exactly file_sz bytes; the 16-aligned padding stays zero
        if file_sz > 0 {
            img[payload_offset..payload_offset + file_sz]
                .copy_from_slice(&elf_bytes[file_off..file_off + file_sz]);
        }

        payload_offset += padded;
    }

    // ---- CLEFIA-CMAC over imgsize bytes ----
    let cmac = crate::clefia::calc_cmac(&VMK, &img[..imgsize]).expect("imgsize must be 16-aligned");
    img[imgsize..imgsize + 16].copy_from_slice(&cmac);

    // ---- footer ----
    let total = imgsize + 16 + 16;
    img.resize(total, 0);
    img[imgsize + 16..total].copy_from_slice(b"MKSPK_BN_FOOTER\0");

    Ok(img)
}
