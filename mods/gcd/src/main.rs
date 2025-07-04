#[link(wasm_import_module = "host")]
unsafe extern "C" {
    fn host_log(val: i32);
}

fn log(val: i32) {
    unsafe { host_log(val) }
}

#[unsafe(export_name = "gcd")]
pub fn gcd(m: u32, n: u32) -> u32 {
    let mut a = m;
    let mut b = n;

    while b != 0 {
        (a, b) = (b, a % b)
    }

    //println!("gcd({}, {}) = {}", m, n, a);
    log(100);
    a
}

fn main() {
    println!("Hello, world! Please call gcd(10, 5) to see the result.");
}
