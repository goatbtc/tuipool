mod cli;

use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Paragraph};
use tui::text::{Span, Spans, Text};
use tui::Terminal;
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::execute;
use std::io::stdout;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Definindo argumentos de exemplo para taxa mínima e tamanho mínimo
    let fee_min = Some(2.0);  // Exemplo: Filtro de taxa mínima (em satoshis/vByte)
    let size_min = Some(500); // Exemplo: Filtro de tamanho mínimo (em bytes)

    // Setup do terminal para TUI
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Executa a lógica da CLI para obter dados do mempool
    let cli_result = cli::run_cli(fee_min, size_min).await;

    // Checagem se houve erro ao executar a CLI
    if let Err(e) = cli_result {
        eprintln!("Erro ao executar a CLI: {}", e);
        disable_raw_mode()?;
        return Err(e.into()); // Return the error in the expected format
    }

    // Exemplo de dados do mempool que serão renderizados
    let mempool_data = "Dados do mempool carregados com sucesso!"; // Substitua por dados reais se necessário

    // Criação de Text para o Paragraph
    let text = Text::from(vec![
        Spans::from(Span::raw(mempool_data)),
        Spans::from(Span::raw("Pressione 'q' para sair.")),
    ]);

    // Loop da TUI
    loop {
        terminal.draw(|f| {
            let size = f.size();

            // Layout com uma única coluna
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(size);

            // Painel com dados do mempool
            let block = Block::default()
                .title("Bitcoin Mempool Status")
                .borders(Borders::ALL);
            let paragraph = Paragraph::new(text.clone()) // Usando clone para evitar o erro de move
                .block(block)
                .style(Style::default().fg(Color::White));

            f.render_widget(paragraph, chunks[0]);
        })?;

        // Captura de eventos do teclado
        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                break; // Sai do loop se o usuário apertar 'q'
            }
        }
    }

    // Retorna ao modo normal do terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), DisableMouseCapture)?;
    terminal.show_cursor()?;

    Ok(())
}

