use super::config::Msz;

pub static mut M: &'static [u8] = &mut [0..Msz];//&mut [0; Msz as usize];

pub fn dump(addr: u32, sz: u32) -> String {
    let mut ret = String::from("");
    for a in addr..addr + sz {
        let mut byte: u8 = 0;
        unsafe {
            byte = M[a as usize];
        }
        ret.push_str(&format!("{:02X} ", byte))
    }
    ret
}
