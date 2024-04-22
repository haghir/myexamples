use itertools::Itertools;

const CHR: &'static [u8] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

fn main() {
    for _ in 0..5 {
        let data: Vec<u8> = (0..20).map(|_| rand::random::<u8>()).collect();
        calc(&data);
    }

    let data: Vec<u8> = (0..20).map(|i| if i < 8 {0} else {255}).collect();
    calc(&data);
}

fn calc(data: &Vec<u8>) {
    let base58 = to_base58(data);
    println!("{} ({})", base58, base58.len());
}

// based on https://github.com/trezor/trezor-crypto/blob/master/base58.c
fn to_base58(data: &Vec<u8>) -> String {
    let base = CHR.len() as u32;
    let zcount = data.iter().take_while(|x| **x == 0).count();
    let slen = data.len();
    let dlen = (slen - zcount) * 137 / 100 + 1;
    let mut buf = vec![0u8; dlen];

    let mut i = zcount;
    let mut h = 0;
    while i < slen {
        let mut carry = data[i] as u32;
        let mut j = 0;

        while j < h || carry != 0 {
            carry += 256 * buf[j] as u32;
            buf[j] = (carry % base) as u8;
            carry /= base;
            j += 1;
        }

        i += 1;
        h = j;
    }

    let mut ret = (0..zcount).map(|_| "1").join("");
    for i in (dlen - h)..dlen {
        ret.push(CHR[buf[dlen - i - 1] as usize] as char);
    }

    ret
}
