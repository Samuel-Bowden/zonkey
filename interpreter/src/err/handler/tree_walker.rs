use super::err_reporter::ErrReporter;
use crate::err::tree_walker::TreeWalkerErr;

pub fn err_handler(err_reporter: &mut ErrReporter, tree_walker_err: TreeWalkerErr) {
    err_reporter.uncaught_exception_prefix();

    match tree_walker_err {
        TreeWalkerErr::PropertyNotInitialised(prop_name) => {
            err_reporter.writeln("PropertyNotInitialised");
            err_reporter.writeln(format!("Attempted to access property with name '{}' without initialising it first.", prop_name.token_type).as_str());
            err_reporter.report_token(prop_name);
        }
        _ => err_reporter.writeln(format!("{:?}", tree_walker_err).as_str()),
    }
}
