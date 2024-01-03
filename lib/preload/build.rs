fn main() {
    cc::Build::new()
        .file("src/shim.c")
        .compile("shim");
}
