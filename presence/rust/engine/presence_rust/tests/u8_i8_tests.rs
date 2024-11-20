#[test]
fn test_u8_i8() {
    let u8_data : Vec<u8> = vec![0b01, 0b10, 0b11];
    println!("data in u8:");
    for i in &u8_data {
        println!("{:08b}", i);
    }
    let mut i8_data : Vec<i8> = Vec::new();
    for i in u8_data {
       i8_data.push(i as i8) ;
    }
    println!("data in i8:");
    for i in &i8_data {
        println!("{:08b}", i);
    }
    assert!(true);
}