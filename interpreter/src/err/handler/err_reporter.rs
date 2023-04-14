use crate::token::Token;
use std::fmt::Write;

pub struct ErrReporter<'a> {
    pub stderr: String,
    graphemes: &'a Vec<&'a str>,
}

impl<'a> ErrReporter<'a> {
    pub fn new(graphemes: &'a Vec<&'a str>) -> Self {
        Self {
            stderr: String::new(),
            graphemes,
        }
    }

    pub fn error_prefix(&mut self) {
        write!(&mut self.stderr, "(ERROR) ").unwrap();
    }

    pub fn aborting_prefix(&mut self) {
        write!(&mut self.stderr, "(ABORTING) ").unwrap();
    }

    pub fn uncaught_exception_prefix(&mut self) {
        write!(&mut self.stderr, "(UNCAUGHT EXCEPTION) ").unwrap();
    }

    pub fn writeln(&mut self, line: &str) {
        writeln!(&mut self.stderr, "{line}").unwrap();
    }

    pub fn newln(&mut self) {
        writeln!(&mut self.stderr).unwrap();
    }

    pub fn get_grapheme(&self, position: usize) -> &str {
        self.graphemes[position]
    }

    pub fn report_token(&mut self, token: Token) {
        self.report_section(token.start, token.end);
    }

    pub fn report_section(&mut self, start: usize, end: usize) {
        let (line_num, line_start) = self.find_line(start);

        write!(&mut self.stderr, "\t{line_num} | ").unwrap();

        let mut current = line_start;

        loop {
            let grapheme = self.graphemes.get(current);

            match grapheme {
                Some(&"\n") | None => {
                    if current == end {
                        write!(&mut self.stderr, ">").unwrap();
                    }
                    break;
                }
                Some(t) => {
                    if current == start {
                        write!(&mut self.stderr, "<{}", t).unwrap();
                    } else if current == end {
                        write!(&mut self.stderr, ">{}", t).unwrap();
                    } else {
                        write!(&mut self.stderr, "{}", t).unwrap();
                    }
                }
            }

            current += 1;
        }

        writeln!(&mut self.stderr).unwrap();
    }

    pub fn find_line(&self, start: usize) -> (usize, usize) {
        let mut line_num = 1;
        let mut line_start = 0;

        for (i, grapheme) in self.graphemes.iter().enumerate() {
            if start < i {
                break;
            } else if grapheme == &"\n" {
                line_num += 1;
                line_start = i + 1;
            }
        }

        (line_num, line_start)
    }

    pub fn report_next_token(&mut self, token: Option<Token>) {
        if let Some(token) = token {
            writeln!(
                &mut self.stderr,
                "        But the next token was '{}'.",
                token.token_type
            )
            .unwrap();
            self.report_token(token);
        } else {
            writeln!(
                &mut self.stderr,
                "        But the end of the file was reached.",
            )
            .unwrap();
        }
    }

    pub fn give_tip(&mut self, tip: &str) {
        write!(&mut self.stderr, "        Tip: ",).unwrap();

        writeln!(&mut self.stderr, "{tip}").unwrap();
    }
}
