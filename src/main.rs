extern crate encoding;
use encoding::utf8_to_cp932;
use encoding::cp932_to_utf8;

fn main() {
    //
    let msg = "あいうえお123漢字";
    let mut c = 0;
    for v in utf8_to_cp932::convert(msg).unwrap() {
        print!("{:02X} ", v);
        c += 1;
        if c > 15 {
            println!("");
            c = 0;
        }
    }
    println!("");

    //
    let msg = vec![0x82, 0xA0, 0x82, 0xA2, 0x82, 0xA4, 0x82, 0xA6,
                   0x82, 0xA8, 0x31, 0x32, 0x33, 0x8A, 0xBF, 0x8E, 0x9A ];
    let ans = cp932_to_utf8::convert(&msg);
    println!("{}", ans.unwrap());
}
