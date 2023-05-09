use crossterm::event::{Event, KeyCode};
use futures_util::Stream;
use log::error;

use crate::{ui::{enter_ui, exit_ui}, config::Config};

#[derive(Debug)]
pub struct KEditor {}

impl KEditor {
    pub fn new() -> Self {
        let _config = Config::load(include_str!("../../.config/based.toml")).unwrap();
        Self {}
    }

    async fn handle_terminal_event(&mut self, event: Result<Event, crossterm::ErrorKind>) {
        let event = event.unwrap();
        match event {
            Event::Key(key) =>  {
                if let KeyCode::Char('q') = key.code {
                    exit_ui(); 
                    std::process::exit(0);
                }
            }
            _ => { error!("Unhandled event: {:?}", event); }
        }

    }
    async fn event_loop<S>(&mut self, input_stream: &mut S)
    where
        S: Stream<Item = crossterm::Result<crossterm::event::Event>> + Unpin,
    {
        loop {
            use futures_util::StreamExt;
            tokio::select! {
                Some(event) = input_stream.next() => {
                    self.handle_terminal_event(event).await;
                }
            }
        }
    }

    pub async fn run<S>(&mut self, input_stream: &mut S) -> anyhow::Result<i32>
    where
        S: Stream<Item = crossterm::Result<crossterm::event::Event>> + Unpin,
    {
        let _terminal = enter_ui()?;
        let hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            let _ = exit_ui();
            hook(info)
        }));

        self.event_loop(input_stream).await;

        Ok(0)
    }
}
