fn check(s: &str) -> bool {
    if s.contains("cie") {
            return false
        } else if s.matches("ei").count() != s.matches("cei").count() {
            return false
        } else {
            return true
        };
}

fn main() {
    assert_eq!(check("a"), true);
    assert_eq!(check("zombie"), true);
    assert_eq!(check("transceiver"), true);
    assert_eq!(check("veil"), false);
    assert_eq!(check("icier"), false);
}