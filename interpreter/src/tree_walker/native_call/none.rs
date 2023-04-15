use super::prelude::*;
use crate::standard_prelude::calls::NativeCallNone;
use numtoa::NumToA;
use std::{
    io::{stdout, Write},
    thread::sleep,
    time::Duration,
};

impl<'a> TreeWalker<'a> {
    pub fn native_call_none(&mut self, call: &NativeCallNone) -> Result<(), TreeWalkerErr> {
        match call {
            NativeCallNone::Print(expr, line) => match &**expr {
                Expr::Integer(expr) => {
                    let mut buffer = [0u8; 20];
                    let int = self.eval_int(expr)?.numtoa(10, &mut buffer);
                    self.stdout.extend_from_slice(int);
                    if *line {
                        self.stdout.extend_from_slice(b"\n");
                    }
                }
                Expr::Float(expr) => {
                    let mut buffer = ryu::Buffer::new();
                    let float = buffer.format(self.eval_float(expr)?).as_bytes();
                    self.stdout.extend_from_slice(float);
                    if *line {
                        self.stdout.extend_from_slice(b"\n");
                    }
                }
                Expr::String(expr) => {
                    let string = self.eval_string(&expr)?;
                    write!(self.stdout, "{}{}", string, if *line { "\n" } else { "" }).unwrap();
                }
                Expr::Boolean(expr) => {
                    let boolean = self.eval_boolean(expr)?;
                    write!(self.stdout, "{}{}", boolean, if *line { "\n" } else { "" }).unwrap();
                }
                _ => panic!("Unprintable type"),
            },

            NativeCallNone::Sleep(duration) => {
                let duration = self.eval_int(duration)?;
                sleep(Duration::from_millis(duration as u64));
                stdout().write_all(&self.stdout.as_slice()).ok();
                stdout().flush().ok();
                self.stdout.clear();
                self.interpreter_event_sender
                    .send(InterpreterEvent::Update)
                    .ok();
            }

            NativeCallNone::SetPage(page) => {
                let mut page = self.eval_object(page)?;

                self.interpreter_event_sender
                    .send(InterpreterEvent::SetPage(Arc::clone(
                        page.extract_native_object().extract_page(),
                    )))
                    .ok();
            }

            NativeCallNone::CloseTab => {
                self.interpreter_event_sender
                    .send(InterpreterEvent::CloseTab)
                    .ok();
                return Err(TreeWalkerErr::Exit);
            }

            NativeCallNone::OpenLink(link) => {
                let link = self.eval_string(&link)?;
                self.interpreter_event_sender
                    .send(InterpreterEvent::OpenLink(link))
                    .ok();
            }
        }

        Ok(())
    }
}
