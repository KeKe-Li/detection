use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    symbols,
    widgets::{Axis, Block, Borders, Chart, Dataset, Gauge, Paragraph},
    Terminal,
};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use crate::monitor::{DetailedMetrics, MetricsHistory};

pub struct Dashboard {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
    history: MetricsHistory,
}

impl Dashboard {
    pub fn new() -> io::Result<Self> {
        let backend = CrosstermBackend::new(io::stdout());
        let terminal = Terminal::new(backend)?;
        let history = MetricsHistory::new();

        Ok(Self { terminal, history })
    }

    pub fn init(&mut self) -> io::Result<()> {
        enable_raw_mode()?;
        execute!(
            io::stdout(),
            EnterAlternateScreen,
            EnableMouseCapture
        )?;
        self.terminal.clear()?;
        Ok(())
    }

    pub fn update(&mut self, metrics: &DetailedMetrics) -> io::Result<()> {
        self.history.add_metrics(metrics);
        let metrics = metrics.clone();
        
        let cpu_data = self.history.get_cpu_data();
        let mem_data = self.history.get_memory_data();
        
        self.terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([
                    Constraint::Percentage(30),
                    Constraint::Percentage(40),
                    Constraint::Percentage(30),
                ].as_ref())
                .split(f.size());

            let overview_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(33),
                    Constraint::Percentage(33),
                    Constraint::Percentage(34),
                ].as_ref())
                .split(chunks[0]);

            let cpu_gauge = Gauge::default()
                .block(Block::default().title("CPU Usage").borders(Borders::ALL))
                .gauge_style(Style::default().fg(Color::Cyan))
                .percent(metrics.basic.cpu_usage as u16);
            f.render_widget(cpu_gauge, overview_chunks[0]);

            let mem_used_percent = (metrics.basic.used_memory as f64 
                / metrics.basic.total_memory as f64 * 100.0) as u16;
            let memory_gauge = Gauge::default()
                .block(Block::default().title("Memory Usage").borders(Borders::ALL))
                .gauge_style(Style::default().fg(Color::Magenta))
                .percent(mem_used_percent);
            f.render_widget(memory_gauge, overview_chunks[1]);

            let network_info = Paragraph::new(format!(
                "Network:\nRX: {:.2} MB\nTX: {:.2} MB",
                metrics.network.rx_bytes as f64 / 1_000_000.0,
                metrics.network.tx_bytes as f64 / 1_000_000.0,
            ))
            .block(Block::default().title("Network").borders(Borders::ALL));
            f.render_widget(network_info, overview_chunks[2]);

            let chart_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(chunks[1]);

            let cpu_dataset = Dataset::default()
                .name("CPU Usage")
                .marker(symbols::Marker::Braille)
                .style(Style::default().fg(Color::Cyan))
                .data(&cpu_data);

            let cpu_chart = Chart::new(vec![cpu_dataset])
                .block(Block::default().title("CPU History").borders(Borders::ALL))
                .x_axis(Axis::default().bounds([0.0, 100.0]))
                .y_axis(Axis::default().bounds([0.0, 100.0]));
            f.render_widget(cpu_chart, chart_chunks[0]);

            let mem_dataset = Dataset::default()
                .name("Memory Usage")
                .marker(symbols::Marker::Braille)
                .style(Style::default().fg(Color::Magenta))
                .data(&mem_data);

            let mem_chart = Chart::new(vec![mem_dataset])
                .block(Block::default().title("Memory History").borders(Borders::ALL))
                .x_axis(Axis::default().bounds([0.0, 100.0]))
                .y_axis(Axis::default().bounds([0.0, 100.0]));
            f.render_widget(mem_chart, chart_chunks[1]);

            let processes_block = Block::default()
                .title("Processes")
                .borders(Borders::ALL);
            f.render_widget(processes_block, chunks[2]);
        })?;
        
        Ok(())
    }

    pub fn cleanup(&mut self) -> io::Result<()> {
        disable_raw_mode()?;
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        self.terminal.show_cursor()?;
        Ok(())
    }
} 