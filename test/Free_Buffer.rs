/*************************************************************************************************
 *   this is a Free buffer that takes input wherever you place your mouse, or move the cursor    * 
 *   via direction keys and return String type. You can see what is the job of each key in line  *
 *   59 and below, With insert you get a clear screen and best way to exit is Esc.               *
 *   ______________________Created by @ParsaRoidd on github_____________________                 *
 *************************************************************************************************/


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

fn main()
{
    use std::io::stdout;
    let mut stream = stdout();
    let input: String = buffer(&mut stream);
    println!("{:?}", input);
}


pub fn buffer(stream: &mut std::io::Stdout) -> String
{
    use std::io::Write;
    terminal::enable_raw_mode();

    let mut buffer = String::new(); 

    execute!(stream, EnableMouseCapture); 

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
                        /* Follow me on github! @ParsaRoidd */
                        {
                            buffer.push(key);
                            execute!(stream, Clear(ClearType::CurrentLine), MoveTo(current_width, current_height));
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
                        execute!(stream, Clear(ClearType::CurrentLine), MoveTo(current_width, current_height));
                        stream.write(buffer.as_bytes());
                    }, 
                    KeyCode::Enter =>
                    {
                        execute!(stream, Clear(ClearType::All));
                        buffer = String::new();
                        return buffer;
                    },
                    KeyCode::Insert =>
                    {
                        execute!(stream, Clear(ClearType::All), MoveTo(current_width, current_height));
                        stream.write(buffer.as_bytes());
                    }
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
                        current_width = event.column;
                        current_height = event.row;
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
