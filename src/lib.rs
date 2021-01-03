//! # prettylog
//!
//! `prettylog` is a collection of functions to print log messages to terminal
//! with emojis and colors 😍
//!

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

/// Takes a message `&str` as input and
/// Prints <p style='color:green;'>💁 INFO : message</p>
///
/// # Examples
///
/// ```
/// prettylog::info("This is an infomation!");
/// ```
pub fn info(message: &str) {
    println!("💁  \x1B[1;32mINFO :\x1B[0m \x1B[32m{}\x1B[0m", message);
}

/// Takes a message `&str` as input and
/// Prints <p style='color:yellow;'>⚠️ WARN : message</p>
///
/// # Examples
///
/// ```
/// prettylog::warn("This is a warning!");
/// ```
pub fn warn(message: &str) {
    println!("⚠️  \x1B[1;33mWARN :\x1B[0m \x1B[33m{}\x1B[0m", message);
}

/// Takes a message `&str` as input and
/// Prints <p style='color:blue;'>🙄 WAIT : message</p>
///
/// # Examples
///
/// ```
/// prettylog::wait("Compiling...");
/// ```
pub fn wait(message: &str) {
    println!("🙄  \x1B[1;34mWAIT :\x1B[0m \x1B[34m{}\x1B[0m", message);
}

/// Takes a message `&str` as input and
/// Prints <p style='color:purple;'>🚨 CRITICAL : message</p>
///
/// # Examples
///
/// ```
/// prettylog::critical("Are you sure, you want to remove the file ?");
/// ```
pub fn critical(message: &str) {
    println!("🚨  \x1B[1;35mCRITICAL :\x1B[0m \x1B[35m{}\x1B[0m", message);
}

/// Takes a message `&str` as input and
/// Prints <p style='color:cyan;'>⚡ CMD : message</p>
///
/// # Examples
///
/// ```
/// prettylog::command("echo $HOME");
/// ```
pub fn command(message: &str) {
    println!("⚡  \x1B[1;36mCMD :\x1B[0m \x1B[36m{}\x1B[0m", message);
}

/// Takes a message `&str` as input and
/// Prints <p style='color:blue;'>🔗  LINK : </p> <p style='color:blue;text-decoration:underline'>message</p>
///
/// # Examples
///
/// ```
/// prettylog::link("echo $HOME");
/// ```
pub fn link(message: &str) {
    println!("🔗  \x1B[1;34mLINK :\x1B[0m \x1B[4;34m{}\x1B[0m", message);
}

/// Takes a message `&str` as input and
/// Prints <p style='color:grey;'>😴  MISC : message</p>
///
/// # Examples
///
/// ```
/// prettylog::misc("echo $HOME");
/// ```
pub fn misc(message: &str) {
    println!("😴  \x1B[2;37mMISC :\x1B[2;37m{}\x1B[0m", message);
}
