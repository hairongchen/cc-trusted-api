use log::info;
use core::mem;

/* dumnp raw cc report in following format:
    00000000 04 00 02 00 81 00 00 00 00 00 00 00 93 9A 72 33  ..............r3
    00000010 F7 9C 4C A9 94 0A 0D B3 95 7F 06 07 D5 68 59 C7  ..L..........hY.
    00000020 35 FB B4 91 29 27 55 B2 E8 E8 23 B6 00 00 00 00  5...)'U...#.....
...
*/

pub fn dump_data(data: &Vec<u8>) {
    let mut index: usize = 0;
    let mut linestr = "".to_string();
    let mut printstr = "".to_string();

    let printable = vec![
        ' ', '\t', '\n', '\r', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
        'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E',
        'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W',
        'X', 'Y', 'Z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e',
        'f', 'A', 'B', 'C', 'D', 'E', 'F', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-',
        '.', '/', ':', ';', '<', '=', '>', '?', '@', '[', '\\', ']', '^', '_', '`', '{', '|', '}',
        '~', '"', '!',
    ];

    while usize::from(index) < data.len() {
        if index % 16 == 0 {
            if printstr.len() != 0 {
                info!("{} {}", linestr, printstr);
                printstr = "".to_string();
            }
            linestr = format!("{:08X} ", ((index / 16) as u16) * 16);
        }

        let v = data[index];
        linestr.push_str(format!("{:02X} ", v).as_str());
        match printable.iter().position(|&c| c == (v as char)) {
            Some(_) => {
                if v < 0x9 || v > 0xD {
                    printstr.push_str(core::str::from_utf8(&[v]).unwrap());
                } else {
                    printstr.push_str(".");
                }
            }
            None => printstr.push_str("."),
        }

        index += 1;
    }

    if index % 16 != 0 {
        let mut blank = "".to_string();
        for _ in 1..=(16 - index % 16) {
            blank.push_str("   ");
        }
        info!("{}{} {}", linestr, blank, printstr);
    } else if usize::from(index) == data.len() {
        info!("{} {}", linestr, printstr);
    }
}

//TODO: error handling
pub fn get_u8(data: &Vec<u8>) -> u8 {
    unsafe { mem::transmute::<<u8>, u8>(data[0]) }
}

pub fn get_u16(data: &Vec<u8>) -> u16 {
    unsafe { mem::transmute::<[u8; 2], u8>(data[0..2]).into() }
}

pub fn get_u32(data: &Vec<u8>) -> u32 {
    unsafe { mem::transmute::<[u8; 4], u8>(data[0..4]).into() }
}
