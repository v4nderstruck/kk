mod ui;
mod editor;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    
    let mut editor = editor::KEditor::new();
    let return_code = editor.run(&mut crossterm::event::EventStream::new()).await?;
    std::process::exit(return_code)
}
