use generate::payloads::*;
use generate::Code;

const REQUEST_FOLDER: &str = "simple/src/restful/api";

fn main() {
    Asset::write(REQUEST_FOLDER, None);
}
