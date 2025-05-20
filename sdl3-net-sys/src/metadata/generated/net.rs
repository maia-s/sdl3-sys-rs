//! Metadata for items in the `crate::net` module

use super::*;

pub const METADATA_NET_PROP_SERVER_REUSEADDR_BOOLEAN: Property = Property {
    module: "net",
    name: "NET_PROP_SERVER_REUSEADDR_BOOLEAN",
    short_name: "SERVER_REUSEADDR_BOOLEAN",
    value: crate::net::NET_PROP_SERVER_REUSEADDR_BOOLEAN,
    ty: PropertyType::BOOLEAN,
    doc: None,
    available_since: None,
};
pub const METADATA_NET_PROP_DATAGRAM_SOCKET_REUSEADDR_BOOLEAN: Property = Property {
    module: "net",
    name: "NET_PROP_DATAGRAM_SOCKET_REUSEADDR_BOOLEAN",
    short_name: "DATAGRAM_SOCKET_REUSEADDR_BOOLEAN",
    value: crate::net::NET_PROP_DATAGRAM_SOCKET_REUSEADDR_BOOLEAN,
    ty: PropertyType::BOOLEAN,
    doc: None,
    available_since: None,
};
pub const METADATA_NET_PROP_DATAGRAM_SOCKET_ALLOW_BROADCAST_BOOLEAN: Property = Property {
    module: "net",
    name: "NET_PROP_DATAGRAM_SOCKET_ALLOW_BROADCAST_BOOLEAN",
    short_name: "DATAGRAM_SOCKET_ALLOW_BROADCAST_BOOLEAN",
    value: crate::net::NET_PROP_DATAGRAM_SOCKET_ALLOW_BROADCAST_BOOLEAN,
    ty: PropertyType::BOOLEAN,
    doc: None,
    available_since: None,
};
pub const METADATA_NET_Status: Group = Group {
    module: "net",
    kind: GroupKind::Enum,
    name: "NET_Status",
    short_name: "Status",
    doc: Some(
        "A tri-state for asynchronous operations.\n\nLots of tasks in SDL_net are asynchronous, as they can't complete until\ndata passes over a network at some murky future point in time.\n\nThis includes sending data over a stream socket, resolving a hostname,\nconnecting to a remote system, and other tasks.\n\nThe library never blocks on tasks that take time to complete, with the\nexception of functions named \"Wait\", which are intended to do nothing but\nblock until a task completes. Functions that are attempting to do something\nthat might block, or are querying the status of a task in-progress, will\nreturn a [`NET_Status`], so an app can see if a task completed, and its final\noutcome.\n\n## Availability\nThis enum is available since SDL_net 3.0.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 0, 0)),
    values: &[
        GroupValue {
            name: "NET_FAILURE",
            short_name: "FAILURE",
            doc: Some("Async operation complete, result was failure.\n"),
            available_since: None,
        },
        GroupValue {
            name: "NET_WAITING",
            short_name: "WAITING",
            doc: Some("Async operation is still in progress, check again later.\n"),
            available_since: None,
        },
        GroupValue {
            name: "NET_SUCCESS",
            short_name: "SUCCESS",
            doc: Some("Async operation complete, result was success.\n"),
            available_since: None,
        },
    ],
};
pub const METADATA_NET_Datagram: Struct = Struct {
    module: "net",
    kind: StructKind::Struct,
    name: "NET_Datagram",
    doc: Some(
        "The data provided for new incoming packets from [`NET_ReceiveDatagram()`].\n\n## Availability\nThis datatype is available since SDL_net 3.0.0.\n\n## See also\n- [`NET_ReceiveDatagram`]\n- [`NET_DestroyDatagram`]\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 0, 0)),
    fields: &[
        Field {
            name: "addr",
            doc: Some(
                "Sender's address. This is unref'd by [`NET_DestroyDatagram`]. You only need to ref it if you want to keep it.\n",
            ),
            available_since: None,
            ty: "*mut NET_Address",
        },
        Field {
            name: "port",
            doc: Some(
                "Sender's port. These do not have to come from the same port the receiver is bound to. These are in host byte order, don't byteswap them!\n",
            ),
            available_since: None,
            ty: "Uint16",
        },
        Field {
            name: "buf",
            doc: Some("the payload of this datagram.\n"),
            available_since: None,
            ty: "*mut Uint8",
        },
        Field {
            name: "buflen",
            doc: Some("the number of bytes available at `buf`.\n"),
            available_since: None,
            ty: "::core::ffi::c_int",
        },
    ],
};
