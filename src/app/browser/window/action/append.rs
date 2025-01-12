use gtk::{
    gio::SimpleAction,
    glib::{uuid_string_random, Variant},
    prelude::{ActionExt, ToVariant},
};

/// Append options
pub enum Position {
    After,
    End,
    Number(i32),
}

// C-compatible Position values
const POSITION_AFTER: i32 = -1;
const POSITION_END: i32 = -2;

// Implement conversion trait
impl ToVariant for Position {
    fn to_variant(&self) -> Variant {
        match self {
            Position::After => &POSITION_AFTER,
            Position::End => &POSITION_END,
            Position::Number(value) => value,
        }
        .to_variant()
    }
}

// Action state defaults
const DEFAULT_POSITION: Position = Position::End;
const DEFAULT_REQUEST: String = String::new();
const DEFAULT_IS_PINNED: bool = false;
const DEFAULT_IS_SELECTED: bool = true;
const DEFAULT_IS_ATTENTION: bool = false;
const DEFAULT_IS_LOAD: bool = false;

/// [SimpleAction](https://docs.gtk.org/gio/class.SimpleAction.html) wrapper for `Append` action of `Window` group
pub struct Append {
    pub simple_action: SimpleAction,
}

impl Default for Append {
    fn default() -> Self {
        Self::new()
    }
}

impl Append {
    // Constructors

    /// Create new `Self` with default action state preset
    pub fn new() -> Self {
        Self {
            simple_action: SimpleAction::new_stateful(
                &uuid_string_random(),
                None,
                &(
                    DEFAULT_POSITION,
                    DEFAULT_REQUEST,
                    DEFAULT_IS_PINNED,
                    DEFAULT_IS_SELECTED,
                    DEFAULT_IS_ATTENTION,
                    DEFAULT_IS_LOAD,
                )
                    .to_variant(),
            ),
        }
    }

    // Actions

    /// Emit [activate](https://docs.gtk.org/gio/signal.SimpleAction.activate.html) signal with default state
    /// * this action reset previous state for action after activation
    pub fn activate_default_once(&self) {
        // Save current state in memory
        let (position, request, is_pinned, is_selected, is_attention, is_load) =
            state(&self.simple_action);

        // Set default state
        self.change_state(
            DEFAULT_POSITION,
            Some(DEFAULT_REQUEST),
            DEFAULT_IS_PINNED,
            DEFAULT_IS_SELECTED,
            DEFAULT_IS_ATTENTION,
            DEFAULT_IS_LOAD,
        );

        // Activate action
        self.simple_action.activate(None);

        // Return previous state
        self.change_state(
            position,
            request,
            is_pinned,
            is_selected,
            is_attention,
            is_load,
        );
    }

    /// Emit [activate](https://docs.gtk.org/gio/signal.SimpleAction.activate.html) signal
    /// * this action reset previous state for action after activation
    pub fn activate_stateful_once(
        &self,
        position: Position,
        request: Option<String>,
        is_pinned: bool,
        is_selected: bool,
        is_attention: bool,
        is_load: bool,
    ) {
        // Save current state in memory
        let (_position, _request, _is_pinned, _is_selected, _is_attention, _is_load) =
            state(&self.simple_action);

        // Apply requested state
        self.change_state(
            position,
            request,
            is_pinned,
            is_selected,
            is_attention,
            is_load,
        );

        // Activate action
        self.simple_action.activate(None);

        // Return previous state
        self.change_state(
            _position,
            _request,
            _is_pinned,
            _is_selected,
            _is_attention,
            _is_load,
        );
    }

    /// Emit state change for action
    pub fn change_state(
        &self,
        position: Position,
        request: Option<String>,
        is_pinned: bool,
        is_selected: bool,
        is_attention: bool,
        is_load: bool,
    ) {
        self.simple_action.change_state(
            &(
                // Convert Option to C-based variant value
                position,
                if let Some(request) = request {
                    request
                } else {
                    DEFAULT_REQUEST
                },
                is_pinned,
                is_selected,
                is_attention,
                is_load,
            )
                .to_variant(),
        );
    }

    // Events

    /// Define callback function for
    /// [SimpleAction::activate](https://docs.gtk.org/gio/signal.SimpleAction.activate.html) signal
    /// * return `position`,`request`,`is_pinned`,`is_selected`,`is_attention`,`is_load` state as tuple
    pub fn connect_activate(
        &self,
        callback: impl Fn(Position, Option<String>, bool, bool, bool, bool) + 'static,
    ) {
        self.simple_action.connect_activate(move |this, _| {
            let (position, request, is_pinned, is_selected, is_attention, is_load) = state(this);
            callback(
                position,
                request,
                is_pinned,
                is_selected,
                is_attention,
                is_load,
            )
        });
    }
}

/// Shared helper to get C-based action state in Optional format
pub fn state(this: &SimpleAction) -> (Position, Option<String>, bool, bool, bool, bool) {
    let (position, request, is_pinned, is_selected, is_attention, is_load) = this
        .state()
        .expect("Expected (`position`,`request`,`is_pinned`,`is_selected`,`is_attention`,`is_load`) state")
        .get::<(i32, String, bool, bool, bool, bool)>()
        .expect("Parameter type does not match (`i32`,`String`,`bool`,`bool`,`bool`,`bool`) tuple");
    (
        // Convert from C-based variant value to Position enum
        match position {
            POSITION_AFTER => Position::After,
            POSITION_END => Position::End,
            value => Position::Number(value),
        },
        if request.is_empty() {
            None
        } else {
            Some(request)
        },
        is_pinned,
        is_selected,
        is_attention,
        is_load,
    )
}
