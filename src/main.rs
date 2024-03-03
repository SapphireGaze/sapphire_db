use std::io::{self, Write};

struct InputBuffer {
    buffer: String,
}

impl InputBuffer {
    fn new() -> InputBuffer {
        InputBuffer {
            buffer: String::new(),
        }
    }

    fn read_input(&mut self) {
        print!("db > ");
        io::stdout().flush().expect("Error flushing stdout");

        self.buffer.clear();
        io::stdin()
            .read_line(&mut self.buffer)
            .expect("Error reading input");

        // trim newline character
        self.buffer = self.buffer.trim().to_string();
    }
}

fn main() {
    let mut input_buffer = InputBuffer::new();

    loop {
        input_buffer.read_input();

        if input_buffer.buffer == "exit" {
            break;
        }

        println!("Unrecognized command '{}'", input_buffer.buffer);
    }
}
