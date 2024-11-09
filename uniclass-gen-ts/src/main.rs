use std::fmt::Write;
fn main() {
    let mut s = String::new();
    writeln!(&mut s, "export const UNICLASS = {{").unwrap();
    for (uniclass, name) in &*uniclass_data::UNICLASS_TITLES {
        writeln!(&mut s, "  \"{uniclass}\": \"{name}\",").unwrap();
    }
    writeln!(&mut s, "}} as const;").unwrap();
    std::fs::write("uniclass.ts", s.as_bytes()).unwrap();
}
