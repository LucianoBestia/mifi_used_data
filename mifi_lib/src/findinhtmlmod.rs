//! findinhtmlmod.re

/// find ul and dl mb in u32
pub fn find(resp_str: &str) -> (u32, u32) {
    // find
    let (sentp, cursor_pos) = find_between(resp_str, r#"<label id="lsentPackets">"#, "</label>", 0);
    //println!("{:#?}", sentp);
    let i_sentp = parse_mb(sentp);
    //println!("{:#?}", i_sentp);
    let (recp, _) = find_between(
        resp_str,
        r#"<label id="lRecPackets">"#,
        "</label>",
        cursor_pos,
    );
    //println!("{:#?}", recp);
    let i_recp = parse_mb(recp);
    //println!("{:#?}", i_recp);
    //return
    (i_sentp, i_recp)
}
/// parse the MB or GB numbers and return u32 MB
pub fn parse_mb(source_str: &str) -> u32 {
    //region: allowed only this format 629.74 MB,  3.17 GB
    use lazy_static::lazy_static;
    use regex::Regex;
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^[0-9]*\.[0-9]{2} [MG]B$").unwrap();
    }
    assert!(RE.is_match(source_str));
    //endregion

    let mb: f64;
    if source_str.ends_with("MB") {
        mb = source_str[..source_str.len() - 3].parse::<f64>().unwrap();
    } else if source_str.ends_with("GB") {
        mb = source_str[..source_str.len() - 3].parse::<f64>().unwrap() * 1000.0;
    } else {
        mb = 0.0;
    }
    //return
    mb.round() as u32
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
    fn parse_mb_01() {
        assert_eq!(parse_mb("629.74 MB"), 630);
        assert_eq!(parse_mb("3.17 GB"), 3170);
    }
    #[test]
    #[should_panic]
    fn parse_mb_panic_01() {
        parse_mb("123");
    }
    #[test]
    #[should_panic]
    fn parse_mb_panic_02() {
        parse_mb("123.99MB");
    }
    #[test]
    #[should_panic]
    fn parse_mb_panic_03() {
        parse_mb("629.74 MB ");
    }
    #[test]
    #[should_panic]
    fn parse_mb_panic_04() {
        parse_mb(" 629.74 MB");
    }
    #[test]
    fn find_between_01() {
        let a = "shdjeiwu<x>lalal</x>jshdkfjhd";
        let (s, p) = find_between(a, "<x>", "</x>", 0);
        assert_eq!(s, "lalal");
        assert_eq!(p, 20);
    }
    #[test]
    fn find_between_02() {
        let a = "shdjeiwu<x>lalal</x>jshdkfjhd";
        let (s, p) = find_between(a, "<x>", "</x>", 8);
        assert_eq!(s, "lalal");
        assert_eq!(p, 20);
    }
    #[test]
    fn find_between_03() {
        //the <x> is at position 8. So pos 9 cannot find it.
        //returns empty string and same cursor_pos.
        let a = "shdjeiwu<x>lalal</x>jshdkfjhd";
        let (s, p) = find_between(a, "<x>", "</x>", 9);
        assert_eq!(s, "");
        assert_eq!(p, 9);
    }
    #[test]
    fn find_between_04() {
        let a = "<x>lalal</x>";
        let (s, p) = find_between(a, "<x>", "</x>", 0);
        assert_eq!(s, "lalal");
        assert_eq!(p, 12);
    }
}
