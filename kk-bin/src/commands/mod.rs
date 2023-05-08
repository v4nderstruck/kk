mod fun;
use fun::*;
use std::collections::HashMap;

use crate::ui::exit_ui;

/// taken from helix_term::commands
macro_rules! static_commands {
    ( $($name:ident, $doc:literal,)* ) => {
        $(
            #[allow(non_upper_case_globals)]
            pub const $name: Self = Self {
                name: stringify!($name),
                fun: $name,
                doc: $doc
            };
        )*

        pub const STATIC_COMMAND_LIST: &'static [Self] = &[
            $( Self::$name, )*
        ];
    }
}

pub struct Command {
    pub name: &'static str,
    fun: fn() -> anyhow::Result<()>,
    pub doc: &'static str
}

impl Command {
    pub fn exec(&self) -> anyhow::Result<()> {
        Ok((self.fun)()?)
    }


    static_commands!(
        nop, "Does Nothing",
    );
}
