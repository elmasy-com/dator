use std::net::Ipv6Addr;

/// Checks whether the given string is a valid IPv4 address.
///
/// # Example
///
///
/// ```
/// if dator::ipv4("0.0.0.0") {
///     // 0.0.0.0 is a valid IPv4
/// }
/// ```
///
/// ```
/// if !dator::ipv4("fe80::") {
///     // fe80:: is an invalid IPv4
/// }
/// ```
pub fn ipv4(ip: &str) -> bool {
    // IP string size is 7 <= ip.len() <= 15
    if ip.len() < 7 || ip.len() > 15 {
        return false;
    }

    // Number of parts
    let mut n_parts: u8 = 0;

    for part in ip.split('.') {
        //print!("part[{}]={}", n_dots, part);

        if let Err(_) = str::parse::<u8>(part) {
            return false;
        }

        // Stop the check, ip is 100% invalid
        if n_parts > 4 {
            return false;
        }

        n_parts += 1;
    }

    n_parts == 4
}

/// Checks whether the given string is a valid IPv6 address.
///
/// # Example
///
///
/// ```
/// if dator::ipv6("fe80::") {
///     // "fe80::" is a valid IPv6
/// }
/// ```
///
/// ```
/// if !dator::ipv6("1.1.1.1") {
///     // 1.1.1.1 is an invalid IPv6
/// }
/// ```
pub fn ipv6(ip: &str) -> bool {
    if ip.len() < 2 || ip.len() > 39 || !ip.contains(':') {
        return false;
    }

    ip.parse::<Ipv6Addr>().is_ok()
}

/// Checks whether the given string is a valid IP address.
///
/// Both IPv4 and IPv6 returned as valid.
///
/// # Example
///
/// ```
/// if dator::ip("1.1.1.1") {
///     // 1.1.1.1 is a valid IP
/// }
/// ```
/// ```
/// if dator::ip("fe80::") {
///     // "fe80::" is a valid IP
/// }
/// ```
/// ```
/// if !dator::ip("example.com") {
///     // example.com is not a valid IP
/// }
/// ```
pub fn ip(ip: &str) -> bool {
    ipv4(ip) || ipv6(ip)
}
