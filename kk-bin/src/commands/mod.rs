use crate::ui::exit_ui;

pub struct Command {
    name: &'static str,
    fun: fn() -> anyhow::Result<()>,
    doc: &'static str
}

static COMMANDS_LIST: [Command; 1] = [
    Command {name: "quit", fun: exit_ui, doc: "quit the editor"}
];
