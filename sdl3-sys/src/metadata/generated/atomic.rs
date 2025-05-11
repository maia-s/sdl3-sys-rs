//! Metadata for items in the `crate::atomic` module

use super::*;

pub const METADATA_SDL_SpinLock: Group = Group {
    module: "atomic",
    kind: GroupKind::Lock,
    name: "SDL_SpinLock",
    short_name: "SpinLock",
    doc: Some("An atomic spinlock.\n\nThe atomic locks are efficient spinlocks using CPU instructions, but are\nvulnerable to starvation and can spin forever if a thread holding a lock\nhas been terminated. For this reason you should minimize the code executed\ninside an atomic lock and never do expensive things like API or system\ncalls while holding them.\n\nThey are also vulnerable to starvation if the thread holding the lock is\nlower priority than other threads and doesn't get scheduled. In general you\nshould use mutexes instead, since they have better performance and\ncontention behavior.\n\nThe atomic locks are not safe to lock recursively.\n\nPorting Note: The spin lock functions and type are required and can not be\nemulated because they are used in the atomic emulation code.\n"),
    available_since: None,
    values: &[
    ],
};
