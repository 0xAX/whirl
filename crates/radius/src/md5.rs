/// Implementation of MD5 Message-Digest Algorithm described in RFC 1321
/// For more info see: https://www.ietf.org/rfc/rfc1321.txt

pub fn md5(input: &[u8], output: &mut [u8]) {
    const BLK_SIZE: usize = 64;
    let len: usize = input.len();
    let (mut a, mut b, mut c, mut d) = (0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476);

    let blocks = (len / 64) + 1;
    for blk in 0..blocks {
        if blk == (blocks - 1)  {
            let last_blk_len = len.wrapping_sub(blk * 64);

            if last_blk_len == 0 {
                let mut block: Vec<u8> = vec![0; BLK_SIZE];
                block[0] = 0x80;

                put_u64_le(&mut block[56..], (len * 8) as u64);
                (a, b, c, d) = process_block(&block, &(a, b, c, d));

                continue;
            }

            if last_blk_len < 56 {
                let mut block: Vec<u8> = vec![0; BLK_SIZE];

                block[0..last_blk_len].clone_from_slice(&input[blk*64..blk*64+last_blk_len]);
                block[last_blk_len] = 0x80;

                put_u64_le(&mut block[56..], (len * 8) as u64);
                (a, b, c, d) = process_block(&block, &(a, b, c, d));

                continue;
            }

            if last_blk_len == 56 {
                let mut block: Vec<u8> = vec![0; BLK_SIZE];
                let mut additional_block: Vec<u8> = vec![0; BLK_SIZE];

                block[0..last_blk_len].clone_from_slice(&input[blk*64..blk*64+last_blk_len]);
                block[last_blk_len] = 0x80;
                (a, b, c, d) = process_block(&block, &(a, b, c, d));

                put_u64_le(&mut additional_block[56..], (len * 8) as u64);
                (a, b, c, d) = process_block(&additional_block, &(a, b, c, d));
                
                continue;
            }

            if last_blk_len > 56 {
                let mut block: Vec<u8> = vec![0; BLK_SIZE];
                let mut additional_block: Vec<u8> = vec![0; BLK_SIZE];

                block[0..last_blk_len].clone_from_slice(&input[blk*64..blk*64+last_blk_len]);
                block[last_blk_len] = 0x80;
                (a, b, c, d) = process_block(&block, &(a, b, c, d));
                
                put_u64_le(&mut additional_block[56..], (len * 8) as u64);
                (a, b, c, d) = process_block(&additional_block, &(a, b, c, d));
                
                continue;                
            }
        } else {
            (a, b, c, d) = process_block(&input[blk..(blk + BLK_SIZE as usize)].to_vec(), &(a, b, c, d));
        }
    }

    let aa = a.to_le_bytes();
    let bb = b.to_le_bytes();
    let cc = c.to_le_bytes();
    let dd = d.to_le_bytes();

    output[0] = aa[0];
    output[1] = aa[1];
    output[2] = aa[2];
    output[3] = aa[3];

    output[4] = bb[0];
    output[5] = bb[1];
    output[6] = bb[2];
    output[7] = bb[3];

    output[8] = cc[0];
    output[9] = cc[1];
    output[10] = cc[2];
    output[11] = cc[3];

    output[12] = dd[0];
    output[13] = dd[1];
    output[14] = dd[2];
    output[15] = dd[3];
}

fn f(x: u32, y: u32, z: u32) -> u32 {
    (x & y) | (!x & z)
}

fn g(x: u32, y: u32, z: u32) -> u32 {
    (x & z) | (y & ((!z) & 0xffffffff))
}

fn h(x: u32, y: u32, z: u32) -> u32 {
    x ^ y ^ z
}

fn i(x: u32, y: u32, z: u32) -> u32 {
    y ^ (x | ((!z) & 0xffffffff))
}

fn rotl(value: u32, count: u32) -> u32 {
    (value << count) | (value >> (32 - count))
}

fn put_u64_le(arr: &mut [u8], value: u64) {
    arr[0] = value as u8;
    arr[1] = (value >> 8) as u8;
    arr[2] = (value >> 16) as u8;
    arr[3] = (value >> 24) as u8;
    arr[4] = (value >> 32) as u8;
    arr[5] = (value >> 40) as u8;
    arr[6] = (value >> 48) as u8;
    arr[7] = (value >> 56) as u8;
}

fn process_block(block: &Vec<u8>, state: &(u32, u32, u32, u32)) -> (u32, u32, u32, u32) {
    let (mut a, mut b, mut c, mut d) = state;
    let (aa, bb, cc, dd) = state;

    let w0: u32 = u32::from_le_bytes(block[0..4].try_into().unwrap());
    let w1: u32 = u32::from_le_bytes(block[4..8].try_into().unwrap());
    let w2: u32 = u32::from_le_bytes(block[8..12].try_into().unwrap());
    let w3: u32 = u32::from_le_bytes(block[12..16].try_into().unwrap());
    let w4: u32 = u32::from_le_bytes(block[16..20].try_into().unwrap());
    let w5: u32 = u32::from_le_bytes(block[20..24].try_into().unwrap());
    let w6: u32 = u32::from_le_bytes(block[24..28].try_into().unwrap());
    let w7: u32 = u32::from_le_bytes(block[28..32].try_into().unwrap());
    let w8: u32 = u32::from_le_bytes(block[32..36].try_into().unwrap());
    let w9: u32 = u32::from_le_bytes(block[36..40].try_into().unwrap());
    let w10: u32 = u32::from_le_bytes(block[40..44].try_into().unwrap());
    let w11: u32 = u32::from_le_bytes(block[44..48].try_into().unwrap());
    let w12: u32 = u32::from_le_bytes(block[48..52].try_into().unwrap());
    let w13: u32 = u32::from_le_bytes(block[52..56].try_into().unwrap());
    let w14: u32 = u32::from_le_bytes(block[56..60].try_into().unwrap());
    let w15: u32 = u32::from_le_bytes(block[60..64].try_into().unwrap());

    // round 1
    a = b.wrapping_add(rotl(
        a.wrapping_add(f(b, c, d).try_into().unwrap())
            .wrapping_add(w0)
            .wrapping_add(0xd76aa478),
        7,
    ));
    d = a.wrapping_add(rotl(
        d.wrapping_add(f(a, b, c))
            .wrapping_add(w1)
            .wrapping_add(0xe8c7b756),
        12,
    ));
    c = d.wrapping_add(rotl(
        c.wrapping_add(f(d, a, b))
            .wrapping_add(w2)
            .wrapping_add(0x242070db),
        17,
    ));
    b = c.wrapping_add(rotl(
        b.wrapping_add(f(c, d, a))
            .wrapping_add(w3)
            .wrapping_add(0xc1bdceee),
        22,
    ));

    a = b.wrapping_add(rotl(
        a.wrapping_add(f(b, c, d))
            .wrapping_add(w4)
            .wrapping_add(0xf57c0faf),
        7,
    ));
    d = a.wrapping_add(rotl(
        d.wrapping_add(f(a, b, c))
            .wrapping_add(w5)
            .wrapping_add(0x4787c62a),
        12,
    ));
    c = d.wrapping_add(rotl(
        c.wrapping_add(f(d, a, b))
            .wrapping_add(w6)
            .wrapping_add(0xa8304613),
        17,
    ));
    b = c.wrapping_add(rotl(
        b.wrapping_add(f(c, d, a))
            .wrapping_add(w7)
            .wrapping_add(0xfd469501),
        22,
    ));

    a = b.wrapping_add(rotl(
        a.wrapping_add(f(b, c, d))
            .wrapping_add(w8)
            .wrapping_add(0x698098d8),
        7,
    ));
    d = a.wrapping_add(rotl(
        d.wrapping_add(f(a, b, c))
            .wrapping_add(w9)
            .wrapping_add(0x8b44f7af),
        12,
    ));
    c = d.wrapping_add(rotl(
        c.wrapping_add(f(d, a, b))
            .wrapping_add(w10)
            .wrapping_add(0xffff5bb1),
        17,
    ));
    b = c.wrapping_add(rotl(
        b.wrapping_add(f(c, d, a))
            .wrapping_add(w11)
            .wrapping_add(0x895cd7be),
        22,
    ));

    a = b.wrapping_add(rotl(
        a.wrapping_add(f(b, c, d))
            .wrapping_add(w12)
            .wrapping_add(0x6b901122),
        7,
    ));
    d = a.wrapping_add(rotl(
        d.wrapping_add(f(a, b, c))
            .wrapping_add(w13)
            .wrapping_add(0xfd987193),
        12,
    ));
    c = d.wrapping_add(rotl(
        c.wrapping_add(f(d, a, b))
            .wrapping_add(w14)
            .wrapping_add(0xa679438e),
        17,
    ));
    b = c.wrapping_add(rotl(
        b.wrapping_add(f(c, d, a))
            .wrapping_add(w15)
            .wrapping_add(0x49b40821),
        22,
    ));
    
    // round 2
    a = b.wrapping_add(rotl(
        a.wrapping_add(g(b, c, d))
            .wrapping_add(w1)
            .wrapping_add(0xf61e2562),
        5,
    ));
    d = a.wrapping_add(rotl(
        d.wrapping_add(g(a, b, c))
            .wrapping_add(w6)
            .wrapping_add(0xc040b340),
        9,
    ));
    c = d.wrapping_add(rotl(
        c.wrapping_add(g(d, a, b))
            .wrapping_add(w11)
            .wrapping_add(0x265e5a51),
        14,
    ));
    b = c.wrapping_add(rotl(
        b.wrapping_add(g(c, d, a))
            .wrapping_add(w0)
            .wrapping_add(0xe9b6c7aa),
        20,
    ));

    a = b.wrapping_add(rotl(
        a.wrapping_add(g(b, c, d))
            .wrapping_add(w5)
            .wrapping_add(0xd62f105d),
        5,
    ));
    d = a.wrapping_add(rotl(
        d.wrapping_add(g(a, b, c))
            .wrapping_add(w10)
            .wrapping_add(0x02441453),
        9,
    ));
    c = d.wrapping_add(rotl(
        c.wrapping_add(g(d, a, b))
            .wrapping_add(w15)
            .wrapping_add(0xd8a1e681),
        14,
    ));
    b = c.wrapping_add(rotl(
        b.wrapping_add(g(c, d, a))
            .wrapping_add(w4)
            .wrapping_add(0xe7d3fbc8),
        20,
    ));

    a = b.wrapping_add(rotl(
        a.wrapping_add(g(b, c, d))
            .wrapping_add(w9)
            .wrapping_add(0x21e1cde6),
        5,
    ));
    d = a.wrapping_add(rotl(
        d.wrapping_add(g(a, b, c))
            .wrapping_add(w14)
            .wrapping_add(0xc33707d6),
        9,
    ));
    c = d.wrapping_add(rotl(
        c.wrapping_add(g(d, a, b))
            .wrapping_add(w3)
            .wrapping_add(0xf4d50d87),
        14,
    ));
    b = c.wrapping_add(rotl(
        b.wrapping_add(g(c, d, a))
            .wrapping_add(w8)
            .wrapping_add(0x455a14ed),
        20,
    ));

    a = b.wrapping_add(rotl(
        a.wrapping_add(g(b, c, d))
            .wrapping_add(w13)
            .wrapping_add(0xa9e3e905),
        5,
    ));
    d = a.wrapping_add(rotl(
        d.wrapping_add(g(a, b, c))
            .wrapping_add(w2)
            .wrapping_add(0xfcefa3f8),
        9,
    ));
    c = d.wrapping_add(rotl(
        c.wrapping_add(g(d, a, b))
            .wrapping_add(w7)
            .wrapping_add(0x676f02d9),
        14,
    ));
    b = c.wrapping_add(rotl(
        b.wrapping_add(g(c, d, a))
            .wrapping_add(w12)
            .wrapping_add(0x8d2a4c8a),
        20,
    ));

    // round 3
    a = b.wrapping_add(rotl(
        a.wrapping_add(h(b, c, d))
            .wrapping_add(w5)
            .wrapping_add(0xfffa3942),
        4,
    ));
    d = a.wrapping_add(rotl(
        d.wrapping_add(h(a, b, c))
            .wrapping_add(w8)
            .wrapping_add(0x8771f681),
        11,
    ));
    c = d.wrapping_add(rotl(
        c.wrapping_add(h(d, a, b))
            .wrapping_add(w11)
            .wrapping_add(0x6d9d6122),
        16,
    ));
    b = c.wrapping_add(rotl(
        b.wrapping_add(h(c, d, a))
            .wrapping_add(w14)
            .wrapping_add(0xfde5380c),
        23,
    ));

    a = b.wrapping_add(rotl(
        a.wrapping_add(h(b, c, d))
            .wrapping_add(w1)
            .wrapping_add(0xa4beea44),
        4,
    ));
    d = a.wrapping_add(rotl(
        d.wrapping_add(h(a, b, c))
            .wrapping_add(w4)
            .wrapping_add(0x4bdecfa9),
        11,
    ));
    c = d.wrapping_add(rotl(
        c.wrapping_add(h(d, a, b))
            .wrapping_add(w7)
            .wrapping_add(0xf6bb4b60),
        16,
    ));
    b = c.wrapping_add(rotl(
        b.wrapping_add(h(c, d, a))
            .wrapping_add(w10)
            .wrapping_add(0xbebfbc70),
        23,
    ));

    a = b.wrapping_add(rotl(
        a.wrapping_add(h(b, c, d))
            .wrapping_add(w13)
            .wrapping_add(0x289b7ec6),
        4,
    ));
    d = a.wrapping_add(rotl(
        d.wrapping_add(h(a, b, c))
            .wrapping_add(w0)
            .wrapping_add(0xeaa127fa),
        11,
    ));
    c = d.wrapping_add(rotl(
        c.wrapping_add(h(d, a, b))
            .wrapping_add(w3)
            .wrapping_add(0xd4ef3085),
        16,
    ));
    b = c.wrapping_add(rotl(
        b.wrapping_add(h(c, d, a))
            .wrapping_add(w6)
            .wrapping_add(0x04881d05),
        23,
    ));

    a = b.wrapping_add(rotl(
        a.wrapping_add(h(b, c, d))
            .wrapping_add(w9)
            .wrapping_add(0xd9d4d039),
        4,
    ));

    d = a.wrapping_add(rotl(
        d.wrapping_add(h(a, b, c))
            .wrapping_add(w12)
            .wrapping_add(0xe6db99e5),
        11,
    ));
    c = d.wrapping_add(rotl(
        c.wrapping_add(h(d, a, b))
            .wrapping_add(w15)
            .wrapping_add(0x1fa27cf8),
        16,
    ));
    b = c.wrapping_add(rotl(
        b.wrapping_add(h(c, d, a))
            .wrapping_add(w2)
            .wrapping_add(0xc4ac5665),
        23,
    ));

    // round 4
    a = b.wrapping_add(rotl(
        a.wrapping_add(i(b, c, d))
            .wrapping_add(w0)
            .wrapping_add(0xf4292244),
        6,
    ));
    d = a.wrapping_add(rotl(
        d.wrapping_add(i(a, b, c))
            .wrapping_add(w7)
            .wrapping_add(0x432aff97),
        10,
    ));
    c = d.wrapping_add(rotl(
        c.wrapping_add(i(d, a, b))
            .wrapping_add(w14)
            .wrapping_add(0xab9423a7),
        15,
    ));
    b = c.wrapping_add(rotl(
        b.wrapping_add(i(c, d, a))
            .wrapping_add(w5)
            .wrapping_add(0xfc93a039),
        21,
    ));

    a = b.wrapping_add(rotl(
        a.wrapping_add(i(b, c, d))
            .wrapping_add(w12)
            .wrapping_add(0x655b59c3),
        6,
    ));
    d = a.wrapping_add(rotl(
        d.wrapping_add(i(a, b, c))
            .wrapping_add(w3)
            .wrapping_add(0x8f0ccc92),
        10,
    ));
    c = d.wrapping_add(rotl(
        c.wrapping_add(i(d, a, b))
            .wrapping_add(w10)
            .wrapping_add(0xffeff47d),
        15,
    ));
    b = c.wrapping_add(rotl(
        b.wrapping_add(i(c, d, a))
            .wrapping_add(w1)
            .wrapping_add(0x85845dd1),
        21,
    ));

    a = b.wrapping_add(rotl(
        a.wrapping_add(i(b, c, d))
            .wrapping_add(w8)
            .wrapping_add(0x6fa87e4f),
        6,
    ));
    d = a.wrapping_add(rotl(
        d.wrapping_add(i(a, b, c))
            .wrapping_add(w15)
            .wrapping_add(0xfe2ce6e0),
        10,
    ));
    c = d.wrapping_add(rotl(
        c.wrapping_add(i(d, a, b))
            .wrapping_add(w6)
            .wrapping_add(0xa3014314),
        15,
    ));
    b = c.wrapping_add(rotl(
        b.wrapping_add(i(c, d, a))
            .wrapping_add(w13)
            .wrapping_add(0x4e0811a1),
        21,
    ));
    
    a = b.wrapping_add(rotl(
        a.wrapping_add(i(b, c, d))
            .wrapping_add(w4)
            .wrapping_add(0xf7537e82),
        6,
    ));
    d = a.wrapping_add(rotl(
        d.wrapping_add(i(a, b, c))
            .wrapping_add(w11)
            .wrapping_add(0xbd3af235),
        10,
    ));
    c = d.wrapping_add(rotl(
        c.wrapping_add(i(d, a, b))
            .wrapping_add(w2)
            .wrapping_add(0x2ad7d2bb),
        15,
    ));
    b = c.wrapping_add(rotl(
        b.wrapping_add(i(c, d, a))
            .wrapping_add(w9)
            .wrapping_add(0xeb86d391),
        21,
    ));
    
    a = aa.wrapping_add(a);
    b = bb.wrapping_add(b);
    c = cc.wrapping_add(c);
    d = dd.wrapping_add(d);

    (a, b, c, d)
}

#[cfg(test)]
mod md5_tests {
    use super::*;

    const HASH_SIZE: usize = 16;
    
    fn slice_to_hex_digest(data: &[u8]) -> String {
        format!("{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
                data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7], data[8],
                data[9], data[10], data[11], data[12], data[13], data[14], data[15])
    }
 
    #[test]
    fn md5_test() {
        let mut test = "900150983cd24fb0d6963f7d28e17f72";
        let mut res: [u8; HASH_SIZE] = Default::default();

        md5("abc".as_ref(), &mut res);
        assert_eq!(test, slice_to_hex_digest(&res));

        test = "d41d8cd98f00b204e9800998ecf8427e";
        md5("".as_ref(), &mut res);
        assert_eq!(test, slice_to_hex_digest(&res));

        test = "d174ab98d277d9f5a5611c2c9f419d9f";
        md5("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789".as_ref(), &mut res);
        assert_eq!(test, slice_to_hex_digest(&res));

        test = "64908fc7dd61c3ab33ce1c594ee1783c";
        md5("1234567890123456789012345678901234567890123456789012345678901230".as_ref(), &mut res);
        assert_eq!(test, slice_to_hex_digest(&res));

        test = "0cc175b9c0f1b6a831c399e269772661";
        md5("a".as_ref(), &mut res);
        assert_eq!(test, slice_to_hex_digest(&res));

        test = "f96b697d7cb7938d525a2f31aaf161d0";
        md5("message digest".as_ref(), &mut res);
        assert_eq!(test, slice_to_hex_digest(&res));

        test = "c3fcd3d76192e4007dfb496cca67e13b";
        md5("abcdefghijklmnopqrstuvwxyz".as_ref(), &mut res);
        assert_eq!(test, slice_to_hex_digest(&res));

        test = "c3fcd3d76192e4007dfb496cca67e13b";
        md5("abcdefghijklmnopqrstuvwxyz".as_ref(), &mut res);
        assert_eq!(test, slice_to_hex_digest(&res));

        test = "6e327dc9ad79694a191ed07bc48a69ed";
        md5("12345678901234567890123456789012345678901234567890123456789012305".as_ref(), &mut res);
        assert_eq!(test, slice_to_hex_digest(&res));        
    }
}
