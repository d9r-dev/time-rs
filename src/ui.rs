use crate::app::{App, CurrentScreen, CurrentlyEditing};
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::Direction;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, Cell, Clear, Paragraph, Row, Table, Wrap};

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}

pub fn ui(frame: &mut Frame, app: &mut App) {
    // Create the layout sections.
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled("Time.rs", Style::default().fg(Color::Green)))
        .block(title_block);

    frame.render_widget(title, chunks[0]);

    // render table in chunk[1]

    let rows: Vec<Row> = app
        .timers
        .iter()
        .map(|t| {
            Row::new(vec![
                Cell::from(t.name.clone()),
                Cell::from(t.description.clone()),
                Cell::from(t.formatted_duration().clone()),
            ])
        })
        .collect();

    let selected_row_style = Style::default()
        .add_modifier(Modifier::REVERSED)
        .fg(Color::Red);

    let table = Table::new(
        rows,
        &[
            Constraint::Percentage(30),
            Constraint::Percentage(50),
            Constraint::Percentage(20),
        ],
    )
    .header(
        Row::new(vec![
            Cell::from("Name"),
            Cell::from("Description"),
            Cell::from("Duration"),
        ])
        .style(Style::default().fg(Color::Yellow))
        .bottom_margin(1),
    )
    .row_highlight_style(selected_row_style)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Timers"),
    );

    frame.render_widget(Clear, chunks[1]);
    frame.render_stateful_widget(table, chunks[1], &mut app.state);
    // Footer

    let current_keys_hint = {
        match &app.current_screen {
            CurrentScreen::Main => Span::styled("<Strg + q> Exit", Style::default().fg(Color::Red)),
            CurrentScreen::Exit => Span::styled("<Strg + q> Exit", Style::default().fg(Color::Red)),
            CurrentScreen::Add => Span::styled("<Strg + q> Exit", Style::default().fg(Color::Red)),
        }
    };

    let key_notes_footer =
        Paragraph::new(Line::from(current_keys_hint)).block(Block::default().borders(Borders::ALL));

    frame.render_widget(key_notes_footer, chunks[2]);

    if let CurrentScreen::Exit = app.current_screen {
        frame.render_widget(Clear, frame.area()); //this clears the entire screen and anything already drawn
        let popup_block = Block::default()
            .title("Y/N")
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::DarkGray));

        let exit_text = Text::styled(
            "Would you like to exit? (y/n)",
            Style::default().fg(Color::Red),
        );
        // the `trim: false` will stop the text from being cut off when over the edge of the block
        let exit_paragraph = Paragraph::new(exit_text)
            .block(popup_block)
            .wrap(Wrap { trim: false });

        let area = centered_rect(60, 25, frame.area());
        frame.render_widget(exit_paragraph, area);
    }

    if let CurrentScreen::Add = app.current_screen {
        frame.render_widget(Clear, frame.area()); //this clears the entire screen and anything already drawn
        let popup_block = Block::default()
            .title("Add timer")
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::DarkGray));

        let area = centered_rect(60, 25, frame.area());
        frame.render_widget(popup_block, area);

        let popup_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        let mut name_block = Block::default().title("Name").borders(Borders::ALL);
        let mut desc_block = Block::default().title("Description").borders(Borders::ALL);

        let active_style = Style::default().bg(Color::LightYellow).fg(Color::Black);

        match app.currently_editing {
            Some(CurrentlyEditing::Name) => name_block = name_block.style(active_style),
            Some(CurrentlyEditing::Description) => desc_block = desc_block.style(active_style),
            None => {
                name_block = name_block.style(active_style);
            }
        };

        let key_text = Paragraph::new(app.name_input.clone()).block(name_block);
        frame.render_widget(key_text, popup_chunks[0]);

        let value_text = Paragraph::new(app.description_input.clone()).block(desc_block);
        frame.render_widget(value_text, popup_chunks[1]);
    }
}
