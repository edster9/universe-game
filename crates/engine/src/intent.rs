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
        /// Something that floats, to cross a liquid on.
        aboard: Option<String>,
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
        /// `None` means bare hands.
        tool: Option<String>,
    },
    Rub {
        item: String,
        against: String,
        into: Option<String>,
        /// How long to keep rubbing, in seconds. The world's usual session
        /// if not given.
        seconds: Option<u64>,
    },
    Assemble {
        design: String,
    },
    Disassemble {
        item: String,
    },
    Eat {
        item: String,
    },
    /// Sleep until rested, or for a while.
    Sleep {
        seconds: Option<u64>,
    },
    Drink {
        source: String,
    },
    Gather {
        source: String,
    },
    Divide {
        item: String,
    },
}

impl fmt::Display for Intent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Intent::Go {
                place,
                aboard: None,
            } => write!(f, "go {place}"),
            Intent::Go {
                place,
                aboard: Some(vessel),
            } => write!(f, "go {place} on {vessel}"),
            Intent::Take { item } => write!(f, "take {item}"),
            Intent::TakeFrom { item, from } => write!(f, "take {item} from {from}"),
            Intent::Drop { item } => write!(f, "drop {item}"),
            Intent::Put { item, into } => write!(f, "put {item} in {into}"),
            Intent::Give { item, to } => write!(f, "give {item} to {to}"),
            Intent::Pay { to, amount } => write!(f, "pay {to} {amount}"),
            Intent::Dig { source, tool } => write!(f, "dig {source} with {tool}"),
            Intent::Light { chamber } => write!(f, "light {chamber}"),
            Intent::Pour { liquid, into } => write!(f, "pour {liquid} into {into}"),
            Intent::Work {
                item,
                shape,
                tool: Some(tool),
            } => write!(f, "work {item} into {shape} with {tool}"),
            Intent::Work {
                item,
                shape,
                tool: None,
            } => write!(f, "work {item} into {shape} by hand"),
            Intent::Rub {
                item,
                against,
                into,
                seconds,
            } => {
                write!(f, "rub {item} against {against}")?;
                if let Some(into) = into {
                    write!(f, " into {into}")?;
                }
                match seconds {
                    Some(s) => write!(f, " for {s} s"),
                    None => Ok(()),
                }
            }
            Intent::Assemble { design } => write!(f, "assemble {design}"),
            Intent::Disassemble { item } => write!(f, "take apart {item}"),
            Intent::Eat { item } => write!(f, "eat {item}"),
            Intent::Sleep { seconds: None } => f.write_str("sleep"),
            Intent::Sleep {
                seconds: Some(seconds),
            } => write!(f, "sleep for {seconds} s"),
            Intent::Drink { source } => write!(f, "drink from {source}"),
            Intent::Gather { source } => write!(f, "gather from {source}"),
            Intent::Divide { item } => write!(f, "divide {item}"),
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
        "go" => match two(&["on", "aboard"], "<place> on <something that floats>") {
            Ok((place, vessel)) => Intent::Go {
                place,
                aboard: Some(vessel),
            },
            Err(_) => Intent::Go {
                place: one("<place>")?,
                aboard: None,
            },
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
            // "work <thing> into <shape> with <tool>", or by hand without "with".
            let (what, tool) = match two(&["with"], "<thing> into <shape> [with <tool>]") {
                Ok((what, tool)) => (what, Some(tool)),
                Err(_) => (rest.trim_end_matches(" by hand").to_string(), None),
            };
            let (item, shape) = what
                .rsplit_once(" into ")
                .map(|(a, b)| (a.trim().to_string(), b.trim().to_string()))
                .filter(|(a, b)| !a.is_empty() && !b.is_empty())
                .ok_or_else(|| usage("<thing> into <shape> [with <tool>]"))?;
            Intent::Work { item, shape, tool }
        }
        "rub" => {
            let (rest, seconds) = match rest.rsplit_once(" for ") {
                Some((r, time)) => {
                    let seconds = crate::units::parse_quantity(
                        time,
                        crate::units::property::DURATION,
                        "a time",
                    )
                    .map_err(|_| ParseError(format!("{time:?} isn't a time like \"1 min\"")))?;
                    (r.to_string(), Some(seconds))
                }
                None => (rest.clone(), None),
            };
            let (rest, into) = match rest.rsplit_once(" into ") {
                Some((r, into)) if !into.trim().is_empty() => {
                    (r.to_string(), Some(into.trim().to_string()))
                }
                _ => (rest.clone(), None),
            };
            let (item, against) = rest
                .rsplit_once(" against ")
                .map(|(a, b)| (a.trim().to_string(), b.trim().to_string()))
                .filter(|(a, b)| !a.is_empty() && !b.is_empty())
                .ok_or_else(|| usage("<thing> against <thing> [into <container>]"))?;
            Intent::Rub {
                item,
                against,
                into,
                seconds,
            }
        }
        "assemble" | "build" => Intent::Assemble {
            design: one("<design>")?,
        },
        "sleep" => {
            let time = rest.strip_prefix("for ").unwrap_or(&rest).trim();
            let seconds = if time.is_empty() {
                None
            } else {
                Some(
                    crate::units::parse_quantity(time, crate::units::property::DURATION, "a time")
                        .map_err(|_| ParseError(format!("{time:?} isn't a time like \"2 h\"")))?,
                )
            };
            Intent::Sleep { seconds }
        }
        "eat" => Intent::Eat {
            item: one("<thing>")?,
        },
        "drink" => {
            let source = one("<liquid>")?;
            let source = source
                .strip_prefix("from ")
                .unwrap_or(&source)
                .trim()
                .to_string();
            Intent::Drink { source }
        }
        "gather" | "collect" => {
            let source = one("<source>")?;
            let source = source
                .strip_prefix("from ")
                .unwrap_or(&source)
                .trim()
                .to_string();
            Intent::Gather { source }
        }
        "divide" | "split" => Intent::Divide {
            item: one("<thing>")?,
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
                tool: Some("heavy tool".into())
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
