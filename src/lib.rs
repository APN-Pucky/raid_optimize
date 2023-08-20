#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_xml_rs;
extern crate rand;
extern crate log;
extern crate enum_map;
extern crate enum_map_derive;

pub mod hero;
pub mod sim;
pub mod wave;
pub mod player;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

/// # Examples
///
/// ```
/// raid_optimize::test();
/// ```
pub fn test() {
    println!("Test");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
