fn main() {
    // the IUP version detected: e.g. v3_29;
    if let Ok(cfg_viup) = std::env::var("DEP_IUP_IUPVERSION") {
        println!("cargo:rustc-cfg={}", cfg_viup);
    }
    // the CD version detected: e.g. cdv5_13
    if std::env::var("DEP_IUP_CDVERSION").is_ok() {
        println!("cargo:rustc-cfg=installedCD");
    }
    // the IM version detected: e.g. imv3_14 or installedIM
    if std::env::var("DEP_IUP_IMVERSION").is_ok() {
        println!("cargo:rustc-cfg=installedIM");
    }
    // the Webkit version detected on *nix: e.g. webv2_4
    if std::env::var("DEP_IUP_WEBVERSION").is_ok() {
        println!("cargo:rustc-cfg=installedWebkit");
    }
    // the FTGL "variant" detected: IUP: it's from IUP, otherwise not set
    if std::env::var("DEP_IUP_FTGLVERSION").is_ok() {
        println!("cargo:rustc-cfg=installedFTGL");
    }
}
