extern crate embed_resource;
use std::env;

fn main() {
    let target = env::var("TARGET").expect("Failed to read env var TARGET");
    if target.contains("windows") {
        // on windows we will set our game icon as icon for the executable
        embed_resource::compile("build/windows/icon.rc", embed_resource::NONE);
    }
}
