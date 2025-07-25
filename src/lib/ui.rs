use crate::lib::app;
use crate::lib::app::{App, CurrentScreen, CurrentlyEditing};
use crate::lib::throbber::Throbber;
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::Direction;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, Cell, Clear, Paragraph, Row, Table};

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

fn create_rows_with_subheaders(
    timers: &Vec<app::Timer>,
    throbber: &Throbber,
) -> (Vec<Row<'static>>, Vec<bool>) {
    let mut rows = Vec::new();
    let mut selectable_rows = Vec::new();

    if timers.is_empty() {
        return (rows, selectable_rows);
    }

    let mut current_date = timers.first().unwrap().formatted_date();
    rows.push(create_row_for_date(current_date.clone()));
    selectable_rows.push(false);

    for (i, timer) in timers.iter().enumerate() {
        let is_last = i == timers.len() - 1;
        if current_date != timer.formatted_date() {
            current_date = timer.formatted_date();
            rows.push(create_row_for_date(current_date.clone()));
            selectable_rows.push(false);
            rows.push(create_row_for_timer(timer, is_last, throbber));
            selectable_rows.push(true);
        } else {
            rows.push(create_row_for_timer(timer, is_last, throbber));
            selectable_rows.push(true);
        }
    }

    (rows, selectable_rows)
}

fn create_row_for_date(date: String) -> Row<'static> {
    Row::new(vec![
        Cell::from(date),
        Cell::from(""),
        Cell::from(""),
        Cell::from(""),
    ])
    .style(Style::default().add_modifier(Modifier::BOLD))
}

fn create_row_for_timer(timer: &app::Timer, is_last: bool, throbber: &Throbber) -> Row<'static> {
    Row::new(vec![
        Cell::from(timer.name.clone()),
        Cell::from(timer.description.clone()),
        Cell::from(timer.formatted_duration().clone()),
        Cell::from(if is_last {
            Span::from(throbber.get_state_string().to_string() + " ")
        } else {
            Span::from("")
        }),
    ])
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

    let title = Paragraph::new(Text::styled("Time.rs", Style::default())).block(title_block);

    frame.render_widget(title, chunks[0]);

    // render table in chunk[1]

    let (rows, selectable_rows) = create_rows_with_subheaders(&app.timers, &app.throbber);
    app.selectable_rows = selectable_rows;

    let selected_row_style = Style::default().add_modifier(Modifier::REVERSED);

    let table = Table::new(
        rows,
        &[
            Constraint::Percentage(10),
            Constraint::Fill(1),
            Constraint::Length(8),
            Constraint::Length(2),
        ],
    )
    .header(
        Row::new(vec![
            Cell::from("Name"),
            Cell::from("Description"),
            Cell::from("Duration"),
            Cell::from(""),
        ])
        .style(Style::default())
        .bottom_margin(1),
    )
    .row_highlight_style(selected_row_style)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default())
            .title("Timers"),
    );

    frame.render_widget(Clear, chunks[1]);
    frame.render_stateful_widget(table, chunks[1], &mut app.state);
    // Footer

    let current_keys_hint = {
        match &app.current_screen {
            CurrentScreen::Main => Span::styled(
                "<space> Start/Stop timer | <Alt + i> Add timer | <j> Down | <k> Up | <dd> Delete timer | <Esc> Exit",
                Style::default(),
            ),
            CurrentScreen::Exit => Span::styled("<y> Yes | <n> No", Style::default()),
            CurrentScreen::Add | CurrentScreen::Edit => {
                Span::styled("<Tab> Next field | <Enter> Submit", Style::default())
            }
        }
    };

    let key_notes_footer =
        Paragraph::new(Line::from(current_keys_hint)).block(Block::default().borders(Borders::ALL));

    frame.render_widget(key_notes_footer, chunks[2]);

    if let CurrentScreen::Exit = app.current_screen {
        frame.render_widget(Clear, frame.area()); //this clears the entire screen and anything already drawn
        let popup_block = Block::default()
            .title("Exit?")
            .borders(Borders::ALL)
            .style(Style::default());

        let main_area = Rect {
            x: (frame.area().width.saturating_sub(40)) / 2,
            y: (frame.area().height.saturating_sub(10)) / 2,
            width: 40,
            height: 7,
        };

        let help_area = Rect {
            x: (frame.area().width.saturating_sub(40)) / 2,
            y: (frame.area().height.saturating_sub(15)) / 2 + 9,
            width: 40,
            height: 3,
        };

        // Create layout for the popup content
        let popup_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(1), // Question text
                Constraint::Length(1), // Spacing
                Constraint::Length(1), // Buttons
            ])
            .split(main_area);

        // Question text
        let question_text = Paragraph::new("Would you like to exit?")
            .alignment(ratatui::layout::Alignment::Center)
            .style(Style::default());
        frame.render_widget(question_text, popup_chunks[0]);

        // Button layout
        let button_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Min(1),    // Left flexible spacing
                Constraint::Length(4), // Fixed width for Yes button
                Constraint::Length(2), // Fixed gap between buttons
                Constraint::Length(4), // Fixed width for No button
                Constraint::Min(1),    // Right flexible spacing
            ])
            .split(popup_chunks[2]);

        // Yes button
        let yes_style = if app.exit_button_selected {
            Style::default().add_modifier(Modifier::REVERSED)
        } else {
            Style::default()
        };
        let yes_button = Paragraph::new("[Y]es")
            .alignment(ratatui::layout::Alignment::Center)
            .style(yes_style);
        frame.render_widget(yes_button, button_chunks[1]);

        // No button
        let no_style = if !app.exit_button_selected {
            Style::default().add_modifier(Modifier::REVERSED)
        } else {
            Style::default()
        };
        let no_button = Paragraph::new("[N]o")
            .alignment(ratatui::layout::Alignment::Center)
            .style(no_style);
        frame.render_widget(no_button, button_chunks[3]);

        // Render the popup block border
        frame.render_widget(popup_block, main_area);

        // Create help box below main popup
        let help_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default());

        let help_text = Paragraph::new("<Tab> Switch | <Enter> Confirm")
            .alignment(ratatui::layout::Alignment::Center)
            .block(help_block)
            .style(Style::default());

        frame.render_widget(help_text, help_area);
    }

    if let CurrentScreen::Edit | CurrentScreen::Add = app.current_screen {
        frame.render_widget(Clear, frame.area()); //this clears the entire screen and anything already drawn
        let popup_block = Block::default()
            .borders(Borders::NONE)
            .style(Style::default());

        let area = centered_rect(60, 35, frame.area());
        frame.render_widget(popup_block, area);

        // Split the area into main content and help box
        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Percentage(70), Constraint::Min(2)])
            .split(area);

        let popup_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(main_chunks[0]);

        let mut name_block = Block::default().title("Name").borders(Borders::ALL);
        let mut desc_block = Block::default().title("Description").borders(Borders::ALL);

        let active_style = Style::default().add_modifier(Modifier::REVERSED);

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

        let help_block = Block::default()
            .title("Keyboard Actions")
            .borders(Borders::ALL)
            .style(Style::default());

        let help_paragraph =
            Paragraph::new("<Enter> Save | <Tab> Switch field | <Esc> Back").block(help_block);
        frame.render_widget(help_paragraph, main_chunks[1]);
    }
}
