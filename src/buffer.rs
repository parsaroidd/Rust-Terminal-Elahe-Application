    pub use crossterm::terminal::{self, Clear, ClearType}
;   pub use   std::io::{Write, stdout}
;   pub use crossterm::event::{Event, read, KeyCode, KeyModifiers}
;   pub use crossterm::{QueueableCommand, execute}
;   pub use crossterm::cursor::MoveTo
;

fn take(strr: String) -> String 
{
    strr
}

pub fn buffer()
{
    terminal::enable_raw_mode();

    let mut stream = stdout();
    let mut buffer = String::new(); 

    let (mut current_width, mut current_height) = (1, 10);

    loop
    {
            if current_width   == 0 { current_width  = 1; }
        else if current_height == 0 { current_height = 1; }
        /* preventing a runtime panick, see moving keys */ 


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
                            execute!(stream, Clear(ClearType::CurrentLine), MoveTo(current_width, current_height));
                            stream.write(buffer.as_bytes());
                        }
                    },
                    KeyCode::Esc => break,
                    KeyCode::Backspace => 
                    {
                        buffer.pop().unwrap();
                        execute!(stream, Clear(ClearType::CurrentLine), MoveTo(current_width, current_height));
                        stream.write(buffer.as_bytes());
                    }, 
                    KeyCode::Enter =>
                    {
                        take(buffer);
                        buffer = String::new();
                    }, 
                    navkey @ (KeyCode::Right | KeyCode::Left | KeyCode::Up | KeyCode::Down)  => 
                    {
                        match navkey
                        {
                            KeyCode::Right => current_width  += 1, 
                            KeyCode::Left  => current_width  -= 1, 
                            KeyCode::Down  => current_height += 1,
                            KeyCode::Up    => current_height -= 1, 
                            _ => {},
                        }
                        stream.queue(MoveTo(current_width, current_height));
                    }, 
                    _ => todo!(), 
                }
            }, 
            _ => todo!()
        }
        stream.flush().unwrap()
    }
}
