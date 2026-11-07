fn main() {
    cc::Build::new()
        .file("csrc/cosmic_usb.c")
        .include("csrc")
        .flag("-Wall")
        .flag("-Wextra")
        .flag("-Wno-unused-parameter")
        .compile("cosmic_usb");
}
