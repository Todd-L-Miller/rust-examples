fn main() {
    let mut memory: [u8; 65536] = [0; 65536];
    memory[0] = 1;
    println!("{}", memory[0]);
    store(& mut memory, 0, 7);
    println!("{}", memory[0]);
}

fn store(memory: &mut [u8], offset: usize, value: u8 ) {
    memory[offset] = value;
}
