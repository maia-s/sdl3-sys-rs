//! Metadata for items in the `crate::locale` module

use super::*;

pub const METADATA_SDL_Locale: Struct = Struct {
    module: "locale",
    kind: StructKind::Struct,
    name: "SDL_Locale",
    doc: Some(
        "A struct to provide locale data.\n\nLocale data is split into a spoken language, like English, and an optional\ncountry, like Canada. The language will be in ISO-639 format (so English\nwould be \"en\"), and the country, if not NULL, will be an ISO-3166 country\ncode (so Canada would be \"CA\").\n\n## Availability\nThis function is available since SDL 3.2.0.\n\n## See also\n- [`SDL_GetPreferredLocales`]\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    fields: &[
        Field {
            name: "language",
            doc: Some("A language name, like \"en\" for English.\n"),
            available_since: None,
            ty: "*const ::core::ffi::c_char",
        },
        Field {
            name: "country",
            doc: Some("A country, like \"US\" for America. Can be NULL.\n"),
            available_since: None,
            ty: "*const ::core::ffi::c_char",
        },
    ],
};
