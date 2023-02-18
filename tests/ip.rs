
#[test]
fn ipv4_test() {
    let cases: Vec<(&str, bool)> = vec![("1.1.1.1", true), ("1.1.1", false), ("1.1.1.256", false)];

    for case in cases.iter() {
        assert_eq!(dator::ipv4(case.0), case.1);
    }
}

#[test]
fn ipv6_test() {
    let cases: Vec<(&str, bool)> = vec![("::8888", true), ("8888", false), (":88888", false)];

    for case in cases.iter() {
        assert_eq!(dator::ipv6(case.0), case.1);
    }
}

#[test]
fn ip_test() {
    let cases: Vec<(&str, bool)> = vec![
        ("1.1.1.1", true),
        ("1.1.1", false),
        ("1.1.1.256", false),
        ("::8888", true),
        ("8888", false),
        (":88888", false),
    ];

    for case in cases.iter() {
        assert_eq!(dator::ip(case.0), case.1, "Failed at case {}", case.0);
    }
}
