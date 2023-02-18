
#[test]
fn email_test() {
    let cases: Vec<(&str, bool)> = vec![
        ("simple@example.com", true),
        ("very.common@example.com", true),
        ("disposable.style.email.with+symbol@example.com", true),
        ("other.email-with-hyphen@example.com", true),
        ("fully-qualified-domain@example.com", true),
        ("user.name+tag+sorting@example.com", true),
        ("x@example.com", true),
        ("example-indeed@strange-example.com", true),
        ("test/test@test.com", true),
        ("test", false),
        ("test.com", false),
        ("@test.com", false),
        (".a@test.com", false),
        ("a.@test.com", false),
        ("a..a@test.com", false),
        ("a@test", false),
        ("A@b@c@example.com", false),
        ("a\"b(c)d,e:f;g<h>i[j\\k]l@example.com", false),
        ("just\"not\"right@example.com", false),
        ("this is\"not\\allowed@example.com", false),
        (
            "1234567890123456789012345678901234567890123456789012345678901234+x@example.com",
            false,
        ),
    ];

    for case in cases.iter() {
        assert_eq!(dator::email(case.0), case.1, "Failed case: {}", case.0);
    }
}
