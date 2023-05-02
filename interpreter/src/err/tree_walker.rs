use super::err_reporter::ErrReporter;
use crate::tree_walker::err::TreeWalkerErr;

pub fn err_handler(err_reporter: &mut ErrReporter, tree_walker_err: TreeWalkerErr) {
    err_reporter.exception_prefix();

    match &tree_walker_err {
        TreeWalkerErr::PropertyNotInitialised(prop_name) => {
            err_reporter.writeln("PropertyNotInitialised");
            err_reporter.writeln(
                format!(
                    "  Attempted to access property with name '{}' without initialising it first.",
                    prop_name.token_type
                )
                .as_str(),
            );
            err_reporter.report_token(prop_name);
        }
        TreeWalkerErr::IndexOutOfRange(index, len, location) => {
            err_reporter.writeln("IndexOutOfRange");
            err_reporter.writeln(format!(
                    "  Attempted to index an array at position {}, but the array only contains {} values.",
                    index,
                    len,
            ).as_str());
            err_reporter.report_token(location);
        }
        TreeWalkerErr::DivisionByZero(location) => {
            err_reporter.writeln("DivisionByZero");
            err_reporter.writeln(
                "  Attempted to divide the left of this operator by the evaluated value of 0 on the right.",
            );
            err_reporter.report_token(location);
        }
        TreeWalkerErr::FailedStringToIntegerCast(location) => {
            err_reporter.writeln("FailedStringToIntegerCast");
            err_reporter.writeln("  Failed to convert the provided String value into an Integer.");
            err_reporter.report_token(location);
        }
        TreeWalkerErr::FailedStringToFloatCast(location) => {
            err_reporter.writeln("FailedStringToFloatCast");
            err_reporter.writeln("  Failed to convert the provided String value into a Float.");
            err_reporter.report_token(location);
        }
        TreeWalkerErr::InsufficientPermissionLevel => {
            err_reporter.writeln("InsufficientPermissionLevel");
            err_reporter.writeln("  Cannot read or write data to the file system when the script is loaded over a network.");
        }
        _ => err_reporter.writeln(format!("{:?}", tree_walker_err).as_str()),
    }
}
