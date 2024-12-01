use ratatui::text::Text;
use tokio::sync::mpsc;

pub async fn spawn_visualizer() -> mpsc::Sender<Text<'static>> {
    let (tx, rx) = mpsc::channel::<Text>(100);
    tokio::spawn(async move { run_app(rx).await });
    tx
}

async fn run_app(mut _rx: mpsc::Receiver<Text<'static>>) {
    let _terminal = ratatui::init();
    ratatui::restore();
}
