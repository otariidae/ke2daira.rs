const SUTEKANA: [char; 9] = ['ァ', 'ィ', 'ゥ', 'ェ', 'ォ', 'ャ', 'ュ', 'ョ', 'ヮ'];

/// convert a Katakana string to a list of morae
///
/// precondition: the input string must be all in Katakana
pub fn katakana_to_mora(kana: &str) -> Vec<String> {
    let mut morae = Vec::new();
    for c in kana.chars() {
        if SUTEKANA.contains(&c) {
            let previous = match morae.pop() {
                Some(mora) => mora,
                None => {
                    morae.push(c.to_string());
                    continue;
                }
            };
            let combined = format!("{}{}", previous, c);
            morae.push(combined);
        } else {
            morae.push(c.to_string());
        }
    }
    morae
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_katakana_to_mora() {
        assert_eq!(katakana_to_mora("サル"), vec!["サ", "ル"]);
        assert_eq!(katakana_to_mora("カッパ"), vec!["カ", "ッ", "パ"]);
        assert_eq!(
            katakana_to_mora("チョコレート"),
            vec!["チョ", "コ", "レ", "ー", "ト"]
        );
        assert_eq!(
            katakana_to_mora("ガッキュウシンブン"),
            vec!["ガ", "ッ", "キュ", "ウ", "シ", "ン", "ブ", "ン"]
        );
    }
}
