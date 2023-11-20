pub use json_rust_parser::*;
pub use json_rust_parser::JSONValue;
use anyhow::anyhow;


#[cfg(test)]
mod deserialize_tests {
    use anyhow::Ok;
    use json_rust_parser::JSONParser;
    use pest::Parser;
    

    use super::parse_json_file;
    use super::JSONValue::*;
    use super::*;

    #[test]
    fn deserialize_array() {
        assert_eq!(parse_json_file("[]").unwrap(), Array(vec![]));
        assert_eq!(
            parse_json_file("[null, true, 1, \"test\"]").unwrap(),
            Array(vec![Null, Boolean(true), Number(1.0), String("test")])
        );
        assert_eq!(
            parse_json_file("[[[]]]").unwrap(),
            Array(vec![Array(vec![Array(vec![])])])
        );
        assert!(parse_json_file("[1,]").is_err());
    }

    #[test]
    fn deserialize_object() {
        assert_eq!(parse_json_file("{}").unwrap(), Object(vec![]));
        assert_eq!(
            parse_json_file(
                r#"{
                    "key": "value",
                    "num": 100,
                    "bool": false,
                    "null": null,
                    "arr": ["str", 1.5e+10]
                }"#
            )
            .unwrap(),
            Object(vec![
                ("key", String("value")),
                ("num", Number(100.0)),
                ("bool", Boolean(false)),
                ("null", Null),
                ("arr", Array(vec![String("str"), Number(1.5e+10)]))
            ])
        );
        assert!(parse_json_file("{\"n\":1,}").is_err());
    }
    
    #[test]
    fn deserialize_null() -> anyhow::Result<()> {

        let pair = JSONParser::parse(Rule::null, "null")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
        assert_eq!(pair.as_str(), "null");
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 4);

        let pair = JSONParser::parse(Rule::null, " null ");
        assert!(pair.is_err());

        let pair = JSONParser::parse(Rule::null, "NULL");
        assert!(pair.is_err());

        let pair = JSONParser::parse(Rule::null, "Null");
        assert!(pair.is_err());

        Ok(())
    }

    #[test]
    fn deserialize_boolean() -> anyhow::Result<()> {
        let pair = JSONParser::parse(Rule::boolean, "true")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
        assert_eq!(pair.as_str(), "true");
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 4);
        
        let pair = JSONParser::parse(Rule::boolean, "false")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
        assert_eq!(pair.as_str(), "false");
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 5);

        let pair = JSONParser::parse(Rule::boolean, "TRUE");
        assert!(pair.is_err());

        let pair = JSONParser::parse(Rule::boolean, "False");
        assert!(pair.is_err());

        Ok(())
    }

    #[test]
    fn deserialize_number() -> anyhow::Result<()> {
        // Test valid numbers
        let pair = JSONParser::parse(Rule::number, "42")?
            .next()
            .ok_or_else(|| anyhow!("no pair"))?;
        assert_eq!(pair.as_str(), "42");
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 2);
    
        let pair = JSONParser::parse(Rule::number, "-3.14")?
            .next()
            .ok_or_else(|| anyhow!("no pair"))?;
        assert_eq!(pair.as_str(), "-3.14");
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 5);
    
        let pair = JSONParser::parse(Rule::number, "0.123e-5")?
            .next()
            .ok_or_else(|| anyhow!("no pair"))?;
        assert_eq!(pair.as_str(), "0.123e-5");
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 8);
    
        Ok(())
    }

    #[test]
    #[should_panic]
    fn deserialize_invalid_number() {
        let pair = JSONParser::parse(Rule::number, "12A");
        assert!(pair.is_err());

        let pair = JSONParser::parse(Rule::number, "1.1.1");
        assert!(pair.is_err());
    }

    #[test]
    fn deserialize_string() -> anyhow::Result<()> {
        let pair = JSONParser::parse(Rule::string, r#""hello""#)?
            .next()
            .ok_or_else(|| anyhow!("no pair"))?;
        assert_eq!(pair.as_str(), r#""hello""#);
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 7);
    
        let pair = JSONParser::parse(Rule::string, r#""escape: \"test\"""#)?
            .next()
            .ok_or_else(|| anyhow!("no pair"))?;
        assert_eq!(pair.as_str(), r#""escape: \"test\"""#);
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 18);
    
        let pair = JSONParser::parse(Rule::string, r#""unterminated_string"#);
        assert!(pair.is_err());
    
        let pair = JSONParser::parse(Rule::string, r#"unquoted_string"#);
        assert!(pair.is_err());
    
        Ok(())
    }
    
    #[test]
    fn deserialize_char() -> anyhow::Result<()> {
        let pair = JSONParser::parse(Rule::char, r#"a"#)?
            .next()
            .ok_or_else(|| anyhow!("no pair"))?;
        assert_eq!(pair.as_str(), "a");
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 1);

        let pair = JSONParser::parse(Rule::char, r#"\n"#)?
            .next()
            .ok_or_else(|| anyhow!("no pair"))?;
        assert_eq!(pair.as_str(), "\\n");
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 2);

        let pair = JSONParser::parse(Rule::char, r#"\""#)?
            .next()
            .ok_or_else(|| anyhow!("no pair"))?;
        assert_eq!(pair.as_str(), r#"\""#);
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 2);

        let pair = JSONParser::parse(Rule::char, r#"\u1234"#)?
            .next()
            .ok_or_else(|| anyhow!("no pair"))?;
        assert_eq!(pair.as_str(), r#"\u1234"#);
        assert_eq!(pair.as_span().start(), 0);
        assert_eq!(pair.as_span().end(), 6);

        let pair = JSONParser::parse(Rule::char, r#""#);
        assert!(pair.is_err());

        let pair = JSONParser::parse(Rule::char, r#"\u"#);
        assert!(pair.is_err());

        Ok(())
    }

   
}