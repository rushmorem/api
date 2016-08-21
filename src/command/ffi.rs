// Copyright 2015-2016 Intecture Developers. See the COPYRIGHT file at the
// top-level directory of this distribution and at
// https://intecture.io/COPYRIGHT.
//
// Licensed under the Mozilla Public License 2.0 <LICENSE or
// https://www.tldrlegal.com/l/mpl-2.0>. This file may not be copied,
// modified, or distributed except according to those terms.

//! FFI interface for Command

use host::Host;
use host::ffi::Ffi__Host;
use libc::c_char;
use std::convert;
use std::ffi::CString;
use super::{Command, CommandResult};

#[repr(C)]
pub struct Ffi__Command {
    cmd: *const c_char,
}

impl convert::From<Command> for Ffi__Command {
    fn from(command: Command) -> Ffi__Command {
        Ffi__Command {
            cmd: CString::new(command.cmd).unwrap().into_raw(),
        }
    }
}

impl convert::From<Ffi__Command> for Command {
    fn from(ffi_cmd: Ffi__Command) -> Command {
        let cmd = ptrtostr!(ffi_cmd.cmd, "command string").unwrap();
        Command::new(cmd)
    }
}

#[repr(C)]
pub struct Ffi__CommandResult {
    pub exit_code: i32,
    pub stdout: *const c_char,
    pub stderr: *const c_char,
}

impl convert::From<CommandResult> for Ffi__CommandResult {
    fn from(result: CommandResult) -> Ffi__CommandResult {
        Ffi__CommandResult {
            exit_code: result.exit_code,
            stdout: CString::new(result.stdout).unwrap().into_raw(),
            stderr: CString::new(result.stderr).unwrap().into_raw(),
        }
    }
}

#[no_mangle]
pub extern "C" fn command_new(cmd_ptr: *const c_char) -> *mut Ffi__Command {
    let cmd = trynull!(ptrtostr!(cmd_ptr, "path string"));
    Box::into_raw(Box::new(Ffi__Command::from(Command::new(cmd))))
}

#[no_mangle]
pub extern "C" fn command_exec(ffi_cmd_ptr: *mut Ffi__Command, ffi_host_ptr: *mut Ffi__Host) -> *mut Ffi__CommandResult {
    let cmd: Command = trynull!(readptr!(ffi_cmd_ptr, "Command struct"));
    let mut host: Host = trynull!(readptr!(ffi_host_ptr, "Host struct"));

    let result = Ffi__CommandResult::from(trynull!(cmd.exec(&mut host)));

    // Convert ZMQ socket to raw to avoid destructor closing sock
    Ffi__Host::from(host);

    Box::into_raw(Box::new(result))
}

#[cfg(test)]
mod tests {
    use {Command, CommandResult};
    #[cfg(feature = "remote-run")]
    use Host;
    #[cfg(feature = "remote-run")]
    use czmq::{ZMsg, ZSys};
    use error::ERRMSG;
    use host::ffi::Ffi__Host;
    #[cfg(feature = "remote-run")]
    use std::{str, thread};
    use std::ffi::CStr;
    use std::ffi::CString;
    use std::ptr;
    use super::*;

    #[test]
    fn test_convert_command() {
        let command = Command {
            cmd: "whoami".to_string(),
        };
        Ffi__Command::from(command);
    }

    #[test]
    fn test_convert_ffi_command() {
        let ffi_command = Ffi__Command {
            cmd: CString::new("whoami").unwrap().as_ptr(),
        };
        Command::from(ffi_command);
    }

    #[test]
    fn test_convert_command_result() {
        let result = CommandResult {
            exit_code: 0,
            stdout: "moo".to_string(),
            stderr: "cow".to_string(),
        };
        Ffi__CommandResult::from(result);
    }

    #[test]
    fn test_command_new() {
        let cmd_cstr = CString::new("moo").unwrap().as_ptr();
        let ffi_cmd = unsafe { ptr::read(command_new(cmd_cstr)) };
        assert_eq!(ffi_cmd.cmd, cmd_cstr);

        assert!(command_new(ptr::null()).is_null());
        assert_eq!(unsafe { CStr::from_ptr(ERRMSG).to_str().unwrap() }, "Received null when we expected a path string pointer");
    }

    #[cfg(feature = "local-run")]
    #[test]
    fn test_command_exec() {
        let mut host = Ffi__Host;
        let cmd = command_new(CString::new("whoami").unwrap().as_ptr());
        let result = unsafe { ptr::read(command_exec(cmd, &mut host)) };
        assert_eq!(result.exit_code, 0);
    }

    #[cfg(feature = "remote-run")]
    #[test]
    fn test_command_exec() {
        ZSys::init();

        let (client, server) = ZSys::create_pipe().unwrap();

        let agent_mock = thread::spawn(move || {
            let req = ZMsg::recv(&server).unwrap();
            assert_eq!("command::exec", req.popstr().unwrap().unwrap());
            assert_eq!("moo", req.popstr().unwrap().unwrap());

            let rep = ZMsg::new();
            rep.addstr("Ok").unwrap();
            rep.addstr("0").unwrap();
            rep.addstr("cow").unwrap();
            rep.addstr("err").unwrap();
            rep.send(&server).unwrap();
        });

        let mut ffi_host = Ffi__Host::from(Host::test_new(None, Some(client), None));

        let mut ffi_command = Ffi__Command {
            cmd: CString::new("moo").unwrap().as_ptr(),
        };

        let result = unsafe { ptr::read(command_exec(&mut ffi_command, &mut ffi_host)) };

        Host::from(ffi_host);

        assert_eq!(result.exit_code, 0);

        let stdout_slice = unsafe { CStr::from_ptr(result.stdout) };
        let stdout = str::from_utf8(stdout_slice.to_bytes()).unwrap();
        assert_eq!(stdout, "cow");

        let stderr_slice = unsafe { CStr::from_ptr(result.stderr) };
        let stderr = str::from_utf8(stderr_slice.to_bytes()).unwrap();
        assert_eq!(stderr, "err");

        agent_mock.join().unwrap();
    }
}
