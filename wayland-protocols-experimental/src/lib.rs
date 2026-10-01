//! This crate provides Wayland object definitions for experimental protocol extensions.
//!
//! This crate provides bindings for protocols that are officially evaluated,
//! but not recommended for use outside of testing.
//!
//! These bindings are built on top of the crates wayland-client and wayland-server.
//!
//! Each protocol module contains a `client` and a `server` submodules, for each side of the
//! protocol. The creation of these modules (and the dependency on the associated crate) is
//! controlled by the two cargo features `client` and `server`.

#![warn(missing_docs)]
#![forbid(improper_ctypes, unsafe_op_in_unsafe_fn)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(rustfmt, rustfmt_skip)]

#[macro_use]
mod protocol_macro;

pub mod session_management {
    //! The xx_session_manager protocol declares interfaces necessary to
    //! allow clients to restore toplevel state from previous executions.

    #[allow(missing_docs)]
    pub mod v1 {
        wayland_protocol!(
            "./protocols/xx-session-management/xx-session-management-v1.xml",
            [wayland_protocols::xdg::shell]
        );
    }
}

/// Experimental text-input counterpart used for xx-input-method enum types.
///
/// Application text input remains [`wayland_protocols::wp::text_input`]; this
/// module exists so xx-input-method can reference `xx_text_input_v3` enums and
/// so compositors can optionally speak xx-text-input.
pub mod text_input {
    #[allow(missing_docs)]
    pub mod v3 {
        wayland_protocol!(
            "./protocols/xx-text-input/xx-text-input-v3.xml",
            []
        );
    }
}

/// Experimental input-method protocol (`xx_input_method_*`).
pub mod input_method {
    //! This protocol allows applications to act as input methods for compositors.
    #[allow(missing_docs)]
    pub mod v1 {
        wayland_protocol!(
            "./protocols/xx-input-method/xx-input-method-v2.xml",
            [crate::text_input::v3]
        );
    }
}

/// Experimental keyboard filter protocol (`xx_keyboard_filter_*`).
pub mod keyboard_filter {
    //! This protocol allows applications to intercept and filter keyboard events.
    #[allow(missing_docs)]
    pub mod v1 {
        wayland_protocol!(
            "./protocols/xx-keyboard-filter/xx-keyboard-filter-v1.xml",
            [crate::input_method::v1]
        );
    }
}
