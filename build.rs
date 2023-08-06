use std::process::exit;

fn main() {
    let mut features_enabled = vec![];

    if cfg!(feature = "mime_guess") {
        features_enabled.push("mime_guess");
    }

    if cfg!(feature = "infer") {
        features_enabled.push("infer");
    }

    if features_enabled.len() != 1 {
        eprintln!(
            "Error: Please enable exactly one feature out of: mime_guess, infer. Currently enabled features: {:?}",
            features_enabled
        );
        exit(1);
    }

    // Set the feature flag for the selected feature
        if cfg!(feature = "mime_guess") {
            println!("cargo:rustc-cfg=feature=\"mime_guess\"");
        }

    if cfg!(feature = "infer") {
        println!("cargo:rustc-cfg=feature=\"infer\"");
    }
}
