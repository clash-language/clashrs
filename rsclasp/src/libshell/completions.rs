/// libshell/completions.rs
/// The purpose of this library is two-fold. It provides a common
/// interface for libenv/env.rs to derive available commands
/// to call on a given system, and also serves as a foundation for 
/// tab-completion capabilities in the Clash shell terminal session.
///
/// As a result, this file serves a high-level API for retrieving 
/// callable OS-commands after the runtime matches on the 
/// appropriate operating system.
/// 
/// For more information, see the BASH source mirror at:
/// https://github.com/bminor/bash/blob/d233b485e83c3a784b803fb894280773f16f2deb/pcomplete.h
#[cfg(target_os = "linux")]
struct CompletionSpec {
    refcount:  i32,
    actions:   f64,
    options:   f64,
    globpat:   String,
    words:     String,
    prefix:    String,
    suffix:    String,
    funcname:  String,
    lcommand:  String,
    filterpat: String
}

/* "Values for COMPSPEC actions.  
   These are things the shell knows how to
   build internally." - BASH (pcomplete.h) */

static CA_ALIAS     : i32 = 1 << 0;
static CA_ARRAYVAR  : i32 = 1 << 1;
static CA_BINDING   : i32 = 1 << 2;
static CA_BUILTIN   : i32 = 1 << 3;
static CA_COMMAND   : i32 = 1 << 4;
static CA_DIRECTORY : i32 = 1 << 5;
static CA_DISABLED  : i32 = 1 << 6;
static CA_ENABLED   : i32 = 1 << 7;
static CA_EXPORT    : i32 = 1 << 8;
static CA_FILE      : i32 = 1 << 9;
static CA_FUNCTION  : i32 = 1 << 10;
static CA_GROUP     : i32 = 1 << 11;
static CA_HELPTOPIC : i32 = 1 << 12;
static CA_HOSTNAME  : i32 = 1 << 13;
static CA_JOB       : i32 = 1 << 14;
static CA_KEYWORD   : i32 = 1 << 15;
static CA_RUNNING   : i32 = 1 << 16;
static CA_SERVICE   : i32 = 1 << 17;
static CA_SETOPT    : i32 = 1 << 18;
static CA_SHOPT     : i32 = 1 << 19;
static CA_SIGNAL    : i32 = 1 << 20;
static CA_STOPPED   : i32 = 1 << 21;
static CA_USER      : i32 = 1 << 22;
static CA_VARIABLE  : i32 = 1 << 23;

// Values for COMPSEC options field
const COPT_RESERVED    : i32 = 1 << 0;
const COPT_DEFAULT     : i32 = 1 << 1;
const COPT_FILENAMES   : i32 = 1 << 2;
const COPT_DIRNAMES    : i32 = 1 << 3;
const COPT_NOQUOTE     : i32 = 1 << 4;
const COPT_NOSPACE     : i32 = 1 << 5;
const COPT_BASHDEFAULT : i32 = 1 << 6;
const COPT_PLUSDIRS    : i32 = 1 << 7;
const COPT_NOSORT      : i32 = 1 << 8;
const COPT_LASTUSER    : i32 = COPT_NOSORT;
const PCOMP_RETRYFAIL  : i32 = COPT_LASTUSER << 1;
const PCOMP_NOTFOUND   : i32 = COPT_LASTUSER << 2;


use std::fmt;
/// Program
/// A polymorphic spin on the "_list_of_items" implementation
/// in the pcomplete.h source, where instead of supplying a list
/// of items to the "__P" function to retrieve the list of mapped
/// actions, we instead define a Program trait and use it as an
/// interface for programs to be callable provided that they implement
/// `Identity`, have an `Target`, accept some postive integer number of
/// `Argument`'s, and describe how to `Format` the output.
enum Number {
    i32,
    i64,
    u32,
    u64,
    f32,
    f64,
    usize,
    isize
}

struct RawArgument<'a> {
    value: Option<&'a str>
}

impl RawArgument<'a> {
    pub fn new(value: &'a value) -> RawArgument {
        RawArgument {
            value: Some(value)
        }
    }
}

struct Argument<'a> {
    index: &'a Option<u32>,
    value: &'a Option<str>,
    name: &'a Option<str>
}

impl<'a> Argument<'a> {
    pub fn new(
}

impl<'a> Argument<'a> {
    pub fn new(value: Option<&'a str>, index: &'a i32, name: &'a str) -> Argument<'a> {
        let raw = match value {
            Some(value) => 
        Argument {
            name: Some(name),
            index: Some(index),
            value: 
    }

struct ArgumentList<'a> {
    items: &'a [Argument<'a>]
}

impl ArgumentList<'a> {
    pub fn new(args: &'a Vec<str>) -> ArgumentList<'a> {
        let mut arg_list = [
    }
} 

pub trait Program {
    type Executable;
    fn run(&self) -> Option<Self::Executable>;
    fn fmt(&self) -> Option<&str>;
    fn get_result(&self) -> Result<
}

