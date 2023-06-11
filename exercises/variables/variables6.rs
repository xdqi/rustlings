// variables6.rs
// Execute `rustlings hint variables6` or use the `hint` watch subcommand for a hint.



static mut NUMBER :i32= 3;
fn main() {
    println!("Number {}", unsafe{ NUMBER});
}
