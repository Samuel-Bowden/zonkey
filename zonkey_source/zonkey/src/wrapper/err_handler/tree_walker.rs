use super::err_reporter::ErrReporter;
use interpreter::err::tree_walker::TreeWalkerErr;

pub fn err_handler(mut err_reporter: ErrReporter, tree_walker_err: TreeWalkerErr) {
    err_reporter.uncaught_exception_prefix();

    err_reporter.writeln(format!("{:?}", tree_walker_err).as_str());
}
