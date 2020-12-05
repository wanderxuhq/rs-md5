fn main() {
    let input = String::from("啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊啊");
    //let input = String::from("Hello World");
    println!("{}", input);
    let mut bytes = Vec::from(input.as_bytes());
    let len = bytes.len();
    if len % 64 != 56 {
        bytes.push(1 << 7);
        for _i in (len + 1) % 64..56 {
            bytes.push(0);
        }
    }
    let size = len * 8;
    println!("{:?}", len);
    println!("{:?}", size.to_le_bytes());
    bytes.extend_from_slice(&size.to_le_bytes());
    bytes.push(0);
    bytes.push(0);
    bytes.push(0);
    bytes.push(0);
    let mut a = 0x67452301 as u32;
    let mut b = 0xEFCDAB89 as u32;
    let mut c = 0x98BADCFE as u32;
    let mut d = 0x10325476 as u32;
    let mut A = 0x67452301 as u32;
    let mut B = 0xEFCDAB89 as u32;
    let mut C = 0x98BADCFE as u32;
    let mut D = 0x10325476 as u32;
    for i in 0..bytes.len() / 64 {
        a = A;
        b = B;
        c = C;
        d = D;
        println!("{}", i);
        let bslice = &bytes[i * 64..i * 64 + 64];
        let bs = [
            tou32(&bslice[0..4]),
            tou32(&bslice[4..8]),
            tou32(&bslice[8..12]),
            tou32(&bslice[12..16]),
            tou32(&bslice[16..20]),
            tou32(&bslice[20..24]),
            tou32(&bslice[24..28]),
            tou32(&bslice[28..32]),
            tou32(&bslice[32..36]),
            tou32(&bslice[36..40]),
            tou32(&bslice[40..44]),
            tou32(&bslice[44..48]),
            tou32(&bslice[48..52]),
            tou32(&bslice[52..56]),
            tou32(&bslice[56..60]),
            tou32(&bslice[60..64]),
        ];
        a = ff(a, b, c, d, bs[0], 7, 0xd76aa478);
        d = ff(d, a, b, c, bs[1], 12, 0xe8c7b756);
        c = ff(c, d, a, b, bs[2], 17, 0x242070db);
        b = ff(b, c, d, a, bs[3], 22, 0xc1bdceee);
        a = ff(a, b, c, d, bs[4], 7, 0xf57c0faf);
        d = ff(d, a, b, c, bs[5], 12, 0x4787c62a);
        c = ff(c, d, a, b, bs[6], 17, 0xa8304613);
        b = ff(b, c, d, a, bs[7], 22, 0xfd469501);
        a = ff(a, b, c, d, bs[8], 7, 0x698098d8);
        d = ff(d, a, b, c, bs[9], 12, 0x8b44f7af);
        c = ff(c, d, a, b, bs[10], 17, 0xffff5bb1);
        b = ff(b, c, d, a, bs[11], 22, 0x895cd7be);
        a = ff(a, b, c, d, bs[12], 7, 0x6b901122);
        d = ff(d, a, b, c, bs[13], 12, 0xfd987193);
        c = ff(c, d, a, b, bs[14], 17, 0xa679438e);
        b = ff(b, c, d, a, bs[15], 22, 0x49b40821);

        a = gg(a, b, c, d, bs[1], 5, 0xf61e2562);
        d = gg(d, a, b, c, bs[6], 9, 0xc040b340);
        c = gg(c, d, a, b, bs[11], 14, 0x265e5a51);
        b = gg(b, c, d, a, bs[0], 20, 0xe9b6c7aa);
        a = gg(a, b, c, d, bs[5], 5, 0xd62f105d);
        d = gg(d, a, b, c, bs[10], 9, 0x02441453);
        c = gg(c, d, a, b, bs[15], 14, 0xd8a1e681);
        b = gg(b, c, d, a, bs[4], 20, 0xe7d3fbc8);
        a = gg(a, b, c, d, bs[9], 5, 0x21e1cde6);
        d = gg(d, a, b, c, bs[14], 9, 0xc33707d6);
        c = gg(c, d, a, b, bs[3], 14, 0xf4d50d87);
        b = gg(b, c, d, a, bs[8], 20, 0x455a14ed);
        a = gg(a, b, c, d, bs[13], 5, 0xa9e3e905);
        d = gg(d, a, b, c, bs[2], 9, 0xfcefa3f8);
        c = gg(c, d, a, b, bs[7], 14, 0x676f02d9);
        b = gg(b, c, d, a, bs[12], 20, 0x8d2a4c8a);

        a = hh(a, b, c, d, bs[5], 4, 0xfffa3942);
        d = hh(d, a, b, c, bs[8], 11, 0x8771f681);
        c = hh(c, d, a, b, bs[11], 16, 0x6d9d6122);
        b = hh(b, c, d, a, bs[14], 23, 0xfde5380c);
        a = hh(a, b, c, d, bs[1], 4, 0xa4beea44);
        d = hh(d, a, b, c, bs[4], 11, 0x4bdecfa9);
        c = hh(c, d, a, b, bs[7], 16, 0xf6bb4b60);
        b = hh(b, c, d, a, bs[10], 23, 0xbebfbc70);
        a = hh(a, b, c, d, bs[13], 4, 0x289b7ec6);
        d = hh(d, a, b, c, bs[0], 11, 0xeaa127fa);
        c = hh(c, d, a, b, bs[3], 16, 0xd4ef3085);
        b = hh(b, c, d, a, bs[6], 23, 0x04881d05);
        a = hh(a, b, c, d, bs[9], 4, 0xd9d4d039);
        d = hh(d, a, b, c, bs[12], 11, 0xe6db99e5);
        c = hh(c, d, a, b, bs[15], 16, 0x1fa27cf8);
        b = hh(b, c, d, a, bs[2], 23, 0xc4ac5665);

        a = ii(a, b, c, d, bs[0], 6, 0xf4292244);
        d = ii(d, a, b, c, bs[7], 10, 0x432aff97);
        c = ii(c, d, a, b, bs[14], 15, 0xab9423a7);
        b = ii(b, c, d, a, bs[5], 21, 0xfc93a039);
        a = ii(a, b, c, d, bs[12], 6, 0x655b59c3);
        d = ii(d, a, b, c, bs[3], 10, 0x8f0ccc92);
        c = ii(c, d, a, b, bs[10], 15, 0xffeff47d);
        b = ii(b, c, d, a, bs[1], 21, 0x85845dd1);
        a = ii(a, b, c, d, bs[8], 6, 0x6fa87e4f);
        d = ii(d, a, b, c, bs[15], 10, 0xfe2ce6e0);
        c = ii(c, d, a, b, bs[6], 15, 0xa3014314);
        b = ii(b, c, d, a, bs[13], 21, 0x4e0811a1);
        a = ii(a, b, c, d, bs[4], 6, 0xf7537e82);
        d = ii(d, a, b, c, bs[11], 10, 0xbd3af235);
        c = ii(c, d, a, b, bs[2], 15, 0x2ad7d2bb);
        b = ii(b, c, d, a, bs[9], 21, 0xeb86d391);
        A = A.wrapping_add(a);
        B = B.wrapping_add(b);
        C = C.wrapping_add(c);
        D = D.wrapping_add(d);

    }

    println!(
        "{:X} {:X} {:X} {:X}",
        reverse_byte(A),
        reverse_byte(B),
        reverse_byte(C),
        reverse_byte(D)
    );
}

fn tou32(b: &[u8]) -> u32 {
    ((b[3] as u32) << 24) + ((b[2] as u32) << 16) + ((b[1] as u32) << 8) + (b[0] as u32)
}

fn reverse_byte(b: u32) -> u32 {
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
