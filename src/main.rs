use crate::errors::Handler;

mod tcp;
mod gopher;
mod errors;


fn main() {
    let get_gopher = tcp::Request::go_to("gopher://gopher.floodgap.com", 70, "/").unwrap();

    let parser = gopher::Parser::new(&get_gopher);
    println!("{:#?}", parser);
}
