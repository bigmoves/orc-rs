#[phase(plugin, link)]
extern crate "std" as std;
extern crate "native" as rt;
#[prelude_import]
use std::prelude::*;
pub struct Path {
    pub collection: String,
    pub key: String,
    pub ref_: Option<String>,
}
#[automatically_derived]
impl ::std::fmt::Show for Path {
    fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Path {
            collection: ref __self_0_0,
            key: ref __self_0_1,
            ref_: ref __self_0_2 } =>
            match (&(*__self_0_0), &(*__self_0_1), &(*__self_0_2)) {
                (__arg0, __arg1, __arg2) => {
                    #[inline]
                    #[allow(dead_code)]
                    static __STATIC_FMTSTR: [&'static str, ..4u] =
                        ["Path { collection: ", ", key: ", ", ref_: ", " }"];
                    let __args_vec =
                        &[::std::fmt::argument(::std::fmt::Show::fmt, __arg0),
                          ::std::fmt::argument(::std::fmt::Show::fmt, __arg1),
                          ::std::fmt::argument(::std::fmt::Show::fmt,
                                               __arg2)];
                    let __args =
                        unsafe {
                            ::std::fmt::Arguments::new(__STATIC_FMTSTR,
                                                       __args_vec)
                        };
                    __arg_0.write_fmt(&__args)
                }
            },
        }
    }
}
#[automatically_derived]
impl <__D: ::serialize::Decoder<__E>, __E> ::serialize::Decodable<__D, __E>
     for Path {
    fn decode(__arg_0: &mut __D) -> ::std::result::Result<Path, __E> {
        __arg_0.read_struct("Path", 3u,
                            |_d|
                                ::std::result::Ok(Path{collection:
                                                           match _d.read_struct_field("collection",
                                                                                      0u,
                                                                                      |_d|
                                                                                          ::serialize::Decodable::decode(_d))
                                                               {
                                                               Ok(__try_var)
                                                               => __try_var,
                                                               Err(__try_var)
                                                               =>
                                                               return Err(__try_var),
                                                           },
                                                       key:
                                                           match _d.read_struct_field("key",
                                                                                      1u,
                                                                                      |_d|
                                                                                          ::serialize::Decodable::decode(_d))
                                                               {
                                                               Ok(__try_var)
                                                               => __try_var,
                                                               Err(__try_var)
                                                               =>
                                                               return Err(__try_var),
                                                           },
                                                       ref_:
                                                           match _d.read_struct_field("ref",
                                                                                      2u,
                                                                                      |_d|
                                                                                          ::serialize::Decodable::decode(_d))
                                                               {
                                                               Ok(__try_var)
                                                               => __try_var,
                                                               Err(__try_var)
                                                               =>
                                                               return Err(__try_var),
                                                           },}))
    }
}
#[automatically_derived]
impl <__S: ::serialize::Encoder<__E>, __E> ::serialize::Encodable<__S, __E>
     for Path {
    fn encode(&self, __arg_0: &mut __S) -> ::std::result::Result<(), __E> {
        match *self {
            Path {
            collection: ref __self_0_0,
            key: ref __self_0_1,
            ref_: ref __self_0_2 } =>
            __arg_0.emit_struct("Path", 3u, |_e| {
                                match _e.emit_struct_field("collection", 0u,
                                                           |_e|
                                                               (*__self_0_0).encode(_e))
                                    {
                                    Ok(__try_var) => __try_var,
                                    Err(__try_var) => return Err(__try_var),
                                };
                                match _e.emit_struct_field("key", 1u,
                                                           |_e|
                                                               (*__self_0_1).encode(_e))
                                    {
                                    Ok(__try_var) => __try_var,
                                    Err(__try_var) => return Err(__try_var),
                                };
                                return _e.emit_struct_field("ref", 2u,
                                                            |_e|
                                                                (*__self_0_2).encode(_e));
                            }),
        }
    }
}
