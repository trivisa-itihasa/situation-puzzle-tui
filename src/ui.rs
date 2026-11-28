use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout},
    widgets::{Block, Paragraph},
};

/// Draws the start screen.
pub fn render(frame: &mut Frame) {
    let area = frame.area();
    let block = Block::bordered().title(" situation-puzzle-tui ");
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let [_, middle, _] = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(1),
        Constraint::Fill(1),
    ])
    .areas(inner);

    let hint = Paragraph::new("Press q or Esc to quit").alignment(Alignment::Center);
    frame.render_widget(hint, middle);
}

#[cfg(test)]
mod tests {
    use super::render;
    use ratatui::{Terminal, backend::TestBackend};

    #[test]
    fn renders_start_screen() {
        let mut terminal = Terminal::new(TestBackend::new(80, 24)).unwrap();
        terminal.draw(render).unwrap();
        insta::assert_snapshot!(terminal.backend());
    }
}
