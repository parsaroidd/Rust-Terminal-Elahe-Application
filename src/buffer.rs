pub use crossterm::{
        terminal::{ self, Clear, ClearType }, 
        event::{
            Event, read, KeyCode, KeyModifiers, DisableMouseCapture, EnableMouseCapture, MouseEventKind,
            MouseButton
        },
        QueueableCommand, 
        execute,
        cursor::MoveTo

};
use std::io::Write;
const MARGIN: (u16, u16, &str) = (5, 2, "&"); 
const WARNING: (u16, u16) = (10, 10);

pub fn buffer(stream: &mut std::io::Stdout, size: (u16, u16)) -> String
{
    terminal::enable_raw_mode();

    let mut buffer = String::new(); 
    execute!(stream, EnableMouseCapture); 
    let (mut current_width, mut current_height) = (3, size.1 - MARGIN.0 - 1);
    let mut free = false;
    /*warp it in a if statement so you can't just wonder around unless I want or You want*/

    loop
    {
            if current_width   == 0 { current_width  = 1; }
        else if current_height == 0 { current_height = 1; }
        /* preventing a runtime panick, see moving keys */

        if buffer.len() as u16 > size.0 * 3 - 19 
        {
            buffer.pop(); 
        }

        if current_height > size.1 - MARGIN.0 && current_height < size.1 - MARGIN.1
        {
            stream.queue(MoveTo(1, current_height));
            stream.write(MARGIN.2.as_bytes());
            stream.queue(MoveTo(size.0 - 1, current_height));
            stream.write(MARGIN.2.as_bytes());
        }

        match read().unwrap()
        {
            Event::Key(event) =>
            {
                match event.code
                {
                    KeyCode::Char(key) =>
                    {
                        if key == 'c' && event.modifiers.contains(KeyModifiers::CONTROL)
                        {
                            stream.write("You cannot escape dear:)))".as_bytes());
                        }
                        else
                        {
                            buffer.push(key);
                            if current_width > size.0 - 4
                            {
                                current_width = 3;
                            }
                            execute!(stream, Clear(ClearType::UntilNewLine), MoveTo(current_width, current_height));
                            stream.write(buffer.as_bytes());
                        }
                    },
                    KeyCode::Esc =>
                    {
                        execute!(stream, DisableMouseCapture);
                        terminal::disable_raw_mode();
                        return buffer;
                    }
                    KeyCode::Backspace => 
                    {
                        buffer.pop().unwrap();
                        execute!(stream, Clear(ClearType::UntilNewLine), MoveTo(current_width, current_height));
                        stream.write(buffer.as_bytes());
                    }, 
                    KeyCode::Enter =>
                    {
                        execute!(stream, DisableMouseCapture);
                        terminal::disable_raw_mode();

                        stream.queue(MoveTo(0, size.1));

                        return buffer;
                    },
                    KeyCode::Delete =>
                    {
                        execute!(stream, Clear(ClearType::All), MoveTo(current_width, current_height));
                        stream.write(buffer.as_bytes());
                    }, 
                    KeyCode::Insert =>
                    {
                        free = true;
                    },
                    navkey @ (KeyCode::Right | KeyCode::Left | KeyCode::Up | KeyCode::Down)  => 
                    {
                        if free
                        {
                            match navkey
                            {
                                KeyCode::Right => current_width  += 1, 
                                KeyCode::Left  => current_width  -= 1, 
                                KeyCode::Down  => current_height += 1,
                                KeyCode::Up    => current_height -= 1, 
                                _ => {},
                            }
                        }
                        stream.queue(MoveTo(current_width, current_height));
                        /*You can navigate in terminal, but you cannot delete characters! for now*/

                    }, 
                    _ => todo!(), 
                }
            },
            Event::Mouse(event) =>
            {
                match event.kind
                {
                    MouseEventKind::Down(MouseButton::Left) => 
                    {
                        if free { 
                            current_width = event.column;
                            current_height = event.row;
                        }
                        stream.queue(MoveTo(current_width, current_height));
                    }, 
                    _ => {}, //buffer = format!("{:?}", event), 
                }
            }, 
            _ => todo!()
        }
        stream.flush().unwrap()
    }
}
