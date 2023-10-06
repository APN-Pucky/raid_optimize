use crate::wave::heroes::tests::{test_1_vs_1, test_5_vs_5};

// constant &str to hero name
static HERO: &str = "Maya";

#[test]
fn test_maya_1_vs_1() {
    test_1_vs_1(HERO);
}

#[test]
fn test_maya_5_vs_5() {
    test_5_vs_5(HERO);
}