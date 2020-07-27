/// libenv/lib.rs
/// This module is responsible for bringing the names
/// and paths of all shell-executable programs into
/// the static global namespace.
///
/// The idea here is to leverage pre-existing available
/// on the OS's `PATH` environment, and make those names
/// static, immutable FnOnce targets.
///
/// In the future, it may be appealing or necessary to reinvent the
/// wheel for certain utilities like `cd`, `ls`, or `rm`,
/// but for the time being this library aims to grab all utilities
/// that are available on the system.
///
/// Going forward, there will likely be a need to intelligently 
/// look ahead during the compile process to identify all external
/// command-line executable names being used either create symlinks
/// to those utilities in a nested directory of the project; or even 
/// a need to duplicate and minify the external binary when compiling 
/// a production build.
mod libenv;

