use std::env;

fn main() {
    let enabled_assert_levels = cfg!(feature = "assert-level-disabled") as usize
        + cfg!(feature = "assert-level-release") as usize
        + cfg!(feature = "assert-level-debug") as usize
        + cfg!(feature = "assert-level-paranoid") as usize;
    if enabled_assert_levels == 0 {
        if env::var("DEBUG").unwrap() == "true" {
            println!(r#"cargo:rustc-cfg=feature="assert-level-debug""#);
        } else {
            println!(r#"cargo:rustc-cfg=feature="assert-level-release""#);
        }
    }
}
