/// parse the MB or GB numbers
pub fn parse_mb(source_str: &str) -> f64 {
    // allowed only this format 629.74 MB,  3.17 GB
    use lazy_static::lazy_static;
    use regex::Regex;
    lazy_static! {
        static ref RE: Regex = Regex::new(r"[0-9]*\.[0-9]{2} [MG]B").unwrap();
    }
    assert!(RE.is_match(source_str));

    let mb: f64;
    if source_str.contains("MB") {
        mb = source_str[..source_str.len() - 3].parse::<f64>().unwrap();
    } else if source_str.contains("GB") {
        mb = source_str[..source_str.len() - 3].parse::<f64>().unwrap() * 1000.0;
    } else {
        mb = 0.0;
    }
    //return
    mb
}

/// find and return the first occurrence between start and end delimiters
/// returns also the cursor position, so the next find will continue from there
/// returns empty string if no found and same cursor_pos
pub fn find_between<'a>(
    source_str: &'a str,
    start_delimiter: &str,
    end_delimiter: &str,
    start_cursor_pos: usize,
) -> (&'a str, usize) {
    let mut sentp = "";
    let mut cursor_pos = start_cursor_pos;
    if let Some(pos1) = source_str[start_cursor_pos..].find(start_delimiter) {
        cursor_pos += pos1;
        if let Some(pos2) = source_str[start_cursor_pos + pos1..].find(end_delimiter) {
            cursor_pos += pos2 + end_delimiter.len();
            sentp = &source_str
                [start_cursor_pos + pos1 + start_delimiter.len()..start_cursor_pos + pos1 + pos2];
        }
    }
    //return
    (sentp, cursor_pos)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_mb() {
        assert_eq!(parse_mb("629.74 MB"), 629.74);
        assert_eq!(parse_mb("3.17 GB"), 3170.0);
    }
    #[test]
    #[should_panic]
    fn test_parse_mb_panic() {
        parse_mb("123");
    }
    #[test]
    #[should_panic]
    fn test_parse_mb_panic2() {
        parse_mb("123.99MB");
    }
    #[test]
    #[should_panic]
    fn test_parse_mb_panic3() {
        parse_mb("629.74 MB ");
    }
    #[test]
    #[should_panic]
    fn test_parse_mb_panic4() {
        parse_mb(" 629.74 MB");
    }
    #[test]
    fn test_find_between() {
        let a = "shdjeiwu<x>lalal</x>jshdkfjhd";
        let (s, p) = find_between(a, "<x>", "</x>", 0);
        assert_eq!(s, "lalal");
        assert_eq!(p, 20);
    }
    #[test]
    fn test_find_between2() {
        let a = "shdjeiwu<x>lalal</x>jshdkfjhd";
        let (s, p) = find_between(a, "<x>", "</x>", 8);
        assert_eq!(s, "lalal");
        assert_eq!(p, 20);
    }
    #[test]
    fn test_find_between3() {
        //the <x> is at position 8. So pos 9 cannot find it.
        //returns empty string and same cursor_pos.
        let a = "shdjeiwu<x>lalal</x>jshdkfjhd";
        let (s, p) = find_between(a, "<x>", "</x>", 9);
        assert_eq!(s, "");
        assert_eq!(p, 9);
    }
    #[test]
    fn test_find_between4() {
        let a = "<x>lalal</x>";
        let (s, p) = find_between(a, "<x>", "</x>", 0);
        assert_eq!(s, "lalal");
        assert_eq!(p, 12);
    }
}
