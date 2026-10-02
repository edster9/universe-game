//! What a person wants to do, parsed from text. Intents are what a browser
//! will eventually send to a server: requests, never results. See
//! docs/technology.md, "Browser and server".

use std::fmt;

use crate::units::Credits;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum Command {
    Look,
    Inventory,
    Act(Intent),
}

/// A request to change the world. The laws decide whether it's allowed.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
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
    /// Fill a container from a liquid.
    Fill {
        container: String,
        source: String,
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
    /// Search around for a way out not yet known.
    Explore,
    /// Strike at someone, with something that has an edge.
    Attack {
        target: String,
        with: Option<String>,
    },
    /// Read something, such as a map.
    Read {
        item: String,
    },
    /// Take in the view: see what lies in the distance.
    Survey,
    /// Walk within the place: up to something, until it's within reach, or
    /// to a spot given as metres east and north of the place's middle.
    Walk {
        to: String,
    },
    /// Sleep until rested, or for a while.
    Sleep {
        seconds: Option<u64>,
        /// A shelter to sleep in.
        shelter: Option<String>,
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
    /// Cut a dead body into its parts with an edged tool.
    Butcher {
        body: String,
        tool: String,
    },
    /// Put things together without a design, to make something new.
    Join {
        items: Vec<String>,
    },
    /// Name something in your own words. "it" is what you made last.
    Call {
        item: String,
        word: String,
    },
    /// Offer something you carry to someone here, for something they carry.
    Offer {
        item: String,
        person: String,
        want: String,
    },
    /// Ask someone here, who has a mind of their own, to do something.
    Ask {
        person: String,
        request: Box<Intent>,
    },
    /// Tell someone here what you call something, so they learn the word.
    Tell {
        person: String,
        item: String,
        word: String,
    },
    /// Wear something you carry, on your feet or about your body.
    Wear {
        item: String,
        on: crate::world::Covering,
    },
    /// Stop wearing something, and just carry it.
    TakeOff {
        item: String,
    },
}

impl Intent {
    /// The names of things the intent acts on, which a person with words of
    /// their own may find fit more than one thing.
    pub fn things_named(&self) -> Vec<&str> {
        match self {
            Intent::Take { item }
            | Intent::Drop { item }
            | Intent::Disassemble { item }
            | Intent::Eat { item }
            | Intent::Read { item }
            | Intent::Divide { item }
            | Intent::Rub { item, .. }
            | Intent::Call { item, .. }
            | Intent::Tell { item, .. }
            | Intent::Wear { item, .. }
            | Intent::TakeOff { item }
            | Intent::Give { item, .. }
            | Intent::Offer { item, .. } => vec![item.as_str()],
            Intent::TakeFrom { item, from } => vec![item.as_str(), from.as_str()],
            Intent::Put { item, into } => vec![item.as_str(), into.as_str()],
            Intent::Dig { source, tool } => vec![source.as_str(), tool.as_str()],
            Intent::Fill { container, source } => vec![container.as_str(), source.as_str()],
            Intent::Light { chamber } => vec![chamber.as_str()],
            Intent::Pour { liquid, into } => vec![liquid.as_str(), into.as_str()],
            Intent::Work { item, tool, .. } => {
                let mut names = vec![item.as_str()];
                names.extend(tool.as_deref());
                names
            }
            Intent::Attack { with, .. } => with.as_deref().into_iter().collect(),
            Intent::Butcher { body, tool } => vec![body.as_str(), tool.as_str()],
            Intent::Drink { source } | Intent::Gather { source } => vec![source.as_str()],
            Intent::Sleep { shelter, .. } => shelter.as_deref().into_iter().collect(),
            Intent::Join { items } => items.iter().map(String::as_str).collect(),
            Intent::Go { .. }
            | Intent::Ask { .. }
            | Intent::Pay { .. }
            | Intent::Assemble { .. }
            | Intent::Explore
            | Intent::Walk { .. }
            | Intent::Survey => Vec::new(),
        }
    }

    /// The names of what must be within reach: what's handled, struck, or
    /// handed to someone. Pointing at something, naming it, or talking to
    /// someone only needs seeing them.
    pub fn touches(&self) -> Vec<&str> {
        match self {
            Intent::Call { .. } | Intent::Tell { .. } | Intent::Offer { .. } => Vec::new(),
            Intent::Give { item, to } => vec![item.as_str(), to.as_str()],
            Intent::Attack { target, with } => std::iter::once(target.as_str())
                .chain(with.as_deref())
                .collect(),
            _ => self.things_named(),
        }
    }
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
            Intent::Fill { container, source } => write!(f, "fill {container} from {source}"),
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
            Intent::Explore => f.write_str("explore"),
            Intent::Read { item } => write!(f, "read {item}"),
            Intent::Attack { target, with: None } => write!(f, "attack {target}"),
            Intent::Attack {
                target,
                with: Some(tool),
            } => write!(f, "attack {target} with {tool}"),
            Intent::Survey => f.write_str("survey"),
            Intent::Walk { to } => write!(f, "walk to {to}"),
            Intent::Sleep { seconds, shelter } => {
                f.write_str("sleep")?;
                if let Some(shelter) = shelter {
                    write!(f, " in {shelter}")?;
                }
                if let Some(seconds) = seconds {
                    write!(f, " for {seconds} s")?;
                }
                Ok(())
            }
            Intent::Drink { source } => write!(f, "drink from {source}"),
            Intent::Gather { source } => write!(f, "gather from {source}"),
            Intent::Divide { item } => write!(f, "divide {item}"),
            Intent::Butcher { body, tool } => write!(f, "butcher {body} with {tool}"),
            Intent::Join { items } => write!(f, "join {}", items.join(" and ")),
            Intent::Call { item, word } => write!(f, "call {item} {word}"),
            Intent::Tell { person, item, word } => {
                write!(f, "tell {person} that {item} is {word}")
            }
            Intent::Ask { person, request } => write!(f, "ask {person} to {request}"),
            Intent::Offer { item, person, want } => {
                write!(f, "offer {item} to {person} for {want}")
            }
            Intent::Wear {
                item,
                on: crate::world::Covering::Feet,
            } => write!(f, "wear {item} on your feet"),
            Intent::Wear { item, .. } => write!(f, "wear {item}"),
            Intent::TakeOff { item } => write!(f, "take off {item}"),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ParseError(pub String);

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for ParseError {}

/// Splits "<thing> a <word>" at the last article, "it <word>", or, for a
/// word that takes no article, as a material's name, "<thing> <word>" at the last
/// space.
fn split_word(rest: &str) -> Option<(String, String)> {
    // The word keeps its article, as said: "a stabber", "shoes".
    let with_article = [" a ", " an "]
        .iter()
        .filter_map(|a| {
            rest.rsplit_once(a)
                .map(|(item, word)| (item, format!("{}{word}", &a[1..])))
        })
        .max_by_key(|(item, _)| item.len());
    let split = with_article
        .or_else(|| {
            rest.split_once(' ')
                .filter(|(item, _)| *item == "it")
                .map(|(item, word)| (item, word.to_string()))
        })
        .or_else(|| {
            rest.rsplit_once(' ')
                .map(|(item, word)| (item, word.to_string()))
        });
    split
        .map(|(item, word)| (item.trim().to_string(), word.trim().to_string()))
        .filter(|(item, word)| !item.is_empty() && !word.is_empty())
}

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
        "take" if rest.starts_with("off ") => Intent::TakeOff {
            item: rest["off ".len()..].trim().to_string(),
        },
        "remove" => Intent::TakeOff {
            item: one("<thing>")?,
        },
        "wear" | "don" => {
            let item = one("<thing> [on your feet]")?;
            let feet = ["on your feet", "on feet", "on my feet"]
                .iter()
                .find_map(|end| item.strip_suffix(end).map(|i| i.trim().to_string()));
            match feet {
                Some(item) if !item.is_empty() => Intent::Wear {
                    item,
                    on: crate::world::Covering::Feet,
                },
                _ => Intent::Wear {
                    item,
                    on: crate::world::Covering::Body,
                },
            }
        }
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
        "fill" => {
            let (container, source) = two(&["from", "with"], "<container> from <liquid>")?;
            Intent::Fill { container, source }
        }
        "dig" => {
            let (source, tool) = two(&["with"], "<source> with <tool>")?;
            Intent::Dig { source, tool }
        }
        "light" => Intent::Light {
            chamber: one("<thing>")?,
        },
        // Tipping is pouring loose pieces out of a container: an ember into
        // tinder.
        "pour" | "tip" => {
            let (liquid, into) = two(&["into", "in"], "<thing> into <container>")?;
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
        "assemble" | "build" | "make" => Intent::Assemble {
            design: one("<design>")?,
        },
        "join" => {
            let items: Vec<String> = rest
                .split(" and ")
                .flat_map(|part| part.split(" to "))
                .map(|part| part.trim().to_string())
                .filter(|part| !part.is_empty())
                .collect();
            if items.len() < 2 {
                return Err(usage("<thing> and <thing> [and <thing> …]"));
            }
            Intent::Join { items }
        }
        "call" | "name" => {
            let (item, word) = split_word(&rest).ok_or_else(|| usage("<thing> a <word>"))?;
            Intent::Call { item, word }
        }
        "tell" => {
            let (person, claim) = rest
                .split_once(" that ")
                .map(|(p, c)| (p.trim().to_string(), c.trim().to_string()))
                .filter(|(p, c)| !p.is_empty() && !c.is_empty())
                .ok_or_else(|| usage("<person> that <thing> is a <word>"))?;
            let (item, word) = ["is a", "is an", "is"]
                .iter()
                .find_map(|w| claim.rsplit_once(&format!(" {w} ")))
                .map(|(i, w)| (i.trim().to_string(), w.trim().to_string()))
                .filter(|(i, w)| !i.is_empty() && !w.is_empty())
                .ok_or_else(|| usage("<person> that <thing> is a <word>"))?;
            Intent::Tell { person, item, word }
        }
        "offer" | "trade" => {
            let form = "<something> to <someone> for <something>";
            let (item, rest) = rest.split_once(" to ").ok_or_else(|| usage(form))?;
            let (person, want) = rest.rsplit_once(" for ").ok_or_else(|| usage(form))?;
            let (item, person, want) = (item.trim(), person.trim(), want.trim());
            if item.is_empty() || person.is_empty() || want.is_empty() {
                return Err(usage(form));
            }
            Intent::Offer {
                item: item.to_string(),
                person: person.to_string(),
                want: want.to_string(),
            }
        }
        "ask" => {
            let (person, request) = rest
                .split_once(" to ")
                .map(|(p, r)| (p.trim(), r.trim()))
                .filter(|(p, r)| !p.is_empty() && !r.is_empty())
                .ok_or_else(|| usage("<person> to <do something>"))?;
            match parse(request)? {
                Command::Act(request) => Intent::Ask {
                    person: person.to_string(),
                    request: Box::new(request),
                },
                _ => return Err(usage("<person> to <do something>")),
            }
        }
        "explore" | "search" => Intent::Explore,
        "attack" | "strike" | "stab" => match two(&["with"], "<someone> with <something>") {
            Ok((target, tool)) => Intent::Attack {
                target,
                with: Some(tool),
            },
            Err(_) => Intent::Attack {
                target: one("<someone>")?,
                with: None,
            },
        },
        "read" | "study" => Intent::Read {
            item: one("<thing>")?,
        },
        "survey" => Intent::Survey,
        "walk" | "approach" => {
            let to = one("<thing>")?;
            Intent::Walk {
                to: to.strip_prefix("to ").unwrap_or(&to).trim().to_string(),
            }
        }
        "sleep" => {
            // "sleep", "sleep for 9 h", "sleep in <shelter>", or both.
            let (shelter, time) = match rest.strip_prefix("in ") {
                Some(inside) => match inside.rsplit_once(" for ") {
                    Some((shelter, time)) => (Some(shelter.trim().to_string()), time.to_string()),
                    None => (Some(inside.trim().to_string()), String::new()),
                },
                None => (None, rest.strip_prefix("for ").unwrap_or(&rest).to_string()),
            };
            let time = time.trim();
            let seconds = if time.is_empty() {
                None
            } else {
                Some(
                    crate::units::parse_quantity(time, crate::units::property::DURATION, "a time")
                        .map_err(|_| ParseError(format!("{time:?} isn't a time like \"2 h\"")))?,
                )
            };
            Intent::Sleep { seconds, shelter }
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
        "butcher" | "carve" => {
            let (body, tool) = two(&["with"], "<body> with <something with an edge>")?;
            Intent::Butcher { body, tool }
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
