use std::fmt::Debug;

pub trait ClassPattern: Sized + Copy + Debug {
    fn matches(self, class: &str) -> bool;
}

impl<'a> ClassPattern for &'a str {
    fn matches(self, class: &str) -> bool {
        let matcher = wildmatch::WildMatch::new(self);
        matcher.matches(class)
    }
}

impl<'a, const N: usize> ClassPattern for [&'a str; N] {
    fn matches(self, class: &str) -> bool {
        ClassPattern::matches(&self[..], class)
    }
}

impl<'a, 'b> ClassPattern for &'a [&'b str] {
    fn matches(self, class: &str) -> bool {
        for &pattern in self.into_iter() {
            if ClassPattern::matches(pattern, class) {
                return true;
            }
        }
        false
    }
}

pub trait SubtypePattern: Sized + Copy + Debug {
    fn matches(self, subtype: Option<&str>) -> bool;
    fn to_schema(self) -> serde_json::Value;
}

impl<'a> SubtypePattern for &'a str {
    fn matches(self, subtype: Option<&str>) -> bool {
        if let Some(subtype) = subtype {
            let matcher = wildmatch::WildMatch::new(self);
            matcher.matches(subtype)
        } else {
            false
        }
    }
    fn to_schema(self) -> serde_json::Value {
        serde_json::json!({
            "type": "string",
            "enum": [self],
        })
    }
}
impl<'a, const N: usize> SubtypePattern for [&'a str; N] {
    fn matches(self, subtype: Option<&str>) -> bool {
        SubtypePattern::matches(&self[..], subtype)
    }
    fn to_schema(self) -> serde_json::Value {
        SubtypePattern::to_schema(&self[..])
    }
}

impl<'a, 'b> SubtypePattern for &'a [&'b str] {
    fn matches(self, subtype: Option<&str>) -> bool {
        if let Some(subtype) = subtype {
            for &pattern in self.into_iter() {
                if SubtypePattern::matches(pattern, Some(subtype)) {
                    return true;
                }
            }
        }
        false
    }
    fn to_schema(self) -> serde_json::Value {
        serde_json::json!({
            "type": "string",
            "enum": &self,
        })
    }
}

impl SubtypePattern for () {
    fn matches(self, subtype: Option<&str>) -> bool {
        subtype.is_none()
    }
    fn to_schema(self) -> serde_json::Value {
        serde_json::json!(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matches() {
        assert!(ClassPattern::matches("*", "Symbol"));
        assert!(ClassPattern::matches("Symbol*", "Symbol"));
        assert!(ClassPattern::matches("Symbol*", "Symbol"));
        assert!(ClassPattern::matches("Symbol*", "Symbol"));
        assert!(ClassPattern::matches("Symbol*", "Symbol"));
        assert!(ClassPattern::matches(["Symbol*", "Symbol"], "Symbol"));
        assert!(ClassPattern::matches(&["Symbol*", "Symbol"][..], "Symbol"));
    }
}
