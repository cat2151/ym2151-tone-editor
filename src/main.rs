use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::io;

const GRID_WIDTH: usize = 10;
const GRID_HEIGHT: usize = 5;

// Parameter names for each column
const PARAM_NAMES: [&str; GRID_WIDTH] = [
    "DT", "MUL", "TL", "KS", "AR", "D1R", "D1L", "D2R", "RR", "ALG"
];

// Maximum values for each parameter (respecting YM2151 bit ranges)
const PARAM_MAX: [u8; GRID_WIDTH] = [
    7,   // DT: 3 bits (0-7)
    15,  // MUL: 4 bits (0-15)
    99,  // TL: 7 bits (0-127, limited to 99 for display)
    3,   // KS: 2 bits (0-3)
    31,  // AR: 5 bits (0-31)
    31,  // D1R: 5 bits (0-31)
    15,  // D1L: 4 bits (0-15)
    15,  // D2R: 4 bits (0-15)
    15,  // RR: 4 bits (0-15)
    7    // ALG: 3 bits (0-7)
];

// Row names for operators
const ROW_NAMES: [&str; GRID_HEIGHT] = [
    "OP1", "OP2", "OP3", "OP4", "CH "
];

struct App {
    values: [[u8; GRID_WIDTH]; GRID_HEIGHT],
    cursor_x: usize,
    cursor_y: usize,
}

impl App {
    fn new() -> App {
        // Initialize with a basic FM piano-like tone
        // Based on typical YM2151 patch settings
        let mut values = [[0; GRID_WIDTH]; GRID_HEIGHT];
        
        // Operator 1 (Carrier): DT, MUL, TL, KS, AR, D1R, D1L, D2R, RR, ALG
        values[0] = [0, 1, 20, 0, 31, 10, 5, 5, 7, 4];
        
        // Operator 2 (Modulator): softer attack
        values[1] = [0, 1, 30, 0, 25, 8, 6, 4, 6, 0];
        
        // Operator 3 (Modulator): even softer
        values[2] = [0, 2, 40, 0, 20, 6, 7, 3, 5, 0];
        
        // Operator 4 (Modulator): gentle
        values[3] = [0, 1, 35, 0, 22, 7, 6, 4, 6, 0];
        
        // Channel settings: can be used for feedback, LFO, etc.
        values[4] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 4];
        
        App {
            values,
            cursor_x: 0,
            cursor_y: 0,
        }
    }

    fn move_cursor_left(&mut self) {
        if self.cursor_x > 0 {
            self.cursor_x -= 1;
        }
    }

    fn move_cursor_right(&mut self) {
        if self.cursor_x < GRID_WIDTH - 1 {
            self.cursor_x += 1;
        }
    }

    fn move_cursor_up(&mut self) {
        if self.cursor_y > 0 {
            self.cursor_y -= 1;
        }
    }

    fn move_cursor_down(&mut self) {
        if self.cursor_y < GRID_HEIGHT - 1 {
            self.cursor_y += 1;
        }
    }

    fn increase_value(&mut self) {
        let current = self.values[self.cursor_y][self.cursor_x];
        let max = PARAM_MAX[self.cursor_x];
        if current < max {
            self.values[self.cursor_y][self.cursor_x] = current + 1;
        }
    }

    fn decrease_value(&mut self) {
        let current = self.values[self.cursor_y][self.cursor_x];
        if current > 0 {
            self.values[self.cursor_y][self.cursor_x] = current - 1;
        }
    }
}

fn main() -> Result<(), io::Error> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = App::new();

    // Main loop
    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => app.decrease_value(),
                KeyCode::Char('e') => app.increase_value(),
                KeyCode::Char('h') => app.move_cursor_left(),
                KeyCode::Char('j') => app.move_cursor_down(),
                KeyCode::Char('k') => app.move_cursor_up(),
                KeyCode::Char('l') => app.move_cursor_right(),
                KeyCode::Esc => return Ok(()),
                _ => {}
            }
        }
    }
}

fn ui(f: &mut ratatui::Frame, app: &App) {
    let size = f.area();

    let block = Block::default()
        .title("YM2151 Tone Editor (hjkl:move, q/e:dec/inc, ESC:quit)")
        .borders(Borders::ALL);
    let inner = block.inner(size);
    f.render_widget(block, size);

    // Calculate cell dimensions
    let cell_width = 4; // 2 digits + spacing
    let cell_height = 1;
    let label_offset = 1; // Space for parameter name labels
    let row_label_width = 4; // Width for row labels (e.g., "OP1 ")

    // Draw parameter names (column headers)
    for col in 0..GRID_WIDTH {
        let x = inner.x + row_label_width + (col as u16 * cell_width);
        let y = inner.y;

        let area = Rect {
            x,
            y,
            width: cell_width,
            height: cell_height,
        };

        let param_name = PARAM_NAMES[col];
        let paragraph = Paragraph::new(Span::styled(
            param_name,
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        ));
        f.render_widget(paragraph, area);
    }

    // Draw grid values with row labels
    for row in 0..GRID_HEIGHT {
        // Draw row label (operator name)
        let row_label_area = Rect {
            x: inner.x,
            y: inner.y + label_offset + row as u16,
            width: row_label_width,
            height: cell_height,
        };
        let row_name = ROW_NAMES[row];
        let row_label = Paragraph::new(Span::styled(
            row_name,
            Style::default().fg(Color::Yellow),
        ));
        f.render_widget(row_label, row_label_area);

        // Draw values
        for col in 0..GRID_WIDTH {
            let value = app.values[row][col];
            let x = inner.x + row_label_width + (col as u16 * cell_width);
            let y = inner.y + label_offset + row as u16;

            let area = Rect {
                x,
                y,
                width: cell_width,
                height: cell_height,
            };

            let style = if app.cursor_x == col && app.cursor_y == row {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::White)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            let text = format!("{:2}", value);
            let paragraph = Paragraph::new(Span::styled(text, style));
            f.render_widget(paragraph, area);
        }
    }
}
