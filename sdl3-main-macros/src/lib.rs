#![doc = include_str!("../README.md")]

use parse::{
    Error, Function, GenericArg, ImplBlock, IntoTokenTrees, Item, Parse, Type, Visibility,
};
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

fn app_fn(
    name: &str,
    attr: TokenStream,
    item: TokenStream,
    f: impl FnOnce(&mut TokenStream, Function) -> Result<(), Error>,
) -> TokenStream {
    wrap(attr, item, |out, attr, item| {
        if !attr.is_empty() {
            Err(Error::new(
                Some(attr.first().unwrap().span()),
                format!("other attributes aren't supported with `#[{name}]`"),
            ))
        } else {
            let item = Function::parse_all(item)?;
            if let Some(abi) = &item.abi {
                return Err(Error::new(
                    Some(abi.span),
                    "this function shouldn't set an ABI",
                ));
            }
            miniquote_to! { out =>
                mod #{&item.ident} {}
                #[allow(non_upper_case_globals)]
                const #{app_raw_fn_ident(name)}: #{item.signature()} = const {
                    #{&item}
                    #{&item.ident}
                };
            };
            f(out, item)
        }
    })
}

fn wrap(
    attr: TokenStream,
    item: TokenStream,
    f: impl FnOnce(&mut TokenStream, &mut &[TokenTree], &mut &[TokenTree]) -> Result<(), Error>,
) -> TokenStream {
    let mut ts = TokenStream::new();
    match f(&mut ts, input!(attr), input!(item)) {
        Ok(()) => ts,
        Err(err) => err.into_token_stream(),
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
    app_fn("main", attr, item, |out, f| {
        let app_main = app_raw_fn_ident("main");

        let simple_return = if let Some(rtype) = &f.return_type {
            rtype.is_ident_no_gen("bool") || rtype.is_ident_no_gen("c_int")
        } else {
            true
        };

        if simple_return {
            if cfg!(feature = "std") {
                miniquote_to! { out =>
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
                }
                Ok(())
            } else {
                // no_std main
                miniquote_to! { out =>
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
                }
                Ok(())
            }
        } else {
            if !cfg!(feature = "std") {
                return Err(Error::new(
                    Some(f.ident.span()),
                    "main return types other than `()`, `bool` or `c_int` require the `std` feature",
                ));
            }
            let rtype = &f.return_type.unwrap();
            miniquote_to! { out =>
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
            }
            Ok(())
        }
    })
}

#[proc_macro_attribute]
pub fn app_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    wrap(attr, item, |out, attr, item| {
        let impl_block = ImplBlock::parse_all(item)?;
        let state_t = impl_block.ty.clone();
        let mut has_init = false;
        let mut has_iterate = false;
        let mut has_event = false;
        let mut has_quit = false;

        for item in impl_block.items.iter() {
            if let Item::Function(f) = item {
                let attr = Ident::new(
                    match f.ident.to_string().as_str() {
                        "app_init" => {
                            if has_init {
                                return Err(Error::new(
                                    Some(f.ident.span()),
                                    "`app_init` already defined",
                                ));
                            }
                            has_init = true;
                            "app_init"
                        }
                        "app_iterate" => {
                            if has_iterate {
                                return Err(Error::new(
                                    Some(f.ident.span()),
                                    "`app_iterate` already defined",
                                ));
                            }
                            has_iterate = true;
                            "app_iterate"
                        }
                        "app_event" => {
                            if has_event {
                                return Err(Error::new(
                                    Some(f.ident.span()),
                                    "`app_event` already defined",
                                ));
                            }
                            has_event = true;
                            "app_event"
                        }
                        "app_quit" => {
                            if has_quit {
                                return Err(Error::new(
                                    Some(f.ident.span()),
                                    "`app_quit` already defined",
                                ));
                            }
                            has_quit = true;
                            "app_quit"
                        }
                        _ => continue,
                    },
                    Span::call_site(),
                );
                let mut wrapper = f.clone();
                wrapper.attrs = Vec::new();
                wrapper.vis = Visibility::default();
                wrapper.abi = None;
                wrapper.return_type = wrapper.return_type.map(|t| t.replace_self(state_t.clone()));
                for param in wrapper.params.iter_mut() {
                    if param.ident.to_string() == "self" {
                        param.ident = Ident::new("__sdl3_main_self", Span::mixed_site());
                    }
                    param.ty = param.ty.replace_self(state_t.clone());
                }
                let args = wrapper.params.to_args();
                wrapper.body = TokenTree::Group(Group::new(
                    Delimiter::Brace,
                    miniquote!(#{&state_t}::#{&f.ident} #args),
                ));
                miniquote_to!(out => #[#{sdl3_main_path()}::#attr] #wrapper)
            }
        }
        if !has_init {
            return Err(Error::new(None, "missing `app_init`"));
        }
        if !has_iterate {
            return Err(Error::new(None, "missing `app_iterate`"));
        }
        if !has_event {
            return Err(Error::new(None, "missing `app_event`"));
        }
        if !has_quit {
            let f = Function::new(Ident::new("app_quit", Span::mixed_site()));
            miniquote_to!(out => #[#{sdl3_main_path()}::app_quit] #f);
        }
        miniquote_to!(out => #attr #impl_block);
        Ok(())
    })
}

#[proc_macro_attribute]
pub fn app_init(attr: TokenStream, item: TokenStream) -> TokenStream {
    app_fn("app_init", attr, item, |out, f| {
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

        miniquote_to! { out =>
            #[allow(non_camel_case_types)]
            type #state_t = #state;

            #[#{sdl3_main_path()}::main]
            unsafe fn __sdl3_main_callbacks(argc: ::core::ffi::c_int, argv: *mut *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
                use ::core::ffi::{c_char, c_int, c_void};
                use #{sdl3_sys_path()}::{init::SDL_AppResult, main::SDL_EnterAppMainCallbacks};
                use #{sdl3_main_path()}::{app::AppInit, MainThreadToken};

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
        }
        Ok(())
    })
}

#[proc_macro_attribute]
pub fn app_iterate(attr: TokenStream, item: TokenStream) -> TokenStream {
    let name = "app_iterate";
    app_fn(name, attr, item, |out, f| {
        let state_ac = match f.params.len() {
            1 => f.params[0].ty.classify()? as u8,
            _ => 0,
        };
        miniquote_to! { out =>
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
        }
        Ok(())
    })
}

#[proc_macro_attribute]
pub fn app_event(attr: TokenStream, item: TokenStream) -> TokenStream {
    let name = "app_event";
    app_fn(name, attr, item, |out, f| {
        let (state_ac, event_ac) = match f.params.len() {
            1 => (0, f.params[0].ty.classify()? as u8),
            2 => (
                f.params[0].ty.classify()? as u8,
                f.params[1].ty.classify()? as u8,
            ),
            _ => (0, 0),
        };
        miniquote_to! { out =>
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
        }
        Ok(())
    })
}

#[proc_macro_attribute]
pub fn app_quit(attr: TokenStream, item: TokenStream) -> TokenStream {
    let name = "app_quit";
    app_fn(name, attr, item, |out, f| {
        let state_ac = match f.params.len() {
            1 | 2 => f.params[0].ty.classify()? as u8,
            _ => 0,
        };
        miniquote_to! { out =>
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
        }
        Ok(())
    })
}
