use query_driver::driver_init;

fn main() {
    let db = driver_init("./data/works/", "./data/out.txt").unwrap();
}
