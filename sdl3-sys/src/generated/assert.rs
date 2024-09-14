//! A helpful assertion macro!
//!
//! SDL assertions operate like your usual `assert` macro, but with some added
//! features:
//!
//! - It uses a trick with the `sizeof` operator, so disabled assertions
//!   vaporize out of the compiled code, but variables only referenced in the
//!   assertion won't trigger compiler warnings about being unused.
//! - It is safe to use with a dangling-else: `if (x) SDL_assert(y); else
//!   do_something();`
//! - It works the same everywhere, instead of counting on various platforms'
//!   compiler and C runtime to behave.
//! - It provides multiple levels of assertion (SDL_assert, SDL_assert_release,
//!   SDL_assert_paranoid) instead of a single all-or-nothing option.
//! - It offers a variety of responses when an assertion fails (retry, trigger
//!   the debugger, abort the program, ignore the failure once, ignore it for
//!   the rest of the program's run).
//! - It tries to show the user a dialog by default, if possible, but the app
//!   can provide a callback to handle assertion failures however they like.
//! - It lets failed assertions be retried. Perhaps you had a network failure
//!   and just want to retry the test after plugging your network cable back
//!   in? You can.
//! - It lets the user ignore an assertion failure, if there's a harmless
//!   problem that one can continue past.
//! - It lets the user mark an assertion as ignored for the rest of the
//!   program's run; if there's a harmless problem that keeps popping up.
//! - It provides statistics and data on all failed assertions to the app.
//! - It allows the default assertion handler to be controlled with environment
//!   variables, in case an automated script needs to control it.
//!
//! To use it: do a debug build and just sprinkle around tests to check your
//! code!

use super::stdinc::*;

#[cfg(doc)]
emit! {}

#[cfg(not(doc))]
emit! {
    #[cfg(any(all(not(not(debug_assertions)), any(/* always disabled: __GNUC__ */)), debug_assertions, debug_assertions))]
    emit! {
        pub const SDL_ASSERT_LEVEL: ::core::primitive::i32 = 2;

    }

    #[cfg(not(any(all(not(not(debug_assertions)), any(/* always disabled: __GNUC__ */)), debug_assertions, debug_assertions)))]
    emit! {
        pub const SDL_ASSERT_LEVEL: ::core::primitive::i32 = 1;

    }

}

#[cfg(doc)]
emit! {}

#[cfg(not(doc))]
emit! {
    #[cfg(all(windows, target_env = "msvc"))]
    emit! {
        extern "cdecl" {
            pub fn __debugbreak();
        }

    }

    #[cfg(not(all(windows, target_env = "msvc")))]
    emit! {
    }

}

#[cfg(all(windows, target_env = "msvc"))]
emit! {
    pub const SDL_NULL_WHILE_LOOP_CONDITION: ::core::primitive::i32 = 0;

}

#[cfg(not(all(windows, target_env = "msvc")))]
emit! {
    pub const SDL_NULL_WHILE_LOOP_CONDITION: ::core::primitive::i32 = 0;

}

/// Possible outcomes from a triggered assertion.
///
/// When an enabled assertion triggers, it may call the assertion handler
/// (possibly one provided by the app via SDL_SetAssertionHandler), which will
/// return one of these values, possibly after asking the user.
///
/// Then SDL will respond based on this outcome (loop around to retry the
/// condition, try to break in a debugger, kill the program, or ignore the
/// problem).
///
/// \since This enum is available since SDL 3.0.0.
///
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_ASSERTION_RETRY`], [`SDL_ASSERTION_BREAK`], [`SDL_ASSERTION_ABORT`], [`SDL_ASSERTION_IGNORE`], [`SDL_ASSERTION_ALWAYS_IGNORE`]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_AssertState(pub ::core::ffi::c_int);
impl SDL_AssertState {
    /// Retry the assert immediately.
    pub const RETRY: Self = Self(0);
    /// Make the debugger trigger a breakpoint.
    pub const BREAK: Self = Self(1);
    /// Terminate the program.
    pub const ABORT: Self = Self(2);
    /// Ignore the assert.
    pub const IGNORE: Self = Self(3);
    /// Ignore the assert from now on.
    pub const ALWAYS_IGNORE: Self = Self(4);
}
/// Retry the assert immediately.
pub const SDL_ASSERTION_RETRY: SDL_AssertState = SDL_AssertState::RETRY;
/// Make the debugger trigger a breakpoint.
pub const SDL_ASSERTION_BREAK: SDL_AssertState = SDL_AssertState::BREAK;
/// Terminate the program.
pub const SDL_ASSERTION_ABORT: SDL_AssertState = SDL_AssertState::ABORT;
/// Ignore the assert.
pub const SDL_ASSERTION_IGNORE: SDL_AssertState = SDL_AssertState::IGNORE;
/// Ignore the assert from now on.
pub const SDL_ASSERTION_ALWAYS_IGNORE: SDL_AssertState = SDL_AssertState::ALWAYS_IGNORE;

/// Information about an assertion failure.
///
/// This structure is filled in with information about a triggered assertion,
/// used by the assertion handler, then added to the assertion report. This is
/// returned as a linked list from SDL_GetAssertionReport().
///
/// \since This struct is available since SDL 3.0.0.
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_AssertData {
    /// SDL_TRUE if app should always continue when assertion is triggered.
    pub always_ignore: SDL_bool,
    /// Number of times this assertion has been triggered.
    pub trigger_count: ::core::ffi::c_uint,
    /// A string of this assert's test code.
    pub condition: *const ::core::ffi::c_char,
    /// The source file where this assert lives.
    pub filename: *const ::core::ffi::c_char,
    /// The line in `filename` where this assert lives.
    pub linenum: ::core::ffi::c_int,
    /// The name of the function where this assert lives.
    pub function: *const ::core::ffi::c_char,
    /// next item in the linked list.
    pub next: *const SDL_AssertData,
}

extern "C" {
    /// Never call this directly.
    ///
    /// Use the SDL_assert* macros instead.
    ///
    /// \param data assert data structure.
    /// \param func function name.
    /// \param file file name.
    /// \param line line number.
    /// \returns assert state.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_ReportAssertion(
        data: *mut SDL_AssertData,
        func: *const ::core::ffi::c_char,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
    ) -> SDL_AssertState;
}

#[cfg(doc)]
emit! {}

#[cfg(not(doc))]
emit! {
    #[cfg(feature = "assert-level-disabled")]
    emit! {
    }

    #[cfg(not(feature = "assert-level-disabled"))]
    emit! {
        #[cfg(feature = "assert-level-release")]
        emit! {
        }

        #[cfg(not(feature = "assert-level-release"))]
        emit! {
            #[cfg(feature = "assert-level-debug")]
            emit! {
            }

            #[cfg(not(feature = "assert-level-debug"))]
            emit! {
                #[cfg(feature = "assert-level-paranoid")]
                emit! {
                }

                #[cfg(not(feature = "assert-level-paranoid"))]
                emit! {
                    ::core::compile_error!("Unknown assertion level.");
                }

            }

        }

    }

}

/// A callback that fires when an SDL assertion fails.
///
/// \param data a pointer to the SDL_AssertData structure corresponding to the
///             current assertion.
/// \param userdata what was passed as `userdata` to SDL_SetAssertionHandler().
/// \returns an SDL_AssertState value indicating how to handle the failure.
///
/// \since This datatype is available since SDL 3.0.0.
pub type SDL_AssertionHandler = ::core::option::Option<
    extern "C" fn(
        data: *const SDL_AssertData,
        userdata: *mut ::core::ffi::c_void,
    ) -> SDL_AssertState,
>;

extern "C" {
    /// Set an application-defined assertion handler.
    ///
    /// This function allows an application to show its own assertion UI and/or
    /// force the response to an assertion failure. If the application doesn't
    /// provide this, SDL will try to do the right thing, popping up a
    /// system-specific GUI dialog, and probably minimizing any fullscreen windows.
    ///
    /// This callback may fire from any thread, but it runs wrapped in a mutex, so
    /// it will only fire from one thread at a time.
    ///
    /// This callback is NOT reset to SDL's internal handler upon SDL_Quit()!
    ///
    /// \param handler the SDL_AssertionHandler function to call when an assertion
    ///                fails or NULL for the default handler.
    /// \param userdata a pointer that is passed to `handler`.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetAssertionHandler
    pub fn SDL_SetAssertionHandler(
        handler: SDL_AssertionHandler,
        userdata: *mut ::core::ffi::c_void,
    );
}

extern "C" {
    /// Get the default assertion handler.
    ///
    /// This returns the function pointer that is called by default when an
    /// assertion is triggered. This is an internal function provided by SDL, that
    /// is used for assertions when SDL_SetAssertionHandler() hasn't been used to
    /// provide a different function.
    ///
    /// \returns the default SDL_AssertionHandler that is called when an assert
    ///          triggers.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetAssertionHandler
    pub fn SDL_GetDefaultAssertionHandler() -> SDL_AssertionHandler;
}

extern "C" {
    /// Get the current assertion handler.
    ///
    /// This returns the function pointer that is called when an assertion is
    /// triggered. This is either the value last passed to
    /// SDL_SetAssertionHandler(), or if no application-specified function is set,
    /// is equivalent to calling SDL_GetDefaultAssertionHandler().
    ///
    /// The parameter `puserdata` is a pointer to a void*, which will store the
    /// "userdata" pointer that was passed to SDL_SetAssertionHandler(). This value
    /// will always be NULL for the default handler. If you don't care about this
    /// data, it is safe to pass a NULL pointer to this function to ignore it.
    ///
    /// \param puserdata pointer which is filled with the "userdata" pointer that
    ///                  was passed to SDL_SetAssertionHandler().
    /// \returns the SDL_AssertionHandler that is called when an assert triggers.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_SetAssertionHandler
    pub fn SDL_GetAssertionHandler(
        puserdata: *mut *mut ::core::ffi::c_void,
    ) -> SDL_AssertionHandler;
}

extern "C" {
    /// Get a list of all assertion failures.
    ///
    /// This function gets all assertions triggered since the last call to
    /// SDL_ResetAssertionReport(), or the start of the program.
    ///
    /// The proper way to examine this data looks something like this:
    ///
    /// ```c
    /// const SDL_AssertData *item = SDL_GetAssertionReport();
    /// while (item) {
    ///    printf("'%s', %s (%s:%d), triggered %u times, always ignore: %s.\\n",
    ///           item->condition, item->function, item->filename,
    ///           item->linenum, item->trigger_count,
    ///           item->always_ignore ? "yes" : "no");
    ///    item = item->next;
    /// }
    /// ```
    ///
    /// \returns a list of all failed assertions or NULL if the list is empty. This
    ///          memory should not be modified or freed by the application.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_ResetAssertionReport
    pub fn SDL_GetAssertionReport() -> *const SDL_AssertData;
}

extern "C" {
    /// Clear the list of all assertion failures.
    ///
    /// This function will clear the list of all assertions triggered up to that
    /// point. Immediately following this call, SDL_GetAssertionReport will return
    /// no items. In addition, any previously-triggered assertions will be reset to
    /// a trigger_count of zero, and their always_ignore state will be false.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_GetAssertionReport
    pub fn SDL_ResetAssertionReport();
}
