fn main() {
    let input = String::from(
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()_+-=",
    );
    println!("{}", input);
    let bytes = Vec::from(input.as_bytes());
    println!("{}", md5(bytes));
}

fn md5(mut bytes: Vec<u8>) -> String {
    let len = bytes.len();
    if len % 64 != 56 {
        bytes.push(1 << 7);
        for _i in (len + 1) % 64..56 {
            bytes.push(0);
        }
    }
    let size = len * 8;
    bytes.extend_from_slice(&size.to_le_bytes());
    bytes.push(0);
    bytes.push(0);
    bytes.push(0);
    bytes.push(0);

    let (mut a, mut b, mut c, mut d) = (
        0x67452301 as u32,
        0xEFCDAB89 as u32,
        0x98BADCFE as u32,
        0x10325476 as u32,
    );
    for i in 0..bytes.len() / 64 {
        let (mut _a, mut _b, mut _c, mut _d) = (a, b, c, d);
        let bs = |bslice: &[u8]| -> Vec<u32> {
            let mut r = Vec::new();
            for i in 0..64 / 4 {
                r.push(
                    ((bslice[i * 4 + 3] as u32) << 24)
                        + ((bslice[i * 4 + 2] as u32) << 16)
                        + ((bslice[i * 4 + 1] as u32) << 8)
                        + (bslice[i * 4] as u32),
                );
            }
            r
        }(&bytes[i * 64..i * 64 + 64]);

        _a = ff(_a, _b, _c, _d, bs[0], 7, 0xd76aa478);
        _d = ff(_d, _a, _b, _c, bs[1], 12, 0xe8c7b756);
        _c = ff(_c, _d, _a, _b, bs[2], 17, 0x242070db);
        _b = ff(_b, _c, _d, _a, bs[3], 22, 0xc1bdceee);
        _a = ff(_a, _b, _c, _d, bs[4], 7, 0xf57c0faf);
        _d = ff(_d, _a, _b, _c, bs[5], 12, 0x4787c62a);
        _c = ff(_c, _d, _a, _b, bs[6], 17, 0xa8304613);
        _b = ff(_b, _c, _d, _a, bs[7], 22, 0xfd469501);
        _a = ff(_a, _b, _c, _d, bs[8], 7, 0x698098d8);
        _d = ff(_d, _a, _b, _c, bs[9], 12, 0x8b44f7af);
        _c = ff(_c, _d, _a, _b, bs[10], 17, 0xffff5bb1);
        _b = ff(_b, _c, _d, _a, bs[11], 22, 0x895cd7be);
        _a = ff(_a, _b, _c, _d, bs[12], 7, 0x6b901122);
        _d = ff(_d, _a, _b, _c, bs[13], 12, 0xfd987193);
        _c = ff(_c, _d, _a, _b, bs[14], 17, 0xa679438e);
        _b = ff(_b, _c, _d, _a, bs[15], 22, 0x49b40821);

        _a = gg(_a, _b, _c, _d, bs[1], 5, 0xf61e2562);
        _d = gg(_d, _a, _b, _c, bs[6], 9, 0xc040b340);
        _c = gg(_c, _d, _a, _b, bs[11], 14, 0x265e5a51);
        _b = gg(_b, _c, _d, _a, bs[0], 20, 0xe9b6c7aa);
        _a = gg(_a, _b, _c, _d, bs[5], 5, 0xd62f105d);
        _d = gg(_d, _a, _b, _c, bs[10], 9, 0x02441453);
        _c = gg(_c, _d, _a, _b, bs[15], 14, 0xd8a1e681);
        _b = gg(_b, _c, _d, _a, bs[4], 20, 0xe7d3fbc8);
        _a = gg(_a, _b, _c, _d, bs[9], 5, 0x21e1cde6);
        _d = gg(_d, _a, _b, _c, bs[14], 9, 0xc33707d6);
        _c = gg(_c, _d, _a, _b, bs[3], 14, 0xf4d50d87);
        _b = gg(_b, _c, _d, _a, bs[8], 20, 0x455a14ed);
        _a = gg(_a, _b, _c, _d, bs[13], 5, 0xa9e3e905);
        _d = gg(_d, _a, _b, _c, bs[2], 9, 0xfcefa3f8);
        _c = gg(_c, _d, _a, _b, bs[7], 14, 0x676f02d9);
        _b = gg(_b, _c, _d, _a, bs[12], 20, 0x8d2a4c8a);

        _a = hh(_a, _b, _c, _d, bs[5], 4, 0xfffa3942);
        _d = hh(_d, _a, _b, _c, bs[8], 11, 0x8771f681);
        _c = hh(_c, _d, _a, _b, bs[11], 16, 0x6d9d6122);
        _b = hh(_b, _c, _d, _a, bs[14], 23, 0xfde5380c);
        _a = hh(_a, _b, _c, _d, bs[1], 4, 0xa4beea44);
        _d = hh(_d, _a, _b, _c, bs[4], 11, 0x4bdecfa9);
        _c = hh(_c, _d, _a, _b, bs[7], 16, 0xf6bb4b60);
        _b = hh(_b, _c, _d, _a, bs[10], 23, 0xbebfbc70);
        _a = hh(_a, _b, _c, _d, bs[13], 4, 0x289b7ec6);
        _d = hh(_d, _a, _b, _c, bs[0], 11, 0xeaa127fa);
        _c = hh(_c, _d, _a, _b, bs[3], 16, 0xd4ef3085);
        _b = hh(_b, _c, _d, _a, bs[6], 23, 0x04881d05);
        _a = hh(_a, _b, _c, _d, bs[9], 4, 0xd9d4d039);
        _d = hh(_d, _a, _b, _c, bs[12], 11, 0xe6db99e5);
        _c = hh(_c, _d, _a, _b, bs[15], 16, 0x1fa27cf8);
        _b = hh(_b, _c, _d, _a, bs[2], 23, 0xc4ac5665);

        _a = ii(_a, _b, _c, _d, bs[0], 6, 0xf4292244);
        _d = ii(_d, _a, _b, _c, bs[7], 10, 0x432aff97);
        _c = ii(_c, _d, _a, _b, bs[14], 15, 0xab9423a7);
        _b = ii(_b, _c, _d, _a, bs[5], 21, 0xfc93a039);
        _a = ii(_a, _b, _c, _d, bs[12], 6, 0x655b59c3);
        _d = ii(_d, _a, _b, _c, bs[3], 10, 0x8f0ccc92);
        _c = ii(_c, _d, _a, _b, bs[10], 15, 0xffeff47d);
        _b = ii(_b, _c, _d, _a, bs[1], 21, 0x85845dd1);
        _a = ii(_a, _b, _c, _d, bs[8], 6, 0x6fa87e4f);
        _d = ii(_d, _a, _b, _c, bs[15], 10, 0xfe2ce6e0);
        _c = ii(_c, _d, _a, _b, bs[6], 15, 0xa3014314);
        _b = ii(_b, _c, _d, _a, bs[13], 21, 0x4e0811a1);
        _a = ii(_a, _b, _c, _d, bs[4], 6, 0xf7537e82);
        _d = ii(_d, _a, _b, _c, bs[11], 10, 0xbd3af235);
        _c = ii(_c, _d, _a, _b, bs[2], 15, 0x2ad7d2bb);
        _b = ii(_b, _c, _d, _a, bs[9], 21, 0xeb86d391);

        a = a.wrapping_add(_a);
        b = b.wrapping_add(_b);
        c = c.wrapping_add(_c);
        d = d.wrapping_add(_d);
    }

    format!(
        "{:X} {:X} {:X} {:X}",
        reverse_bytes(a),
        reverse_bytes(b),
        reverse_bytes(c),
        reverse_bytes(d)
    )
}

fn reverse_bytes(b: u32) -> u32 {
    ((b & 0x000000FF) << 24)
        | ((b & 0x0000FF00) << 8)
        | ((b & 0x00FF0000) >> 8)
        | ((b & 0xFF000000) >> 24)
}

fn ff(a: u32, b: u32, c: u32, d: u32, mj: u32, s: u32, ti: u32) -> u32 {
    ((b & c) | (!b & d))
        .wrapping_add(a)
        .wrapping_add(mj)
        .wrapping_add(ti)
        .rotate_left(s)
        .wrapping_add(b)
}

fn gg(a: u32, b: u32, c: u32, d: u32, mj: u32, s: u32, ti: u32) -> u32 {
    ((b & d) | (c & !d))
        .wrapping_add(a)
        .wrapping_add(mj)
        .wrapping_add(ti)
        .rotate_left(s)
        .wrapping_add(b)
}

fn hh(a: u32, b: u32, c: u32, d: u32, mj: u32, s: u32, ti: u32) -> u32 {
    (b ^ c ^ d)
        .wrapping_add(a)
        .wrapping_add(mj)
        .wrapping_add(ti)
        .rotate_left(s)
        .wrapping_add(b)
}

fn ii(a: u32, b: u32, c: u32, d: u32, mj: u32, s: u32, ti: u32) -> u32 {
    (c ^ (b | !d))
        .wrapping_add(a)
        .wrapping_add(mj)
        .wrapping_add(ti)
        .rotate_left(s)
        .wrapping_add(b)
}
