use std::boxed::Box;
use std::collections::HashMap;
use std::rc::Rc;

pub enum ImplKind {
    SingletonImpl,
    ClassImpl, // Class `ImplKind`s contain ClassSelf `Impl`s
    SelfImpl,  // Should be used to describe the `Self` on a `Class` or an `Type`
    TypeImpl,
    NoneImpl   // Should only be used when `SelfImpl` is not the second parameter
}

struct Impl<'a, T, S> {
    on: T,
    kind: ImplKind,
    sub_impl: S,    
    pub_fns: Box<HashMap<&'a str, Function<'a, ClashType>>>,
    priv_fns: Box<HashMap<&'a str, Function<'a, ClashType>>>
}

struct Arg {
    ident: String,
    annot: Option<String>,
    dfval: Option<String>,
    args: bool,
    kwargs: bool,
    val: Option<String>
}

struct Arguments {
    size: usize,
    args: Box<[Arg]>
}

struct Class<'superctx> {
    ident: String,
    cls_impl: Impl<'superctx, ImplKind, ImplKind>,
    members: &'superctx mut [Vec<ClassMember<'superctx, &'superctx Self>>],
    sub_impl: Impl<'superctx, ImplKind, ImplKind>
}

pub enum FunctionKind {
    Private,
    Public
}

struct ReturnSignature<F> {
    ident: String,
    loc: String,
    func: F
}

struct Return<F,Ty> {
    sig: ReturnSignature<F>,
    value: Option<Ty>
}

struct ClashType {
    ident: String
}

struct Function<'this, R> {
    ident: String,
    status: FunctionKind,
    arguments: Arguments,
    return_type: Return<&'this Self, R>
}
struct Property<'owning_type> {
    ident: String,
    source: &'owning_type u32,
    value: ClashType
}
struct ClassMember<'superctx, C> {
    ident: String,
    cls: &'superctx C,
    properties: Box<[Property<'superctx>]>,
    methods: Box<[Function<'superctx, ClashType>]>,
    links: Box<[&'superctx u32]>
}

fn main() {
    println!("Hello, world!");
}

