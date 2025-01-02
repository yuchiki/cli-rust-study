use std::process::abort;

fn main() {
    if let  Err (e) =  wcr::get_args().and_then(wcr::run) {
        eprintln!("Error: {}", e);
        abort();
    }
}
