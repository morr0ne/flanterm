fn main() {
    cc::Build::new()
        .file("flanterm/flanterm.c")
        .file("flanterm/backends/fb.c")
        .flag("-std=c11")
        .flag("-ffreestanding")
        .flag("-fno-stack-protector")
        .flag("-fno-stack-check")
        .flag("-mno-red-zone")
        .compile("flanterm")
}
