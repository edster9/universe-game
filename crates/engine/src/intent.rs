//! What a person wants to do, parsed from text. Intents are what a browser
//! will eventually send to a server: requests, never results. See
//! docs/technology.md, "Browser and server".

use std::fmt;

use crate::units::Credits;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Command {
    Look,
    Inventory,
    Act(Intent),
}

/// A request to change the world. The laws decide whether it's allowed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Intent {
    Go { place: String },
    Take { item: String },
    Drop { item: String },
    Give { item: String, to: String },
    Pay { to: String, amount: Credits },
}

impl fmt::Display for Intent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Intent::Go { place } => write!(f, "go {place}"),
            Intent::Take { item } => write!(f, "take {item}"),
            Intent::Drop { item } => write!(f, "drop {item}"),
            Intent::Give { item, to } => write!(f, "give {item} to {to}"),
            Intent::Pay { to, amount } => write!(f, "pay {to} {amount}"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParseError(pub String);

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for ParseError {}

pub fn parse(line: &str) -> Result<Command, ParseError> {
    let words: Vec<&str> = line.split_whitespace().collect();
    let Some((verb, rest)) = words.split_first() else {
        return Err(ParseError("say something".into()));
    };
    let rest = rest.join(" ");
    let needs = |what: &str| {
        ParseError(format!(
            "{} what? Try \"{} <{what}>\"",
            verb,
            verb.to_lowercase()
        ))
    };

    match verb.to_lowercase().as_str() {
        "look" | "l" => Ok(Command::Look),
        "inventory" | "inv" | "i" => Ok(Command::Inventory),
        "go" => {
            if rest.is_empty() {
                return Err(needs("place"));
            }
            Ok(Command::Act(Intent::Go { place: rest }))
        }
        "take" | "get" => {
            if rest.is_empty() {
                return Err(needs("thing"));
            }
            Ok(Command::Act(Intent::Take { item: rest }))
        }
        "drop" => {
            if rest.is_empty() {
                return Err(needs("thing"));
            }
            Ok(Command::Act(Intent::Drop { item: rest }))
        }
        "give" => {
            let (item, to) = rest
                .rsplit_once(" to ")
                .ok_or_else(|| ParseError("try \"give <thing> to <person>\"".into()))?;
            let (item, to) = (item.trim(), to.trim());
            if item.is_empty() || to.is_empty() {
                return Err(ParseError("try \"give <thing> to <person>\"".into()));
            }
            Ok(Command::Act(Intent::Give {
                item: item.into(),
                to: to.into(),
            }))
        }
        "pay" => {
            let (to, amount) = rest
                .rsplit_once(' ')
                .ok_or_else(|| ParseError("try \"pay <person> <amount>\"".into()))?;
            let amount = amount
                .parse::<u64>()
                .map_err(|_| ParseError(format!("{amount:?} isn't a whole number of credits")))?;
            Ok(Command::Act(Intent::Pay {
                to: to.trim().into(),
                amount: Credits::new(amount),
            }))
        }
        other => Err(ParseError(format!(
            "I don't know how to {other:?}. Type \"help\" for commands"
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_multi_word_names() {
        assert_eq!(
            parse("give iron ingot to Mara"),
            Ok(Command::Act(Intent::Give {
                item: "iron ingot".into(),
                to: "Mara".into()
            }))
        );
        assert_eq!(
            parse("  pay   the traveller  10 "),
            Ok(Command::Act(Intent::Pay {
                to: "the traveller".into(),
                amount: Credits::new(10)
            }))
        );
    }

    #[test]
    fn refuses_amounts_that_are_not_whole_credits() {
        for line in [
            "pay mara -5",
            "pay mara 1.5",
            "pay mara ten",
            "pay mara 99999999999999999999999",
        ] {
            assert!(parse(line).is_err(), "{line:?} should be refused");
        }
    }
}
