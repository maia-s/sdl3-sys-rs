use core::{
    ffi::CStr,
    hint::spin_loop,
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
};
use libtest_mimic_collect::{libtest_mimic, test};
use sdl3_main::{run_async_on_main_thread, run_sync_on_main_thread, MainThreadToken};
use sdl3_sys::{
    error::SDL_GetError,
    events::SDL_PumpEvents,
    init::{SDL_Init, SDL_Quit, SDL_INIT_VIDEO},
};
use std::{ffi::c_void, thread};

macro_rules! defer {
    ($($tt:tt)*) => {
        let _defer = $crate::Defer(Some(move || {{ $($tt)* };}));
    };
}

struct Defer<F: FnOnce()>(Option<F>);

impl<F: FnOnce()> Drop for Defer<F> {
    fn drop(&mut self) {
        if let Some(f) = self.0.take() {
            f();
        }
    }
}

#[must_use]
fn async_small<F: FnOnce() + Send + 'static>(callback: F) -> bool {
    assert!(
        size_of::<F>() <= size_of::<*mut c_void>() && align_of::<F>() <= align_of::<*mut c_void>()
    );
    run_async_on_main_thread(callback)
}

#[must_use]
fn async_large<F: FnOnce() + Send + 'static>(callback: F) -> bool {
    assert!(
        size_of::<F>() > size_of::<*mut c_void>() || align_of::<F>() > align_of::<*mut c_void>()
    );
    run_async_on_main_thread(callback)
}

#[test]
fn run_sync_on_main_thread_simple_on_main() {
    unsafe { MainThreadToken::init() };
    if !unsafe { SDL_Init(SDL_INIT_VIDEO) } {
        dbg!(unsafe { CStr::from_ptr(SDL_GetError()) });
        panic!();
    }
    defer!(unsafe { SDL_Quit() });
    assert!(MainThreadToken::get().is_some());
    let mut value = 0;
    assert!(run_sync_on_main_thread(|| {
        MainThreadToken::assert();
        value += 0x1234;
    }));
    assert_eq!(value, 0x1234);
}

#[test]
fn run_sync_on_main_thread_drop_on_main() {
    struct Dropper(u32);
    impl Drop for Dropper {
        fn drop(&mut self) {
            DROPPED.store(true, Ordering::Release);
        }
    }
    static DROPPED: AtomicBool = AtomicBool::new(false);
    unsafe { MainThreadToken::init() };
    if !unsafe { SDL_Init(SDL_INIT_VIDEO) } {
        dbg!(unsafe { CStr::from_ptr(SDL_GetError()) });
        panic!();
    }
    defer!(unsafe { SDL_Quit() });
    assert!(MainThreadToken::get().is_some());
    let mut value = 0;
    let dropper = Dropper(0x1234);
    let value_ref = &mut value;
    assert!(!DROPPED.load(Ordering::Acquire));
    assert!(run_sync_on_main_thread(move || {
        MainThreadToken::assert();
        let d = dropper;
        *value_ref += d.0;
    }));
    assert!(DROPPED.load(Ordering::Acquire));
    assert_eq!(value, 0x1234);
}

#[test]
fn run_sync_on_main_thread_simple_on_thread() {
    unsafe { MainThreadToken::init() };
    if !unsafe { SDL_Init(SDL_INIT_VIDEO) } {
        dbg!(unsafe { CStr::from_ptr(SDL_GetError()) });
        panic!();
    }
    defer!(unsafe { SDL_Quit() });
    let t = thread::spawn(|| {
        assert!(MainThreadToken::get().is_none());
        let mut value = 0;
        assert!(run_sync_on_main_thread(|| {
            MainThreadToken::assert();
            value += 0x1234;
        }));
        assert_eq!(value, 0x1234);
    });
    MainThreadToken::assert();
    while !t.is_finished() {
        unsafe { SDL_PumpEvents() };
    }
}

#[test]
fn run_sync_on_main_thread_drop_on_thread() {
    struct Dropper(u32);
    impl Drop for Dropper {
        fn drop(&mut self) {
            DROPPED.store(true, Ordering::Release);
        }
    }
    static DROPPED: AtomicBool = AtomicBool::new(false);
    unsafe { MainThreadToken::init() };
    if !unsafe { SDL_Init(SDL_INIT_VIDEO) } {
        dbg!(unsafe { CStr::from_ptr(SDL_GetError()) });
        panic!();
    }
    defer!(unsafe { SDL_Quit() });
    let t = thread::spawn(|| {
        assert!(MainThreadToken::get().is_none());
        let mut value = 0;
        let value_ref = &mut value;
        let dropper = Dropper(0x1234);
        assert!(!DROPPED.load(Ordering::Acquire));
        assert!(run_sync_on_main_thread(move || {
            MainThreadToken::assert();
            let d = dropper;
            *value_ref += d.0;
        }));
        assert!(DROPPED.load(Ordering::Acquire));
        assert_eq!(value, 0x1234);
    });
    MainThreadToken::assert();
    while !t.is_finished() {
        unsafe { SDL_PumpEvents() };
    }
}

#[test]
fn run_async_on_main_thread_simple_small_on_main() {
    static COMPLETE: AtomicBool = AtomicBool::new(false);
    static VALUE: AtomicUsize = AtomicUsize::new(0);
    unsafe { MainThreadToken::init() };
    if !unsafe { SDL_Init(SDL_INIT_VIDEO) } {
        dbg!(unsafe { CStr::from_ptr(SDL_GetError()) });
        panic!();
    }
    defer!(unsafe { SDL_Quit() });
    assert!(async_small(|| {
        MainThreadToken::assert();
        VALUE.fetch_add(0x1234, Ordering::AcqRel);
        COMPLETE.store(true, Ordering::Release)
    }));
    while !COMPLETE.load(Ordering::Acquire) {
        unsafe { SDL_PumpEvents() };
    }
    assert_eq!(VALUE.load(Ordering::Acquire), 0x1234);
}

#[test]
fn run_async_on_main_thread_simple_small_on_thread() {
    static COMPLETE: AtomicBool = AtomicBool::new(false);
    static VALUE: AtomicUsize = AtomicUsize::new(0);
    unsafe { MainThreadToken::init() };
    if !unsafe { SDL_Init(SDL_INIT_VIDEO) } {
        dbg!(unsafe { CStr::from_ptr(SDL_GetError()) });
        panic!();
    }
    defer!(unsafe { SDL_Quit() });
    let add = 0x1234;
    thread::spawn(move || {
        assert!(MainThreadToken::get().is_none());
        assert!(async_small(move || {
            MainThreadToken::assert();
            VALUE.fetch_add(add, Ordering::AcqRel);
            COMPLETE.store(true, Ordering::Release)
        }));
    });
    while !COMPLETE.load(Ordering::Acquire) {
        unsafe { SDL_PumpEvents() };
    }
    assert_eq!(VALUE.load(Ordering::Acquire), 0x1234);
}

#[test]
fn run_async_on_main_thread_simple_large_on_thread() {
    static COMPLETE: AtomicBool = AtomicBool::new(false);
    static VALUE: AtomicUsize = AtomicUsize::new(0);
    unsafe { MainThreadToken::init() };
    if !unsafe { SDL_Init(SDL_INIT_VIDEO) } {
        dbg!(unsafe { CStr::from_ptr(SDL_GetError()) });
        panic!();
    }
    defer!(unsafe { SDL_Quit() });
    let add = 0x1234_u128;
    thread::spawn(move || {
        assert!(MainThreadToken::get().is_none());
        assert!(async_large(move || {
            MainThreadToken::assert();
            VALUE.fetch_add(add as _, Ordering::AcqRel);
            COMPLETE.store(true, Ordering::Release)
        }));
    });
    while !COMPLETE.load(Ordering::Acquire) {
        unsafe { SDL_PumpEvents() };
    }
    assert_eq!(VALUE.load(Ordering::Acquire), 0x1234);
}

#[test]
fn run_async_on_main_thread_drop_small_on_thread() {
    struct Dropper();
    impl Drop for Dropper {
        fn drop(&mut self) {
            DROPPED.store(true, Ordering::Release);
        }
    }
    static DROPPED: AtomicBool = AtomicBool::new(false);
    static COMPLETE: AtomicBool = AtomicBool::new(false);
    static VALUE: AtomicUsize = AtomicUsize::new(0);
    unsafe { MainThreadToken::init() };
    if !unsafe { SDL_Init(SDL_INIT_VIDEO) } {
        dbg!(unsafe { CStr::from_ptr(SDL_GetError()) });
        panic!();
    }
    defer!(unsafe { SDL_Quit() });
    let add = 0x1234;
    thread::spawn(move || {
        let dropper = Dropper();
        assert!(MainThreadToken::get().is_none());
        assert!(!DROPPED.load(Ordering::Acquire));
        assert!(async_small(move || {
            MainThreadToken::assert();
            let _d = dropper;
            VALUE.fetch_add(add, Ordering::AcqRel);
            COMPLETE.store(true, Ordering::Release)
        }));
    });
    while !COMPLETE.load(Ordering::Acquire) {
        unsafe { SDL_PumpEvents() };
    }
    assert!(DROPPED.load(Ordering::Acquire));
    assert_eq!(VALUE.load(Ordering::Acquire), 0x1234);
}

#[test]
fn run_async_on_main_thread_drop_large_on_thread() {
    struct Dropper();
    impl Drop for Dropper {
        fn drop(&mut self) {
            DROPPED.store(true, Ordering::Release);
        }
    }
    static DROPPED: AtomicBool = AtomicBool::new(false);
    static COMPLETE: AtomicBool = AtomicBool::new(false);
    static VALUE: AtomicUsize = AtomicUsize::new(0);
    unsafe { MainThreadToken::init() };
    if !unsafe { SDL_Init(SDL_INIT_VIDEO) } {
        dbg!(unsafe { CStr::from_ptr(SDL_GetError()) });
        panic!();
    }
    defer!(unsafe { SDL_Quit() });
    let add = 0x1234_u128;
    thread::spawn(move || {
        let dropper = Dropper();
        assert!(MainThreadToken::get().is_none());
        assert!(!DROPPED.load(Ordering::Acquire));
        assert!(async_large(move || {
            MainThreadToken::assert();
            let _d = dropper;
            VALUE.fetch_add(add as _, Ordering::AcqRel);
            COMPLETE.store(true, Ordering::Release)
        }));
    });
    while !COMPLETE.load(Ordering::Acquire) {
        unsafe { SDL_PumpEvents() };
    }
    assert!(DROPPED.load(Ordering::Acquire));
    assert_eq!(VALUE.load(Ordering::Acquire), 0x1234);
}

fn main() {
    let tests = libtest_mimic_collect::TestCollection::collect_tests();
    let mut args = libtest_mimic::Arguments::from_args();
    args.test_threads = Some(1);
    libtest_mimic::run(&args, tests).exit();
}
