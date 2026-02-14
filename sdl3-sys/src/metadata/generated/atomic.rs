//! Metadata for items in the `crate::atomic` module

use super::*;

pub const METADATA_SDL_SpinLock: Group = Group {
    module: "atomic",
    kind: GroupKind::Lock,
    name: "SDL_SpinLock",
    short_name: "SpinLock",
    doc: Some(
        "An atomic spinlock.\n\nThe atomic locks are efficient spinlocks using CPU instructions, but are\nvulnerable to starvation and can spin forever if a thread holding a lock\nhas been terminated. For this reason you should minimize the code executed\ninside an atomic lock and never do expensive things like API or system\ncalls while holding them.\n\nThey are also vulnerable to starvation if the thread holding the lock is\nlower priority than other threads and doesn't get scheduled. In general you\nshould use mutexes instead, since they have better performance and\ncontention behavior.\n\nThe atomic locks are not safe to lock recursively.\n\nPorting Note: The spin lock functions and type are required and can not be\nemulated because they are used in the atomic emulation code.\n",
    ),
    available_since: None,
    values: &[],
};
pub const METADATA_SDL_AtomicInt: Struct = Struct {
    module: "atomic",
    kind: StructKind::Struct,
    name: "SDL_AtomicInt",
    doc: Some(
        "A type representing an atomic integer value.\n\nThis can be used to manage a value that is synchronized across multiple\nCPUs without a race condition; when an app sets a value with\n[`SDL_SetAtomicInt`] all other threads, regardless of the CPU it is running on,\nwill see that value when retrieved with [`SDL_GetAtomicInt`], regardless of CPU\ncaches, etc.\n\nThis is also useful for atomic compare-and-swap operations: a thread can\nchange the value as long as its current value matches expectations. When\ndone in a loop, one can guarantee data consistency across threads without a\nlock (but the usual warnings apply: if you don't know what you're doing, or\nyou don't do it carefully, you can confidently cause any number of\ndisasters with this, so in most cases, you _should_ use a mutex instead of\nthis!).\n\nThis is a struct so people don't accidentally use numeric operations on it\ndirectly. You have to use SDL atomic functions.\n\n## Availability\nThis struct is available since SDL 3.2.0.\n\n## See also\n- [`SDL_CompareAndSwapAtomicInt`]\n- [`SDL_GetAtomicInt`]\n- [`SDL_SetAtomicInt`]\n- [`SDL_AddAtomicInt`]\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    fields: &[Field {
        name: "value",
        doc: None,
        available_since: None,
        ty: "::core::ffi::c_int",
    }],
};
pub const METADATA_SDL_AtomicU32: Struct = Struct {
    module: "atomic",
    kind: StructKind::Struct,
    name: "SDL_AtomicU32",
    doc: Some(
        "A type representing an atomic unsigned 32-bit value.\n\nThis can be used to manage a value that is synchronized across multiple\nCPUs without a race condition; when an app sets a value with\n[`SDL_SetAtomicU32`] all other threads, regardless of the CPU it is running on,\nwill see that value when retrieved with [`SDL_GetAtomicU32`], regardless of CPU\ncaches, etc.\n\nThis is also useful for atomic compare-and-swap operations: a thread can\nchange the value as long as its current value matches expectations. When\ndone in a loop, one can guarantee data consistency across threads without a\nlock (but the usual warnings apply: if you don't know what you're doing, or\nyou don't do it carefully, you can confidently cause any number of\ndisasters with this, so in most cases, you _should_ use a mutex instead of\nthis!).\n\nThis is a struct so people don't accidentally use numeric operations on it\ndirectly. You have to use SDL atomic functions.\n\n## Availability\nThis struct is available since SDL 3.2.0.\n\n## See also\n- [`SDL_CompareAndSwapAtomicU32`]\n- [`SDL_GetAtomicU32`]\n- [`SDL_SetAtomicU32`]\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    fields: &[Field {
        name: "value",
        doc: None,
        available_since: None,
        ty: "Uint32",
    }],
};
