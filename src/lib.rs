#![no_std]

// u16 is maximum 5-character value
pub fn convert_adc_to_str(count: u16) -> heapless::String<5> {
    // info: adc reads at a resolution of 4096
    let mut buffer = heapless::Vec::<u8, 5>::from_slice(b"00000").unwrap();

    let mut pow: i32 = 10; // no exponent function in no-std, use this as substitute
    for idx in 0..5 {
        let ch: u8;

        // calculate character based on the digit-index
        if idx == 0 {
            ch = (count % 10) as u8;
        } else {
            ch = ((count as i32 / pow) % 10) as u8;
            pow *= 10;
        }

        // save character to the buffer
        buffer[4 - idx] = ch + '0' as u8;
    }

    heapless::String::<5>::from_utf8(buffer).unwrap()
}
