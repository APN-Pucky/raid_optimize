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

use rand::Rng;

thread_local!(static LOG_STACK : std::cell::RefCell<usize> = std::cell::RefCell::new(0));
#[macro_export]
#[cfg(debug_assertions)]
macro_rules! indent {
    ($fun:block) => {{
        crate::LOG_STACK.with(|log_stack| {
            {
            let mut log_stack = log_stack.borrow_mut();
            *log_stack += 1;
            }
            let this_is_what_might_get_returned = {
                $fun
            };
            {
            let mut log_stack = log_stack.borrow_mut();
            *log_stack -= 1;
            }
            this_is_what_might_get_returned
        })
    }};
}
#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! indent {
    ($fun:block) => {$fun}
}

#[macro_export]
#[cfg(debug_assertions)]
macro_rules! debug {
    ($($arg:tt)*) => {{
        crate::LOG_STACK.with(|log_stack| {
            let log_stack = log_stack.borrow();
            let indent = " ".repeat(*log_stack);
            log::debug!(target: "", "{}{}", indent, format!($($arg)*));
        })
    }};
}
#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! debug {
    ($($arg:tt)*) => {}
}

#[macro_export]
macro_rules! warn{
    ($($arg:tt)*) => {{
        crate::LOG_STACK.with(|log_stack| {
            let log_stack = log_stack.borrow();
            let indent = " ".repeat(*log_stack);
            log::warn!(target: "","{}{}", indent, format!($($arg)*));
        })
    }};
}

#[macro_export]
macro_rules! error{
    ($($arg:tt)*) => {{
        crate::LOG_STACK.with(|log_stack| {
            let log_stack = log_stack.borrow();
            let indent = " ".repeat(*log_stack);
            log::error!(target: "","{}{}", indent, format!($($arg)*));
        })
    }};
}

#[macro_export]
macro_rules! info{
    ($($arg:tt)*) => {{
        crate::LOG_STACK.with(|log_stack| {
            let log_stack = log_stack.borrow();
            let indent = " ".repeat(*log_stack);
            log::info!(target: "","{}{}", indent, format!($($arg)*));
        })
    }};
}

#[inline]
pub fn roll(chance:f32) -> bool {
    if chance >= 1.0 {
        return true
    }
    else if chance <= 0.0 {
        return false
    }
    else {
        let mut rng = rand::thread_rng();
        rng.gen::<f32>() < chance
    }
}
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
