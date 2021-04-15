mod tcp;


fn main() {
    let get_gopher = tcp::tcp::go_to("gopher.floodgap.com", 70, "/").unwrap();

    println!("{}", get_gopher);
}
