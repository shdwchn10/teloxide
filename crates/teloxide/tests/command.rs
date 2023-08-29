// https://github.com/rust-lang/rust-clippy/issues/7422
#![allow(clippy::nonstandard_macro_braces)]

#[cfg(feature = "macros")]
use teloxide::utils::command::BotCommands;

// We put tests here because macro expand in unit tests in module
// teloxide::utils::command was a failure

#[test]
#[cfg(feature = "macros")]
fn parse_command_with_args() {
    #[derive(BotCommands, Debug, PartialEq)]
    #[command(rename_rule = "lowercase")]
    enum DefaultCommands {
        Start(String),
        Help,
    }

    let data = "/start arg1 arg2";
    let expected = DefaultCommands::Start("arg1 arg2".to_string());
    let actual = DefaultCommands::parse(data, "").unwrap();
    assert_eq!(actual, expected)
}

#[test]
#[cfg(feature = "macros")]
fn parse_command_with_non_string_arg() {
    #[derive(BotCommands, Debug, PartialEq)]
    #[command(rename_rule = "lowercase")]
    enum DefaultCommands {
        Start(i32),
        Help,
    }

    let data = "/start -50";
    let expected = DefaultCommands::Start("-50".parse().unwrap());
    let actual = DefaultCommands::parse(data, "").unwrap();
    assert_eq!(actual, expected)
}

#[test]
#[cfg(feature = "macros")]
fn attribute_prefix() {
    #[derive(BotCommands, Debug, PartialEq)]
    #[command(rename_rule = "lowercase")]
    enum DefaultCommands {
        #[command(prefix = "!")]
        Start(String),
        Help,
    }

    let data = "!start arg1 arg2";
    let expected = DefaultCommands::Start("arg1 arg2".to_string());
    let actual = DefaultCommands::parse(data, "").unwrap();
    assert_eq!(actual, expected)
}

#[test]
#[cfg(feature = "macros")]
fn many_attributes() {
    #[derive(BotCommands, Debug, PartialEq)]
    #[command(rename_rule = "lowercase")]
    enum DefaultCommands {
        #[command(prefix = "!", description = "desc")]
        Start,
        Help,
    }

    assert_eq!(DefaultCommands::Start, DefaultCommands::parse("!start", "").unwrap());
    assert_eq!(DefaultCommands::descriptions().to_string(), "!start — desc\n/help");
}

#[test]
#[cfg(feature = "macros")]
fn global_attributes() {
    #[derive(BotCommands, Debug, PartialEq)]
    #[command(prefix = "!", rename_rule = "lowercase", description = "Bot commands")]
    enum DefaultCommands {
        #[command(prefix = "/")]
        Start,
        Help,
    }

    assert_eq!(DefaultCommands::Start, DefaultCommands::parse("/start", "MyNameBot").unwrap());
    assert_eq!(DefaultCommands::Help, DefaultCommands::parse("!help", "MyNameBot").unwrap());
    assert_eq!(DefaultCommands::descriptions().to_string(), "Bot commands\n\n/start\n!help");
}

#[test]
#[cfg(feature = "macros")]
fn parse_command_with_bot_name() {
    #[derive(BotCommands, Debug, PartialEq)]
    #[command(rename_rule = "lowercase")]
    enum DefaultCommands {
        #[command(prefix = "/")]
        Start,
        Help,
    }

    assert_eq!(
        DefaultCommands::Start,
        DefaultCommands::parse("/start@MyNameBot", "MyNameBot").unwrap()
    );
}

#[test]
#[cfg(feature = "macros")]
fn parse_with_split() {
    #[derive(BotCommands, Debug, PartialEq)]
    #[command(rename_rule = "lowercase")]
    #[command(parse_with = "split")]
    enum DefaultCommands {
        Start(u8, String),
        Help,
    }

    assert_eq!(
        DefaultCommands::Start(10, "hello".to_string()),
        DefaultCommands::parse("/start 10 hello", "").unwrap(),
    );
}

#[test]
#[cfg(feature = "macros")]
fn parse_with_split2() {
    #[derive(BotCommands, Debug, PartialEq)]
    #[command(rename_rule = "lowercase")]
    #[command(parse_with = "split", separator = "|")]
    enum DefaultCommands {
        Start(u8, String),
        Help,
    }

    assert_eq!(
        DefaultCommands::Start(10, "hello".to_string()),
        DefaultCommands::parse("/start 10|hello", "").unwrap()
    );
}

#[test]
#[cfg(feature = "macros")]
fn parse_with_split3() {
    #[derive(BotCommands, Debug, PartialEq)]
    #[command(rename_rule = "lowercase")]
    #[command(parse_with = "split")]
    enum DefaultCommands {
        Start(u8),
        Help,
    }

    assert_eq!(DefaultCommands::Start(10), DefaultCommands::parse("/start 10", "").unwrap(),);
}

#[test]
#[cfg(feature = "macros")]
fn parse_with_split4() {
    #[derive(BotCommands, Debug, PartialEq)]
    #[command(rename_rule = "lowercase")]
    #[command(parse_with = "split")]
    enum DefaultCommands {
        Start(),
        Help,
    }

    assert_eq!(DefaultCommands::Start(), DefaultCommands::parse("/start", "").unwrap(),);
}

#[test]
#[cfg(feature = "macros")]
fn parse_custom_parser() {
    mod parser {
        use teloxide::utils::command::ParseError;

        pub fn custom_parse_function(s: String) -> Result<(u8, String), ParseError> {
            let vec = s.split_whitespace().collect::<Vec<_>>();
            let (left, right) = match vec.as_slice() {
                [l, r] => (l, r),
                _ => return Err(ParseError::IncorrectFormat("might be 2 arguments!".into())),
            };
            left.parse::<u8>().map(|res| (res, (*right).to_string())).map_err(|_| {
                ParseError::Custom("First argument must be a integer!".to_owned().into())
            })
        }
    }

    use parser::custom_parse_function;

    #[derive(BotCommands, Debug, PartialEq)]
    #[command(rename_rule = "lowercase")]
    enum DefaultCommands {
        #[command(parse_with = custom_parse_function)]
        Start(u8, String),

        // Test <https://github.com/teloxide/teloxide/issues/668>.
        #[command(parse_with = parser::custom_parse_function)]
        TestPath(u8, String),

        Help,
    }

    assert_eq!(
        DefaultCommands::Start(10, "hello".to_string()),
        DefaultCommands::parse("/start 10 hello", "").unwrap()
    );
    assert_eq!(
        DefaultCommands::TestPath(10, "hello".to_string()),
        DefaultCommands::parse("/testpath 10 hello", "").unwrap()
    );
}

#[test]
#[cfg(feature = "macros")]
fn parse_named_fields() {
    #[derive(BotCommands, Debug, PartialEq)]
    #[command(rename_rule = "lowercase")]
    #[command(parse_with = "split")]
    enum DefaultCommands {
        Start { num: u8, data: String },
        Help,
    }

    assert_eq!(
        DefaultCommands::Start { num: 10, data: "hello".to_string() },
        DefaultCommands::parse("/start 10 hello", "").unwrap()
    );
}

#[test]
#[cfg(feature = "macros")]
fn descriptions_off() {
    #[derive(BotCommands, Debug, PartialEq)]
    #[command(rename_rule = "lowercase")]
    enum DefaultCommands {
        #[command(hide)]
        Start,
        #[command(hide)]
        Username,
        Help,
    }

    assert_eq!(DefaultCommands::descriptions().to_string(), "/help".to_owned());
}

#[test]
#[cfg(feature = "macros")]
fn description_with_doc_attr() {
    #[derive(BotCommands, Debug, PartialEq)]
    #[command(rename_rule = "lowercase")]
    enum DefaultCommands {
        /// Start command
        Start,
        /// Help command\nwith new line
        Help,
        /// Foo command
        /// with new line
        Foo,
    }

    assert_eq!(
        DefaultCommands::descriptions().to_string(),
        "/start — Start command\n/help — Help command\nwith new line\n/foo — Foo command\nwith \
         new line"
    );
}

#[test]
#[cfg(feature = "macros")]
fn description_with_doc_attr_and_command() {
    #[derive(BotCommands, Debug, PartialEq)]
    #[command(rename_rule = "lowercase")]
    enum DefaultCommands {
        /// Start command
        #[command(description = "Start command")]
        Start,
        #[command(description = "Help command\nwith new line")]
        Help,
        /// Foo command
        /// with new line
        #[command(description = "Foo command\nwith new line")]
        Foo,
    }

    assert_eq!(
        DefaultCommands::descriptions().to_string(),
        "/start — Start command\nStart command\n/help — Help command\nwith new line\n/foo — Foo command\nwith \
         new line\nFoo command\nwith new line"
    );
}

#[test]
#[cfg(feature = "macros")]
fn rename_rules() {
    #[derive(BotCommands, Debug, PartialEq)]
    enum DefaultCommands {
        #[command(rename_rule = "lowercase")]
        AaaAaa,
        #[command(rename_rule = "UPPERCASE")]
        BbbBbb,
        #[command(rename_rule = "PascalCase")]
        CccCcc,
        #[command(rename_rule = "camelCase")]
        DddDdd,
        #[command(rename_rule = "snake_case")]
        EeeEee,
        #[command(rename_rule = "SCREAMING_SNAKE_CASE")]
        FffFff,
        #[command(rename_rule = "kebab-case")]
        GggGgg,
        #[command(rename_rule = "SCREAMING-KEBAB-CASE")]
        HhhHhh,
        #[command(rename = "Bar")]
        Foo,
    }

    assert_eq!(DefaultCommands::AaaAaa, DefaultCommands::parse("/aaaaaa", "").unwrap());
    assert_eq!(DefaultCommands::BbbBbb, DefaultCommands::parse("/BBBBBB", "").unwrap());
    assert_eq!(DefaultCommands::CccCcc, DefaultCommands::parse("/CccCcc", "").unwrap());
    assert_eq!(DefaultCommands::DddDdd, DefaultCommands::parse("/dddDdd", "").unwrap());
    assert_eq!(DefaultCommands::EeeEee, DefaultCommands::parse("/eee_eee", "").unwrap());
    assert_eq!(DefaultCommands::FffFff, DefaultCommands::parse("/FFF_FFF", "").unwrap());
    assert_eq!(DefaultCommands::GggGgg, DefaultCommands::parse("/ggg-ggg", "").unwrap());
    assert_eq!(DefaultCommands::HhhHhh, DefaultCommands::parse("/HHH-HHH", "").unwrap());
    assert_eq!(DefaultCommands::Foo, DefaultCommands::parse("/Bar", "").unwrap());

    assert_eq!(
        "/aaaaaa\n/BBBBBB\n/CccCcc\n/dddDdd\n/eee_eee\n/FFF_FFF\n/ggg-ggg\n/HHH-HHH\n/Bar",
        DefaultCommands::descriptions().to_string()
    );
}

#[test]
#[cfg(feature = "macros")]
fn custom_result() {
    #[allow(dead_code)]
    type Result = ();

    #[derive(BotCommands, Debug, PartialEq)]
    enum DefaultCommands {}
}
