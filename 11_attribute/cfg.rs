//link: http://doc.rust-lang.org/reference.html#conditional-compilation

/*
target_arch = "...". Target CPU architecture, such as "x86", "x86_64" "mips", "powerpc", "arm", or "aarch64".
target_endian = "...". Endianness of the target CPU, either "little" or "big".
target_family = "...". Operating system family of the target, e. g. "unix" or "windows". The value of this configuration option is defined as a configuration itself, like unix or windows.
target_os = "...". Operating system of the target, examples include "win32", "macos", "linux", "android", "freebsd", "dragonfly", "bitrig" or "openbsd".
target_pointer_width = "...". Target pointer width in bits. This is set to "32" for targets with 32-bit pointers, and likewise set to "64" for 64-bit pointers.
unix. See target_family.
windows. See target_family.
*/

// This function only gets compiled if the target OS is linux
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux!")
}

// And this function only gets compiled if the target OS is *not* linux
#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are *not* running linux!")
}

fn main() {
    are_you_on_linux();
}

