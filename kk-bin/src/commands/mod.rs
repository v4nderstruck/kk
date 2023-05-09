mod fun;
use fun::*;
use std::{collections::HashMap, sync::Arc};

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

#[derive(Debug, Clone)]
pub struct KCommand {
    pub name: &'static str,
    fun: fn() -> anyhow::Result<()>,
    pub doc: &'static str,
}
pub type ArcKCommand = Arc<KCommand>;

impl KCommand {
    pub fn exec(&self) -> anyhow::Result<()> {
        Ok((self.fun)()?)
    }

    #[rustfmt::skip]
    static_commands!(
        escape, "Escape from current mode",
        nop, "Does Nothing",
        error, "Just an error",
    );
}
