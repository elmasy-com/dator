/// Checks whether the given string is a valid domain.
///
/// The only difference from the RFC specification is that the string must have a TLD (eg.: com)
/// and a domain (eg.: example).
/// So the root domain (`.`) and the TLD only domains are reported as invalid.
///
/// # Example
///
///
/// ```
/// if dator::domain("example.com") {
///     // example.com is a valid domain
/// }
/// ```
///
/// ```
/// if !dator::domain(".com") {
///     // .com is an invalid domain
/// }
/// ```
pub fn domain(d: &str) -> bool {
    let b = d.as_bytes();
    let l = b.len();

    // The minimum accepted domain is "a.a".
    if l < 3 {
        return false;
    }

    // The maximum number of allowed characters is 255 (254 + '\0')
    if l > 254 {
        return false;
    }

    // If length is 254, than the domain must be a fully qualified domain name, thus must ends with a '.'
    if l == 254 && b[l - 1] != b'.' {
        return false;
    }

    // Valid domains are not starts with a dot.
    if b[0] == b'.' {
        return false;
    }

    let mut num_dots = 0;
    let mut last = b'.';
    let mut non_numeric = false; // true once we've seen a letter or hyphen
    let mut part_len = 0;

    for c in b.iter() {
        match c {
            b'-' => {
                // Byte before dash cannot be dot
                if last == b'.' {
                    return false;
                }
                part_len += 1;
                non_numeric = true;
            }
            b'.' => {
                // Two dot not allowed after each other
                if last == b'.' {
                    return false;
                }

                // A dot cant come after a dash
                if last == b'-' {
                    return false;
                }

                // One part of the domain can be 63 characters long maximum
                if part_len > 63 {
                    return false;
                }

                if part_len == 0 {
                    return false;
                }
                num_dots += 1;
                part_len = 0;
            }
            b'0'..=b'9' => {
                part_len += 1;
            }
            b'A'..=b'Z' => {
                non_numeric = true;
                part_len += 1;
            }
            b'_' => {
                non_numeric = true;
                part_len += 1;
            }
            b'a'..=b'z' => {
                non_numeric = true;
                part_len += 1;
            }
            // invalid character
            _ => {
                return false;
            }
        }

        last = *c;
    }

    // Last byte cant be dash.
    if last == b'-' {
        return false;
    }
    // Local part too big
    if part_len > 63 {
        return false;
    }
    // The only dot is the last character (eg.: "aa.")
    if last == b'.' && num_dots == 1 {
        return false;
    }

    non_numeric && num_dots > 0
}
