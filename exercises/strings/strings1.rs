// strings1.rs
//
// Make me compile without changing the function signature!
//
// Execute `rustlings hint strings1` or use the `hint` watch subcommand for a
// hint.


fn main() {
    // let timestamp = std::time::SystemTime::now()
    //         .duration_since(std::time::UNIX_EPOCH)
    //         .unwrap()
    //         .as_secs();
    //     let your_command = format!(
    //         "rustc-env=TEST_FOO={}",
    //         timestamp
    //     );
    //     println!("cargo:{}", your_command);
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);
}

fn current_favorite_color() -> String {
    "blue".to_string()
}
