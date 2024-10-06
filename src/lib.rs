use lindera::{DictionaryConfig, DictionaryKind, Mode, Token, Tokenizer, TokenizerConfig};

mod kana2mora;

fn is_katanaka(c: char) -> bool {
    // ascii is not Katakana
    if c.is_ascii() {
        return false;
    }
    // Katakana range
    if !('ァ'..='ヶ').contains(&c) && c != 'ー' {
        return false;
    }
    true
}

fn is_katanaka_str(s: &str) -> bool {
    // s is Katakan if all its characters are Katakana
    s.chars().all(is_katanaka)
}

fn build_reading_from_tokens(tokens: &mut [Token]) -> Option<String> {
    let mut reading = String::new();

    for token in tokens {
        let details = match token.get_details() {
            Some(details) => details,
            None => continue,
        };
        if details[0] == "UNK" {
            continue;
        }
        let token_reading = details[7];
        reading += token_reading;
    }

    if reading.is_empty() {
        None
    } else {
        Some(reading)
    }
}

/// convert a word to its Katakana reading
fn to_katakana_reading(word: &str) -> Option<String> {
    // if the word is already Katakana, return it as is
    if is_katanaka_str(word) {
        return Some(word.to_string());
    }

    // otherwise, transform the word to Katakana using morphological analysis
    let dictionary = DictionaryConfig {
        kind: Some(DictionaryKind::IPADIC),
        path: None,
    };
    let config = TokenizerConfig {
        dictionary,
        user_dictionary: None,
        mode: Mode::Normal,
    };

    let Ok(tokenizer) = Tokenizer::from_config(config) else {
        return None;
    };
    let Ok(mut tokens) = tokenizer.tokenize(word) else {
        return None;
    };

    build_reading_from_tokens(&mut tokens)
}

/// Swap the first mora of the first name with the first mora of the last name
/// keeping middle names in place
///
/// precondition: the input names must be all in Katakana
fn swap_names_head(names: &mut Vec<String>) -> Result<&Vec<String>, &str> {
    // nothing to swap with mononyms
    if names.len() < 2 {
        return Ok(names);
    }

    let Some(first_name) = names.first() else {
        return Err("No first name");
    };
    let Some(last_name) = names.last() else {
        return Err("No last name");
    };

    let mut first_name_morae = kana2mora::katakana_to_mora(first_name);
    let mut last_name_morae = kana2mora::katakana_to_mora(last_name);

    let first_mora_first_name = first_name_morae.first_mut().unwrap();
    let first_mora_last_name = last_name_morae.first_mut().unwrap();

    std::mem::swap(first_mora_first_name, first_mora_last_name);

    let swapped_first_name = first_name_morae.concat();
    let swapped_last_name = last_name_morae.concat();

    let last_index = names.len() - 1;
    names[0] = swapped_first_name;
    names[last_index] = swapped_last_name;

    Ok(names)
}

/// transform a name to the Ke2daira style
/// 
/// # Examples
/// 
/// ```
/// assert_eq!(ke2daira::ke2daira("松平 健"), Some("ケツダイラ マン".to_string()));
/// ```
pub fn ke2daira(raw_name: &str) -> Option<String> {
    let names = raw_name.split_whitespace().collect::<Vec<_>>();
    let mut names = names
        .iter()
        .map(|name| to_katakana_reading(name))
        .collect::<Option<Vec<_>>>()?;
    let Ok(names) = swap_names_head(&mut names) else {
        return None;
    };
    let name = names.join(" ");
    Some(name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ke2daira() {
        assert_eq!(ke2daira("松平 健"), Some("ケツダイラ マン".to_string()));
        assert_eq!(
            ke2daira("ハリー ジェームズ ポッター"),
            Some("ポリー ジェームズ ハッター".to_string())
        );
        assert_eq!(ke2daira("チェ ゲバラ"), Some("ゲ チェバラ".to_string()));
        assert_eq!(ke2daira("スカルノ"), Some("スカルノ".to_string()));

        // ghost character that cannot be converted to Katakana
        assert_eq!(ke2daira("彁"), None);
    }
}
