//! Metadata for items in the `crate::guid` module

use super::*;

pub const METADATA_SDL_GUID: Struct = Struct {
    module: "guid",
    kind: StructKind::Struct,
    name: "SDL_GUID",
    doc: Some(
        "An [`SDL_GUID`] is a 128-bit identifier for an input device that identifies\nthat device across runs of SDL programs on the same platform.\n\nIf the device is detached and then re-attached to a different port, or if\nthe base system is rebooted, the device should still report the same GUID.\n\nGUIDs are as precise as possible but are not guaranteed to distinguish\nphysically distinct but equivalent devices. For example, two game\ncontrollers from the same vendor with the same product ID and revision may\nhave the same GUID.\n\nGUIDs may be platform-dependent (i.e., the same device may report different\nGUIDs on different operating systems).\n\n## Availability\nThis struct is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    fields: &[Field {
        name: "data",
        doc: None,
        available_since: None,
        ty: "[Uint8; 16]",
    }],
};
