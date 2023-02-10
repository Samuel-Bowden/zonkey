use interpreter::token::Token;
use std::io::Write;
use termcolor::{Color, ColorSpec, StandardStream, WriteColor};

pub struct ErrReporter<'a> {
    stderr: StandardStream,
    graphemes: &'a Vec<&'a str>,
}

impl<'a> ErrReporter<'a> {
    pub fn new(graphemes: &'a Vec<&'a str>) -> Self {
        Self {
            stderr: StandardStream::stderr(termcolor::ColorChoice::Always),
            graphemes,
        }
    }

    pub fn error_prefix(&mut self) {
        self.stderr
            .set_color(ColorSpec::new().set_fg(Some(Color::Red)))
            .unwrap();
        write!(&mut self.stderr, "(ERROR) ").unwrap();
        self.stderr.reset().unwrap();
    }

    pub fn aborting_prefix(&mut self) {
        self.stderr
            .set_color(ColorSpec::new().set_fg(Some(Color::Blue)))
            .unwrap();
        write!(&mut self.stderr, "(ABORTING) ").unwrap();
        self.stderr.reset().unwrap();
    }

    pub fn uncaught_exception_prefix(&mut self) {
        self.stderr
            .set_color(ColorSpec::new().set_fg(Some(Color::Red)))
            .unwrap();
        write!(&mut self.stderr, "(UNCAUGHT EXCEPTION) ").unwrap();
        self.stderr.reset().unwrap();
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

        self.stderr
            .set_color(ColorSpec::new().set_fg(Some(Color::Magenta)))
            .unwrap();
        write!(&mut self.stderr, "\t{line_num} | ").unwrap();
        self.stderr.reset().unwrap();

        let mut current = line_start;

        loop {
            let grapheme = self.graphemes.get(current);

            match grapheme {
                Some(&"\n") | None => break,
                Some(t) => {
                    if current >= start && current < end {
                        self.stderr
                            .set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))
                            .unwrap();
                        write!(&mut self.stderr, "{}", t).unwrap();
                        self.stderr.reset().unwrap();
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
        self.stderr
            .set_color(ColorSpec::new().set_fg(Some(Color::Green)))
            .unwrap();

        write!(&mut self.stderr, "        Tip: ",).unwrap();

        self.stderr.reset().unwrap();

        writeln!(&mut self.stderr, "{tip}.").unwrap();
    }
}
