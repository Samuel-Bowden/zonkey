use super::prelude::*;
use crate::{permission::PermissionLevel, standard_prelude::calls::NativeCallNone, Address};
use directories_next::{ProjectDirs, UserDirs};
use numtoa::NumToA;
use std::{
    io::{stdout, Write},
    path::PathBuf,
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

            NativeCallNone::OpenLink(link, arguments) => {
                let link = self.eval_string(&link)?;
                let mut arguments_obj = self.eval_object(&arguments)?;

                let arguments = arguments_obj
                    .extract_native_object()
                    .extract_string_array()
                    .lock()
                    .unwrap();

                self.interpreter_event_sender
                    .send(InterpreterEvent::OpenLink(link, arguments.clone()))
                    .ok();
            }

            NativeCallNone::RemoveApplication(application_location) => {
                let application_location = self.eval_string(application_location)?;

                if let PermissionLevel::NetworkOnly = self.permission_level {
                    return Err(TreeWalkerErr::InsufficientPermissionLevel);
                }

                let error = |msg: &str| {
                    Err(TreeWalkerErr::SettingsFailed(format!(
                        "Failed to remove application: {msg}"
                    )))
                };

                let Some(user_dirs) = UserDirs::new() else { return error("Couldn't open user folder to find desktop directory") };

                let Some(desktop_dir) = user_dirs.desktop_dir() else { return error("Couldn't open desktop folder to remove shortcut") };

                let path = PathBuf::from(application_location.clone());

                let Some(file_name) = path.file_name() else { return error("Couldn't obtain file name of desktop shortcut")};

                let mut shortcut_path = desktop_dir.join(file_name);

                #[cfg(target_os = "linux")]
                shortcut_path.set_extension("desktop");

                #[cfg(target_os = "windows")]
                shortcut_path.set_extension("lnk");

                println!("Removing shortcut from desktop: {:?}", shortcut_path);

                // Shortcut may or may not have been added at install, or could have possible been
                // moved
                std::fs::remove_file(shortcut_path).ok();

                println!(
                    "Removing directory from zonkey data: {}",
                    application_location
                );

                let Ok(()) = std::fs::remove_dir_all(application_location) else { return error("Couldn't remove application directory") };
            }

            NativeCallNone::InstallApplication(arguments, shortcut_desired) => {
                let mut arguments_obj = self.eval_object(arguments)?;

                let arguments = arguments_obj
                    .extract_native_object()
                    .extract_string_array()
                    .lock()
                    .unwrap();

                println!("Arguments: {:?}", arguments);

                let mut arguments_iter = arguments.iter();

                let shortcut_desired = self.eval_boolean(shortcut_desired)?;

                if let PermissionLevel::NetworkOnly = self.permission_level {
                    return Err(TreeWalkerErr::InsufficientPermissionLevel);
                }

                let error = |msg: &str| {
                    Err(TreeWalkerErr::InstallFailed(format!(
                        "Failed to install application: {msg}"
                    )))
                };

                let Some(application_id) = arguments_iter.next() else { return error("Expected application id as argument to install application.") };

                let Some(_windows_shortcut) = arguments_iter.next() else { return error("Expected windows shortcut as argument to install application.") };

                let Some(_linux_shortcut) = arguments_iter.next() else { return error("Expected linux shortcut to install application.") };

                let Some(proj_dirs) = ProjectDirs::from("rocks.sambowden", "",  "zonkey") else { return error("Couldn't find zonkey project directory.") };

                let data_dir = proj_dirs.data_dir().join(application_id);

                println!("Ensuring data directory is created at {:?}", data_dir);
                let Ok(()) = std::fs::create_dir_all(data_dir.clone()) else { return error("Couldn't create zonkey data directory.") };

                for file in arguments_iter {
                    let path = PathBuf::from(file);
                    let data = match Address::new(file, vec![]).load_bytes() {
                        Ok(d) => d,
                        Err(e) => return Err(TreeWalkerErr::InstallFailed(e.to_string())),
                    };

                    let Some(file_name) = path.file_name() else { return error("Couldn't obtain file name of required file to be installed from address.")};

                    let file_path = data_dir.join(file_name);

                    println!("Installing file to {:?}", file_path);
                    let Ok(()) = std::fs::write(file_path, data) else { return error("Couldn't save the required files inside application folder.")};
                }

                if shortcut_desired {
                    let Some(user_dirs) = UserDirs::new() else { return error("Couldn't open user folder to find desktop directory") };
                    let Some(desktop_dir) = user_dirs.desktop_dir() else { return error("Couldn't open desktop folder to remove shortcut") };

                    #[cfg(target_os = "linux")]
                    let shortcut = _linux_shortcut;

                    #[cfg(target_os = "windows")]
                    let shortcut = _windows_shortcut;

                    let path = PathBuf::from(shortcut);

                    let data = match Address::new(&shortcut, vec![]).load_bytes() {
                        Ok(d) => d,
                        Err(e) => return Err(TreeWalkerErr::InstallFailed(e.to_string())),
                    };

                    let Some(file_name) = path.file_name() else { return error("Couldn't obtain file name of desktop shortcut")};

                    let shortcut_path = desktop_dir.join(file_name);

                    println!("Installing shortcut to {:?}", shortcut_path);

                    let Ok(()) = std::fs::write(shortcut_path.clone(), data) else { return error("Couldn't create the desktop file on users desktop.")};

                    #[cfg(target_os = "linux")]
                    {
                        use std::process::Command;

                        // Make sure desktop file is executable
                        use std::os::unix::fs::PermissionsExt;
                        std::fs::set_permissions(
                            shortcut_path.clone(),
                            std::fs::Permissions::from_mode(0o700),
                        )
                        .ok();

                        // Make sure the desktop file is allowed to be launched with gnome
                        Command::new("gio")
                            .arg("set")
                            .arg(shortcut_path)
                            .arg("metadata::trusted")
                            .arg("true")
                            .spawn()
                            .ok();
                    }
                }
            }
        }

        Ok(())
    }
}
