//! The console's input line: editing it as text boxes do (a cursor that
//! moves, Home and End, deleting a word at a time, pasting), and Tab
//! completing what's typed, as consoles do. Plain text, no Bevy, so it can
//! be tested on its own.

/// The line being typed, and where the cursor is in it, in characters.
#[derive(Default, Debug, PartialEq)]
pub struct Line {
    pub text: String,
    pub cursor: usize,
}

impl Line {
    /// Where the cursor is in bytes, for slicing.
    fn at(&self, chars: usize) -> usize {
        self.text
            .char_indices()
            .nth(chars)
            .map_or(self.text.len(), |(i, _)| i)
    }

    fn len(&self) -> usize {
        self.text.chars().count()
    }

    /// Replaces the whole line, with the cursor at its end.
    pub fn set(&mut self, text: &str) {
        self.text = text.to_string();
        self.cursor = self.len();
    }

    /// Empties the line, returning what was on it.
    pub fn take(&mut self) -> String {
        self.cursor = 0;
        std::mem::take(&mut self.text)
    }

    /// Types text where the cursor is (a paste on one line: line breaks
    /// become spaces).
    pub fn insert(&mut self, text: &str) {
        let text: String = text
            .chars()
            .map(|c| if c == '\n' || c == '\t' { ' ' } else { c })
            .filter(|c| !c.is_control())
            .collect();
        let at = self.at(self.cursor);
        self.text.insert_str(at, &text);
        self.cursor += text.chars().count();
    }

    pub fn backspace(&mut self) {
        if self.cursor > 0 {
            let at = self.at(self.cursor - 1);
            self.text.remove(at);
            self.cursor -= 1;
        }
    }

    pub fn delete(&mut self) {
        if self.cursor < self.len() {
            let at = self.at(self.cursor);
            self.text.remove(at);
        }
    }

    /// Deletes back to the start of the word before the cursor (and the
    /// spaces after it), as Ctrl+Backspace does.
    pub fn delete_word(&mut self) {
        let before: Vec<char> = self.text.chars().take(self.cursor).collect();
        let mut start = before.len();
        while start > 0 && before[start - 1] == ' ' {
            start -= 1;
        }
        while start > 0 && before[start - 1] != ' ' {
            start -= 1;
        }
        let (from, to) = (self.at(start), self.at(self.cursor));
        self.text.replace_range(from..to, "");
        self.cursor = start;
    }

    pub fn left(&mut self) {
        self.cursor = self.cursor.saturating_sub(1);
    }

    pub fn right(&mut self) {
        self.cursor = (self.cursor + 1).min(self.len());
    }

    pub fn home(&mut self) {
        self.cursor = 0;
    }

    pub fn end(&mut self) {
        self.cursor = self.len();
    }

    /// The line with a mark where the cursor is.
    pub fn shown(&self, mark: char) -> String {
        let at = self.at(self.cursor);
        format!("{}{mark}{}", &self.text[..at], &self.text[at..])
    }
}

/// Which word is being completed: the first (a command, or a tool if it
/// starts with a slash), or one after it.
#[derive(Debug, PartialEq)]
pub enum Slot<'a> {
    First,
    /// After the line's first word: the names it can take.
    After(&'a str),
}

/// Completes what's typed before the cursor from what `candidates` offers
/// for its slot, as far as all the matches agree (with a space after, if
/// only one matches). If that adds nothing and several match, returns them,
/// to be listed. Names can be several words ("dry grass"): the longest
/// stretch of words before the cursor that begins one is completed.
pub fn complete(line: &mut Line, candidates: impl FnOnce(Slot<'_>) -> Vec<String>) -> Vec<String> {
    let before: String = line.text.chars().take(line.cursor).collect();
    let (slot, starts) = match before.find(' ') {
        None => (Slot::First, vec![0]),
        Some(space) => {
            let first = &before[..space];
            // Each word start after the first word, earliest first.
            let starts: Vec<usize> = before
                .char_indices()
                .filter(|&(i, c)| i > space && c != ' ' && before[..i].ends_with(' '))
                .map(|(i, _)| i)
                .chain(before.ends_with(' ').then_some(before.len()))
                .collect();
            (Slot::After(first), starts)
        }
    };
    let options = candidates(slot);
    let lower = |s: &str| s.to_lowercase();
    // The earliest start whose stretch begins some candidate: the longest
    // name typed so far.
    let Some((start, matches)) = starts.iter().find_map(|&start| {
        let typed = lower(&before[start..]);
        let matches: Vec<&String> = options
            .iter()
            .filter(|o| lower(o).starts_with(&typed))
            .collect();
        (!matches.is_empty()).then_some((start, matches))
    }) else {
        return Vec::new();
    };
    let typed = &before[start..];
    // As far as all of them agree.
    let first = matches[0];
    let mut common = first.chars().count();
    for other in &matches[1..] {
        common = first
            .chars()
            .zip(other.chars())
            .take_while(|(a, b)| a.eq_ignore_ascii_case(b))
            .count()
            .min(common);
    }
    let agreed: String = first.chars().take(common).collect();
    let single = matches.len() == 1;
    if agreed.chars().count() <= typed.chars().count() && !single {
        let mut listed: Vec<String> = matches.into_iter().cloned().collect();
        listed.sort();
        listed.dedup();
        return listed;
    }
    let completed = if single { format!("{agreed} ") } else { agreed };
    let after: String = line.text.chars().skip(line.cursor).collect();
    let after = if single {
        after.trim_start().to_string()
    } else {
        after
    };
    let start_chars = before[..start].chars().count();
    line.text = format!("{}{completed}{after}", &before[..start]);
    line.cursor = start_chars + completed.chars().count();
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn line(text: &str) -> Line {
        let mut l = Line::default();
        l.set(text);
        l
    }

    #[test]
    fn the_cursor_moves_and_edits_where_it_is() {
        let mut l = line("gather grss");
        l.left();
        l.left();
        l.insert("a");
        assert_eq!(l.text, "gather grass");
        l.home();
        l.delete();
        assert_eq!(l.text, "ather grass");
        l.end();
        l.backspace();
        assert_eq!(l.shown('|'), "ather gras|");
        l.delete_word();
        assert_eq!(l.text, "ather ");
        l.delete_word();
        assert_eq!(l.text, "");
        // A paste over several lines stays on one.
        l.insert("look\nback");
        assert_eq!((l.text.as_str(), l.cursor), ("look back", 9));
    }

    #[test]
    fn tab_completes_a_command_then_a_name_of_several_words() {
        let names = |slot: Slot| match slot {
            Slot::First => vec!["gather".into(), "give".into(), "go".into()],
            Slot::After(_) => vec!["dry grass".into(), "dry twigs".into(), "deadwood".into()],
        };
        let mut l = line("gat");
        assert!(complete(&mut l, names).is_empty());
        assert_eq!(l.text, "gather ");
        // Several match, and agree only so far: completed that far.
        let mut l = line("gather dr");
        assert!(complete(&mut l, names).is_empty());
        assert_eq!(l.text, "gather dry ");
        // Then nothing more agrees: they're listed instead.
        let listed = complete(&mut l, names);
        assert_eq!(listed, vec!["dry grass", "dry twigs"]);
        let mut l = line("gather dry g");
        complete(&mut l, names);
        assert_eq!(l.text, "gather dry grass ");
        // Nothing matches: nothing changes.
        let mut l = line("gather moss");
        assert!(complete(&mut l, names).is_empty());
        assert_eq!(l.text, "gather moss");
        // The first word decides what follows; mid-line, the rest stays.
        let mut l = line("g grass");
        l.cursor = 1;
        let listed = complete(&mut l, names);
        assert_eq!(listed, vec!["gather", "give", "go"]);
    }
}
