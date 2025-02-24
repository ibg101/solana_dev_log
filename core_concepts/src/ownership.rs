// Explanation here:
// https://t.me/solana_dev_log/6

#[allow(dead_code, unused_variables)]
pub fn example() {
    // Heap-allocated data (String) - moved, not copied
    let heap_data: String = String::from("I'm on the heap!");
    consumer(heap_data); // Ownership transferred
    
    // ERROR: Uncomment to see compiler's ownership error
    // consumer(heap_data); 
    
    // Stack-allocated data (u32) - copied automatically
    let stack_data: u32 = 42;
    copy_consumer(stack_data); // Copy allowed
    copy_consumer(stack_data); // Works again!
}

fn consumer(s: String) {
    println!("Consumed: {}", s);
} // `s` dropped here

fn copy_consumer(n: u32) {
    println!("Copied: {}", n);
} // No drop needed (stack data)
