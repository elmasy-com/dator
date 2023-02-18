// Validate the local part of the email (the part before the '@').
fn email_local(s: &str) -> bool {
    let s = s.as_bytes();

    if s.len() == 0 || s.len() > 64 {
        return false;
    }

    let mut last: u8 = 46;

    for c in s.iter() {
        match c {
            // Special chars
            b'#' | b'$' | b'%' | b'&' | b'\'' | b'*' | b'+' | b'-' | b'/' | b'=' | b'?' => {
                last = *c;
            }
            b'^' | b'_' | b'`' | b'{' | b'|' | b'}' | b'~' => {
                last = *c;
            }
            b'0'..=b'9' => {
                last = *c;
            }
            b'A'..=b'Z' => {
                last = *c;
            }
            b'a'..=b'z' => {
                last = *c;
            }
            b'.' => {
                // local part cant start with dot,
                // or contains two in a row.
                if last == b'.' {
                    return false;
                }
                last = *c;
            }
            // Any invalid character
            _ => {
                return false;
            }
        }
    }

    // Last character cant be dot.
    last != b'.'
}

/// Checks whether the given string is a valid email.
///
/// This function not allows IP addresses in the domain part (eg.: `bob@[127.0.0.1]`).
///
/// # Example
///
/// ```
/// if dator::email("bob@example.com") {
///     // bob@example.com is a valid email address
/// }
/// ```
/// ```
/// if !dator::email("bob@[0.0.0.0]") {
///     // bob@[0.0.0.0] is a valid email address
/// }
/// ```
pub fn email(s: &str) -> bool {
    let parts: Vec<&str> = s.split('@').collect();

    if parts.len() != 2 {
        return false;
    }

    // Check local part
    match parts.get(0) {
        Some(part) => {
            if !email_local(part) {
                return false;
            }
        }
        None => {
            return false;
        }
    }

    // Check domain part
    match parts.get(1) {
        Some(part) => {
            if !crate::domain(part) {
                return false;
            }
        }
        None => {
            return false;
        }
    }

    true
}
