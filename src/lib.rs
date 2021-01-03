/// Takes a message `&str` as input and
/// Prints <p style='color:red;'>🤦 ERROR : message</p> 
///
/// # Examples
///
/// ```
/// prettylog::error("This is an error message!");
/// ``` 
pub fn error(message: &str) {
    println!("🤦  \x1B[1;31mERROR :\x1B[0m \x1B[31m{}\x1B[0m", message);
}
