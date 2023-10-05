use crate::wave::heroes::tests::{test_1_vs_1, test_5_vs_5};

// constant &str to hero name
static HERO: &str = "Natalie";

#[test]
fn test_liz_1_vs_1() {
    test_1_vs_1(HERO);
}

#[test]
fn test_liz_5_vs_5() {
    test_5_vs_5(HERO);
}