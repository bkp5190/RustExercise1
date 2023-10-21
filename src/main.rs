fn main() {
    let mut missiles: i32 = 8;

    let ready: i32 = 2;

    println!("Firing {} of my {} missiles...", ready, missiles);

    missiles = missiles - ready;

    println!("{} missiles left", missiles);
}
