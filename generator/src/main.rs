use generate::payloads::*;
use generate::Code;

const REQUEST_FOLDER: &str = "{{project-name}}/src/restful/api";

fn main() {
    Asset::write(REQUEST_FOLDER, None);
}
