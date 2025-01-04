use pwmp_msg::version::Version;

#[test]
fn version_1() {
    assert_eq!(Version::parse("1.0.0"), Some(Version::new(1, 0, 0)));
}

#[test]
fn version_2() {
    assert_eq!(Version::parse("100.45.78"), Some(Version::new(100, 45, 78)));
}

#[test]
fn version_3() {
    assert_eq!(Version::parse("1...0"), None);
}
