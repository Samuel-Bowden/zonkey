use super::err_reporter::ErrReporter;
use crate::err::tree_walker::TreeWalkerErr;

pub fn err_handler(mut err_reporter: ErrReporter, tree_walker_err: TreeWalkerErr) {
    err_reporter.uncaught_exception_prefix();
    err_reporter.writeln(format!("{:?}", tree_walker_err).as_str());

    match tree_walker_err {
        TreeWalkerErr::FailedToSendEventToBrowser => {
            err_reporter.give_tip("You cannot use APIs specific to the browser (such as the GUI API) when running a program
                through the command line interface to Zonkey.")
        }
        _ => ()
    }
}
