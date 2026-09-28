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
    Go {
        place: String,
    },
    Take {
        item: String,
    },
    TakeFrom {
        item: String,
        from: String,
    },
    Drop {
        item: String,
    },
    Put {
        item: String,
        into: String,
    },
    Give {
        item: String,
        to: String,
    },
    Pay {
        to: String,
        amount: Credits,
    },
    Dig {
        source: String,
        tool: String,
    },
    Light {
        chamber: String,
    },
    Pour {
        liquid: String,
        into: String,
    },
    Work {
        item: String,
        shape: String,
        tool: String,
    },
    Rub {
        item: String,
        against: String,
    },
    Assemble {
        design: String,
    },
    Disassemble {
        item: String,
    },
}

impl fmt::Display for Intent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Intent::Go { place } => write!(f, "go {place}"),
            Intent::Take { item } => write!(f, "take {item}"),
            Intent::TakeFrom { item, from } => write!(f, "take {item} from {from}"),
            Intent::Drop { item } => write!(f, "drop {item}"),
            Intent::Put { item, into } => write!(f, "put {item} in {into}"),
            Intent::Give { item, to } => write!(f, "give {item} to {to}"),
            Intent::Pay { to, amount } => write!(f, "pay {to} {amount}"),
            Intent::Dig { source, tool } => write!(f, "dig {source} with {tool}"),
            Intent::Light { chamber } => write!(f, "light {chamber}"),
            Intent::Pour { liquid, into } => write!(f, "pour {liquid} into {into}"),
            Intent::Work { item, shape, tool } => write!(f, "work {item} into {shape} with {tool}"),
            Intent::Rub { item, against } => write!(f, "rub {item} against {against}"),
            Intent::Assemble { design } => write!(f, "assemble {design}"),
            Intent::Disassemble { item } => write!(f, "take apart {item}"),
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
    let verb = verb.to_lowercase();
    let usage = |form: &str| ParseError(format!("try \"{verb} {form}\""));
    let one = |form: &str| {
        if rest.is_empty() {
            Err(usage(form))
        } else {
            Ok(rest.clone())
        }
    };
    // Splits "a <word> b" at the last <word>, needing both sides.
    let two = |words: &[&str], form: &str| {
        words
            .iter()
            .find_map(|w| rest.rsplit_once(&format!(" {w} ")))
            .map(|(a, b)| (a.trim().to_string(), b.trim().to_string()))
            .filter(|(a, b)| !a.is_empty() && !b.is_empty())
            .ok_or_else(|| usage(form))
    };

    let intent = match verb.as_str() {
        "look" | "l" => return Ok(Command::Look),
        "inventory" | "inv" | "i" => return Ok(Command::Inventory),
        "go" => Intent::Go {
            place: one("<place>")?,
        },
        "take" | "get" => match two(&["from"], "<thing> from <container>") {
            Ok((item, from)) => Intent::TakeFrom { item, from },
            Err(_) => Intent::Take {
                item: one("<thing>")?,
            },
        },
        "drop" => Intent::Drop {
            item: one("<thing>")?,
        },
        "put" => {
            let (item, into) = two(&["into", "in"], "<thing> in <container>")?;
            Intent::Put { item, into }
        }
        "give" => {
            let (item, to) = two(&["to"], "<thing> to <person>")?;
            Intent::Give { item, to }
        }
        "pay" => {
            let (to, amount) = rest
                .rsplit_once(' ')
                .ok_or_else(|| usage("<person> <amount>"))?;
            let amount = amount
                .parse::<u64>()
                .map_err(|_| ParseError(format!("{amount:?} isn't a whole number of credits")))?;
            Intent::Pay {
                to: to.trim().into(),
                amount: Credits::new(amount),
            }
        }
        "dig" => {
            let (source, tool) = two(&["with"], "<source> with <tool>")?;
            Intent::Dig { source, tool }
        }
        "light" => Intent::Light {
            chamber: one("<thing>")?,
        },
        "pour" => {
            let (liquid, into) = two(&["into", "in"], "<liquid> into <container>")?;
            Intent::Pour { liquid, into }
        }
        "work" => {
            let (what, tool) = two(&["with"], "<thing> into <shape> with <tool>")?;
            let (item, shape) = what
                .rsplit_once(" into ")
                .map(|(a, b)| (a.trim().to_string(), b.trim().to_string()))
                .filter(|(a, b)| !a.is_empty() && !b.is_empty())
                .ok_or_else(|| usage("<thing> into <shape> with <tool>"))?;
            Intent::Work { item, shape, tool }
        }
        "rub" => {
            let (item, against) = two(&["against", "on", "with"], "<thing> against <thing>")?;
            Intent::Rub { item, against }
        }
        "assemble" | "build" => Intent::Assemble {
            design: one("<design>")?,
        },
        "disassemble" | "dismantle" => Intent::Disassemble {
            item: one("<thing>")?,
        },
        other => {
            return Err(ParseError(format!(
                "I don't know how to {other:?}. Type \"help\" for commands"
            )));
        }
    };
    Ok(Command::Act(intent))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_multi_word_names() {
        assert_eq!(
            parse("give wooden box to Mara"),
            Ok(Command::Act(Intent::Give {
                item: "wooden box".into(),
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
        assert_eq!(
            parse("work lump of stuff into long shape with heavy tool"),
            Ok(Command::Act(Intent::Work {
                item: "lump of stuff".into(),
                shape: "long shape".into(),
                tool: "heavy tool".into()
            }))
        );
        assert_eq!(
            parse("take box from big chest"),
            Ok(Command::Act(Intent::TakeFrom {
                item: "box".into(),
                from: "big chest".into()
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

    #[test]
    fn refuses_incomplete_commands() {
        for line in [
            "dig",
            "dig here",
            "put box",
            "work box with tool",
            "pour",
            "give box to",
            "take",
        ] {
            assert!(parse(line).is_err(), "{line:?} should be refused");
        }
    }
}
