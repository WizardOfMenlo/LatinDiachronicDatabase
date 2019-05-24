use query_driver::driver_init;

fn main() {
    let _db = driver_init("./data/works/", "./data/out.txt").unwrap();
}
