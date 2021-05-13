//! A small example to run on your computer to see what locale the library returns.

use sys_locale::get_locale;

fn main() {
    let locale = get_locale().unwrap_or_else(|| String::from("en-US"));

    println!("The current locale is {}", locale);
}
