pub fn mango_b(m: String) {
    let will_be_captured_by_mem_guard = 1_500_000;
    let vec: Vec<u8> = vec![0; will_be_captured_by_mem_guard];
    println!("mango_b {}", m);
}

#[cfg(test)]
pub mod test;

