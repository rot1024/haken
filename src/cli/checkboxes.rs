// from: https://github.com/mitsuhiko/dialoguer
use std::io;
use std::ops::Rem;
use std::iter::repeat;

use console::{style, Key, Term};

pub struct Checkboxes {
    items: Vec<String>,
    clear: bool,
    indicator: String,
    noindicator: String,
    check: String,
    nocheck: String,
}

impl Checkboxes {
    /// Creates the prompt with a specific text.
    pub fn new(items: Vec<String>, clear: bool) -> Checkboxes {
        Checkboxes {
            items: items,
            clear: clear,
            indicator: format!("{}", style("❯").blue()),
            noindicator: format!(" "),
            check: format!("{} ", style("◉").yellow()),
            nocheck: format!("{} ", style("◯")),
        }
    }

    /// Enables user interaction and returns the result.
    ///
    /// The user can select the items with the space bar and on enter
    /// the selected items will be returned.
    pub fn interact(&self) -> io::Result<Vec<usize>> {
        self.interact_on(&Term::stderr())
    }

    /// Like `interact` but allows a specific terminal to be set.
    pub fn interact_on(&self, term: &Term) -> io::Result<Vec<usize>> {
        let mut sel = 0;
        let mut selected: Vec<_> = repeat(false).take(self.items.len()).collect();
        loop {
            for (idx, item) in self.items.iter().enumerate() {
                term.write_line(&format!(
                    "{}{}{}",
                    if sel == idx {
                        &self.indicator
                    } else {
                        &self.noindicator
                    },
                    if selected[idx] {
                        &self.check
                    } else {
                        &self.nocheck
                    },
                    item,
                ))?;
            }
            match term.read_key()? {
                Key::ArrowDown | Key::Char('j') => {
                    if sel == !0 {
                        sel = 0;
                    } else {
                        sel = (sel as u64 + 1).rem(self.items.len() as u64) as usize;
                    }
                }
                Key::ArrowUp | Key::Char('k') => {
                    if sel == !0 {
                        sel = self.items.len() - 1;
                    } else {
                        sel = ((sel as i64 - 1 + self.items.len() as i64)
                            % (self.items.len() as i64)) as usize;
                    }
                }
                Key::Char('a') => {
                    let b = selected.iter().any(|&x| !x);
                    for e in selected.iter_mut() {
                        *e = b;
                    }
                }
                Key::Char(' ') => {
                    selected[sel] = !selected[sel];
                }
                Key::Escape => {
                    if self.clear {
                        term.clear_last_lines(self.items.len())?;
                    }
                    return Ok(vec![]);
                }
                Key::Enter => {
                    if self.clear {
                        term.clear_last_lines(self.items.len())?;
                    }
                    return Ok(selected
                        .iter()
                        .enumerate()
                        .filter_map(|(idx, &selected)| if selected { Some(idx) } else { None })
                        .collect());
                }
                _ => {}
            }
            term.clear_last_lines(self.items.len())?;
        }
    }
}
