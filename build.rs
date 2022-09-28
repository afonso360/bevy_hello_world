macro_rules! p {
    ($($tokens: tt)*) => {
        println!("cargo:warning={}", format!($($tokens)*))
    }
}

fn main () {
    p!("VS: {:?}", cc::windows_registry::find_vs_version());

    let link = cc::windows_registry::find_tool("x86_64-pc-windows-msvc", "link.exe");
    p!("link.exe: {:?}", link.map(|t| t.to_command()));
}
