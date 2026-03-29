fn main() {
    let x: u8 = 255;
    let x1 = x.wrapping_add(1);
    let x2 = x.checked_add(1);
    let (x3, b1) = x.overflowing_add(1);
    let x4 = x.saturating_add(1);
    println!("wrapping_add 255 + 1 = {x1}");
    println!("checked_add 255 + 1 = {:?}", x2);
    println!("overflowing_add 255 + 1 = {x3}, {b1}");
    println!("saturating_add 255 + 1 = {x4}");
}
