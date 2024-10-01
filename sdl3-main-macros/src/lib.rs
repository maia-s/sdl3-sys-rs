#![doc = include_str!("../README.md")]

use parse::{Error, Function, GenericArg, IntoTokenTrees, Parse, Type};
use proc_macro::{Delimiter, Group, Ident, Punct, Spacing, Span, TokenStream, TokenTree};

const SDL3_MAIN: &str = "sdl3_main";

macro_rules! input {
    ($input:expr) => {
        &mut { &$crate::parse::into_input($input) as &[::proc_macro::TokenTree] }
    };
}

macro_rules! miniquote_to {
    ($out:expr =>) => {};

    ($out:expr => #{$expr:expr} $($rest:tt)*) => {{
        $expr.into_token_trees($out);
        miniquote_to!($out => $($rest)*);
    }};

    ($out:expr => #$ident:ident $($rest:tt)*) => {{
        $ident.into_token_trees($out);
        miniquote_to!($out => $($rest)*);
    }};

    ($out:expr => ($($group:tt)*) $($rest:tt)*) => {{
        $out.extend([TokenTree::Group(Group::new(Delimiter::Parenthesis, miniquote!($($group)*)))]);
        miniquote_to!($out => $($rest)*);
    }};

    ($out:expr => [$($group:tt)*] $($rest:tt)*) => {{
        $out.extend([TokenTree::Group(Group::new(Delimiter::Bracket, miniquote!($($group)*)))]);
        miniquote_to!($out => $($rest)*);
    }};

    ($out:expr => {$($group:tt)*} $($rest:tt)*) => {{
        $out.extend([TokenTree::Group(Group::new(Delimiter::Brace, miniquote!($($group)*)))]);
        miniquote_to!($out => $($rest)*);
    }};

    ($out:expr => # $($rest:tt)*) => {{
        $out.extend($crate::op1('#'));
        miniquote_to!($out => $($rest)*);
    }};

    ($out:expr => != $($rest:tt)*) => {
        $out.extend($crate::op2('!', '='));
        miniquote_to!($out => $($rest)*);
    };

    ($out:expr => ! $($rest:tt)*) => {{
        $out.extend($crate::op1('!'));
        miniquote_to!($out => $($rest)*);
    }};

    ($out:expr => == $($rest:tt)*) => {{
        $out.extend($crate::op2('=', '='));
        miniquote_to!($out => $($rest)*);
    }};

    ($out:expr => => $($rest:tt)*) => {{
        $out.extend($crate::op2('=', '>'));
        miniquote_to!($out => $($rest)*);
    }};

    ($out:expr => = $($rest:tt)*) => {{
        $out.extend($crate::op1('='));
        miniquote_to!($out => $($rest)*);
    }};

    ($out:expr => ; $($rest:tt)*) => {{
        $out.extend($crate::op1(';'));
        miniquote_to!($out => $($rest)*);
    }};

    ($out:expr => . $($rest:tt)*) => {{
        $out.extend($crate::op1('.'));
        miniquote_to!($out => $($rest)*);
    }};

    ($out:expr => :: $($rest:tt)*) => {{
        $out.extend($crate::op2(':', ':'));
        miniquote_to!($out => $($rest)*);
    }};

    ($out:expr => : $($rest:tt)*) => {{
        $out.extend($crate::op1(':'));
        miniquote_to!($out => $($rest)*);
    }};

    ($out:expr => * $($rest:tt)*) => {{
        $out.extend($crate::op1('*'));
        miniquote_to!($out => $($rest)*);
    }};

    ($out:expr => + $($rest:tt)*) => {{
        $out.extend($crate::op1('+'));
        miniquote_to!($out => $($rest)*);
    }};

    ($out:expr => -> $($rest:tt)*) => {{
        $out.extend($crate::op2('-', '>'));
        miniquote_to!($out => $($rest)*);
    }};

    ($out:expr => & $($rest:tt)*) => {{
        $out.extend($crate::op1('&'));
        miniquote_to!($out => $($rest)*);
    }};

    ($out:expr => || $($rest:tt)*) => {{
        $out.extend($crate::op2('|', '|'));
        miniquote_to!($out => $($rest)*);
    }};

    ($out:expr => | $($rest:tt)*) => {{
        $out.extend($crate::op1('|'));
        miniquote_to!($out => $($rest)*);
    }};

    ($out:expr => , $($rest:tt)*) => {{
        $out.extend($crate::op1(','));
        miniquote_to!($out => $($rest)*);
    }};

    ($out:expr => < $($rest:tt)*) => {{
        $out.extend($crate::op1('<'));
        miniquote_to!($out => $($rest)*);
    }};

    ($out:expr => >> $($rest:tt)*) => {{
        $out.extend($crate::op2('>', '>'));
        miniquote_to!($out => $($rest)*);
    }};

    ($out:expr => > $($rest:tt)*) => {{
        $out.extend($crate::op1('>'));
        miniquote_to!($out => $($rest)*);
    }};

    ($out:expr => _ $($rest:tt)*) => {{
        $out.extend([TokenTree::Ident(Ident::new("_", Span::mixed_site()))]);
        miniquote_to!($out => $($rest)*);
    }};

    ($out:expr => $ident:ident $($rest:tt)*) => {{
        $out.extend([TokenTree::Ident(Ident::new(stringify!($ident), Span::mixed_site()))]);
        miniquote_to!($out => $($rest)*);
    }};

    ($out:expr => $lt:lifetime $($rest:tt)*) => {{
        stringify!($lt).parse::<TokenStream>().unwrap().into_token_trees($out);
        miniquote_to!($out => $($rest)*);
    }};

    ($out:expr => $lit:literal $($rest:tt)*) => {{
        $out.extend(stringify!($lit).parse::<TokenStream>().unwrap());
        miniquote_to!($out => $($rest)*);
    }};

    ($out:expr => $tt:tt $($rest:tt)*) => {{
        // match `$`
        miniquote_to!(@@@(-$tt-); $out => $($rest)*);
    }};

    (@@@($(-)$+); $out:expr => $($rest:tt)*) => {{
        // match `$`
        $out.extend($crate::op1('$'));
        miniquote_to!($out => $($rest)*);
    }};
}

macro_rules! miniquote {
    ($($tt:tt)*) => {{
        #[allow(unused_mut)]
        let mut out = TokenStream::new();
        miniquote_to!(&mut out => $($tt)*);
        out
    }};
}

mod parse;

fn op1(c0: char) -> [TokenTree; 1] {
    [TokenTree::Punct(Punct::new(c0, Spacing::Alone))]
}

fn op2(c0: char, c1: char) -> [TokenTree; 2] {
    [
        TokenTree::Punct(Punct::new(c0, Spacing::Joint)),
        TokenTree::Punct(Punct::new(c1, Spacing::Alone)),
    ]
}

fn sdl3_main_path() -> TokenStream {
    miniquote!(::#{Ident::new(SDL3_MAIN, Span::mixed_site())})
}

fn sdl3_main_internal_path() -> TokenStream {
    miniquote!(#{sdl3_main_path()}::__internal)
}

fn sdl3_sys_path() -> TokenStream {
    miniquote!(#{sdl3_main_internal_path()}::sdl3_sys)
}

fn priv_ident(kind: &str, name: &str) -> Ident {
    Ident::new(&format!("__sdl3_main_{kind}_{name}"), Span::mixed_site())
}

fn app_raw_fn_ident(name: &str) -> Ident {
    priv_ident("fnp", name)
}

fn app_fn_ident(name: &str) -> Ident {
    priv_ident("fn", name)
}

fn app_type_ident(name: &str) -> Ident {
    priv_ident("t", name)
}

fn app_fn(name: &str, f: &Function) -> Result<TokenStream, Error> {
    Ok(miniquote! {
        mod #{&f.ident} {}
        #[allow(non_upper_case_globals)]
        const #{app_raw_fn_ident(name)}: #{f.signature()} = const {
            #f
            #{&f.ident}
        };
    })
}

fn wrap(
    name: &str,
    attr: TokenStream,
    item: TokenStream,
    f: impl FnOnce(Function, TokenStream) -> Result<TokenStream, Error>,
) -> TokenStream {
    let attr = input!(attr);
    if !attr.is_empty() {
        Error::new(
            Some(attr.first().unwrap().span()),
            format!("other attributes aren't supported with `#[{name}]`"),
        )
        .into_token_stream()
    } else {
        match Function::parse(input!(item))
            .and_then(|item| {
                let ts = app_fn(name, &item)?;
                Ok((item, ts))
            })
            .and_then(|(item, ts)| f(item, ts))
        {
            Ok(ts) => ts,
            Err(err) => err.into_token_stream(),
        }
    }
}

fn shuttle_unsafe() -> TokenStream {
    if cfg!(feature = "std") {
        miniquote!(unsafe)
    } else {
        miniquote!()
    }
}

fn shuttle_unit_def() -> TokenStream {
    if cfg!(feature = "std") {
        miniquote! {
            #[allow(non_upper_case_globals)]
            static #{priv_ident("static", "shuttle_unit")}:
                #{sdl3_main_internal_path()}::Shuttle<()> =
                #{sdl3_main_internal_path()}::Shuttle::new();
        }
    } else {
        miniquote!()
    }
}

fn shuttle_unit_capture_and_continue() -> TokenStream {
    if cfg!(feature = "std") {
        miniquote!(#{priv_ident("static", "shuttle_unit")}.capture_and_continue)
    } else {
        miniquote!((|_, f| f()))
    }
}

fn shuttle_unit_capture() -> TokenStream {
    if cfg!(feature = "std") {
        miniquote!(#{priv_ident("static", "shuttle_unit")}.capture)
    } else {
        miniquote!((|f| f()))
    }
}

fn shuttle_unit_resume() -> TokenStream {
    if cfg!(feature = "std") {
        miniquote!(unsafe { #{priv_ident("static", "shuttle_unit")}.resume() })
    } else {
        miniquote!()
    }
}

#[proc_macro_attribute]
pub fn main(attr: TokenStream, item: TokenStream) -> TokenStream {
    wrap("main", attr, item, |f, main| {
        let app_main = app_raw_fn_ident("main");

        let simple_return = if let Some(rtype) = &f.return_type {
            rtype.is_ident_no_gen("bool") || rtype.is_ident_no_gen("c_int")
        } else {
            true
        };

        if simple_return {
            if cfg!(feature = "std") {
                Ok(miniquote! {
                    #main

                    #{shuttle_unit_def()}

                    fn main() -> ::core::result::Result<(), &'static ::core::ffi::CStr> {
                        use ::core::{any::Any, ffi::{c_char, c_int, CStr}, option::Option, result::Result};
                        use ::std::panic::catch_unwind;
                        use #{sdl3_main_path()}::{app::AppMain, MainThreadToken};
                        use #{sdl3_main_internal_path()}::{Shuttle, run_app};
                        use #{sdl3_sys_path()}::error::SDL_GetError;

                        unsafe extern "C" fn sdl_main(argc: c_int, argv: *mut *mut c_char) -> c_int {
                            unsafe {
                                #{shuttle_unit_capture_and_continue()}(
                                    1,
                                    || unsafe { #app_main.main(MainThreadToken::assert(), argc, argv) },
                                )
                            }
                        }

                        unsafe {
                            // safety: this is the main thread
                            MainThreadToken::init();
                        }

                        if unsafe { run_app(sdl_main) } == 0 {
                            Result::Ok(())
                        } else {
                            #{shuttle_unit_resume()}
                            Result::Err(unsafe { CStr::from_ptr(SDL_GetError()) })
                        }
                    }
                })
            } else {
                // no_std main
                Ok(miniquote! {
                    #main

                    #[unsafe(no_mangle)]
                    extern "C" fn main(argc: ::core::ffi::c_int, argv: *mut *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
                        use ::core::{ffi::{c_char, c_int}, option::Option, ptr};
                        use #{sdl3_main_path()}::{app::AppMain, MainThreadToken};
                        use #{sdl3_sys_path()}::main::SDL_RunApp;

                        unsafe extern "C" fn sdl_main(argc: c_int, argv: *mut *mut c_char) -> c_int {
                            unsafe { #app_main.main(MainThreadToken::assert(), argc, argv) }
                        }

                        unsafe {
                            // safety: this is the main thread
                            MainThreadToken::init();
                        }

                        unsafe { SDL_RunApp(argc, argv, Option::Some(sdl_main), ptr::null_mut()) }
                    }
                })
            }
        } else {
            if !cfg!(feature = "std") {
                return Err(Error::new(
                    Some(f.ident.span()),
                    "main return types other than `()`, `bool` or `c_int` require the `std` feature",
                ));
            }
            let rtype = &f.return_type.unwrap();
            Ok(miniquote! {
                #main

                fn main() -> #rtype {
                    use ::core::{ffi::{c_char, c_int}, mem::MaybeUninit, ptr::{addr_of, addr_of_mut}};
                    use #{sdl3_main_path()}::{app::AppMainWithResult, MainThreadToken};
                    use #{sdl3_main_internal_path()}::{Shuttle, run_app};

                    static SHUTTLE: Shuttle<#rtype> = Shuttle::new();

                    unsafe extern "C" fn sdl_main(argc: c_int, argv: *mut *mut c_char) -> c_int {
                        unsafe {
                            SHUTTLE.capture(
                                || #app_main.main_with_result(MainThreadToken::assert(), argc, argv)
                            );
                        };
                        0
                    }

                    unsafe {
                        // safety: this is the main thread
                        MainThreadToken::init();
                    }

                    unsafe {
                        run_app(sdl_main);
                        SHUTTLE.resume()
                    }
                }
            })
        }
    })
}

#[proc_macro_attribute]
pub fn app_init(attr: TokenStream, item: TokenStream) -> TokenStream {
    wrap("app_init", attr, item, |f, init| {
        let mut state = Type::unit();
        if let Some(rtype) = &f.return_type {
            if let Some(generics) = rtype.path_generics() {
                if generics.args.len() == 1 {
                    if let GenericArg::Type(t) = &generics.args[0] {
                        state = t.clone();
                    }
                }
            }
        }

        let state_t = &app_type_ident("AppState");

        Ok(miniquote! {
            #init

            #[allow(non_camel_case_types)]
            type #state_t = #state;

            #[#{sdl3_main_path()}::main]
            unsafe fn __sdl3_main_callbacks(argc: ::core::ffi::c_int, argv: *mut *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
                use ::core::ffi::{c_char, c_int, c_void};
                use #{sdl3_sys_path()}::{events::SDL_Event, init::SDL_AppResult, main::SDL_EnterAppMainCallbacks};
                use #{sdl3_main_path()}::{app::{AppEvent, AppInit, AppIterate, AppQuit}, MainThreadToken};

                unsafe extern "C" fn app_init(
                    appstate: *mut *mut c_void, argc: c_int, argv: *mut *mut c_char
                ) -> SDL_AppResult {
                    #{shuttle_unsafe()} {
                        #{shuttle_unit_capture_and_continue()}(
                            SDL_AppResult::FAILURE,
                            || AppInit::<#state_t>::init(
                                #{app_raw_fn_ident("app_init")},
                                MainThreadToken::assert(),
                                appstate,
                                argc,
                                argv
                            )
                        )
                    }
                }

                let st = unsafe {
                    SDL_EnterAppMainCallbacks(
                        argc,
                        argv,
                        Option::Some(app_init),
                        Option::Some(#{app_fn_ident("app_iterate")}),
                        Option::Some(#{app_fn_ident("app_event")}),
                        Option::Some(#{app_fn_ident("app_quit")}),
                    )
                };
                #{shuttle_unit_resume()}
                st
            }
        })
    })
}

#[proc_macro_attribute]
pub fn app_iterate(attr: TokenStream, item: TokenStream) -> TokenStream {
    let name = "app_iterate";
    wrap(name, attr, item, |f, ts| {
        let state_ac = match f.params.len() {
            1 => f.params[0].ty.classify()? as u8,
            _ => 0,
        };
        Ok(miniquote! {
            #ts

            unsafe extern "C" fn #{app_fn_ident(name)}(
                appstate: *mut ::core::ffi::c_void
            ) -> #{sdl3_sys_path()}::init::SDL_AppResult {
                #{shuttle_unsafe()} {
                    #{shuttle_unit_capture_and_continue()}(
                        #{sdl3_sys_path()}::init::SDL_AppResult::FAILURE,
                        || #{sdl3_main_path()}::app::AppIterate::<#{app_type_ident("AppState")}, #state_ac>::iterate(
                            #{app_raw_fn_ident(name)},
                            appstate
                        )
                    )
                }
            }
        })
    })
}

#[proc_macro_attribute]
pub fn app_event(attr: TokenStream, item: TokenStream) -> TokenStream {
    let name = "app_event";
    wrap(name, attr, item, |f, ts| {
        let (state_ac, event_ac) = match f.params.len() {
            1 => (0, f.params[0].ty.classify()? as u8),
            2 => (
                f.params[0].ty.classify()? as u8,
                f.params[1].ty.classify()? as u8,
            ),
            _ => (0, 0),
        };
        Ok(miniquote! {
            #ts

            unsafe extern "C" fn #{app_fn_ident(name)}(
                appstate: *mut ::core::ffi::c_void,
                event: *mut #{sdl3_sys_path()}::events::SDL_Event
            ) -> #{sdl3_sys_path()}::init::SDL_AppResult {
                #{shuttle_unsafe()} {
                    #{shuttle_unit_capture_and_continue()}(
                        #{sdl3_sys_path()}::init::SDL_AppResult::FAILURE,
                        || #{sdl3_main_path()}::app::AppEvent::<#{app_type_ident("AppState")}, #state_ac, #event_ac>::event(
                            #{app_raw_fn_ident(name)},
                            appstate,
                            event
                        )
                    )
                }
            }
        })
    })
}

#[proc_macro_attribute]
pub fn app_quit(attr: TokenStream, item: TokenStream) -> TokenStream {
    let name = "app_quit";
    wrap(name, attr, item, |f, ts| {
        let state_ac = match f.params.len() {
            1 | 2 => f.params[0].ty.classify()? as u8,
            _ => 0,
        };
        Ok(miniquote! {
            #ts

            unsafe extern "C" fn #{app_fn_ident(name)}(
                appstate: *mut ::core::ffi::c_void,
                result: #{sdl3_sys_path()}::init::SDL_AppResult
            ) {
                #{shuttle_unsafe()} {
                    #{shuttle_unit_capture()}(
                        || #{sdl3_main_path()}::app::AppQuit::<#{app_type_ident("AppState")}, #state_ac>::quit(
                            #{app_raw_fn_ident(name)},
                            appstate,
                            result
                        )
                    )
                }
            }
        })
    })
}
