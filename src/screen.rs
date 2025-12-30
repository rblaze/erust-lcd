use crate::hd44780;

/// Generic code for LCD screen of a given size.
/// User needs to implement send_command() and send_data().
pub trait Screen<const WIDTH: usize, const HEIGHT: usize, Error> {
    /// Sends command byte to the screen.
    fn send_command(&mut self, command: u8) -> Result<(), Error>;
    /// Sends data byte to the screen.
    fn send_data(&mut self, data: u8) -> Result<(), Error>;

    /// Sends multiple command bytes to the screen.
    /// Default implementation just calls send_command() for each byte.
    fn send_commands(&mut self, commands: &[u8]) -> Result<(), Error> {
        for command in commands {
            self.send_command(*command)?;
        }
        Ok(())
    }

    /// Sends multiple data bytes to the screen.
    /// Default implementation just calls send_data() for each byte.
    fn send_data_bytes(&mut self, data: &[u8]) -> Result<(), Error> {
        for byte in data {
            self.send_data(*byte)?;
        }
        Ok(())
    }

    /// Clears screen.
    fn cls(&mut self) -> Result<(), Error> {
        self.send_command(hd44780::clear_screen())
    }

    /// Prints string on the screen. Control characters not supported.
    fn write(&mut self, s: &str) -> Result<(), Error> {
        let mut string_buf = [0; WIDTH];

        // Copy `s` to string buffer, replacing multibyte characters with '?'
        let len = s
            .chars()
            .take(WIDTH)
            .map(|c| if (c as u32) < 256 { c } else { '?' })
            .fold(0, |i, c| {
                string_buf[i] = c as u8;
                i + 1
            });

        self.send_data_bytes(&string_buf[..len])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    enum ScreenError {}

    #[derive(Debug, Default)]
    struct TestScreen {
        commands: Vec<u8>,
        data: Vec<u8>,
    }

    impl TestScreen {
        fn new() -> Self {
            Self {
                commands: Vec::new(),
                data: Vec::new(),
            }
        }
    }

    impl Screen<16, 1, ScreenError> for TestScreen {
        fn send_command(&mut self, command: u8) -> Result<(), ScreenError> {
            self.commands.push(command);
            Ok(())
        }

        fn send_data(&mut self, data: u8) -> Result<(), ScreenError> {
            self.data.push(data);
            Ok(())
        }
    }

    #[test]
    fn cls() {
        let mut screen = TestScreen::new();
        screen.cls().unwrap();
        assert_eq!(screen.commands, vec![hd44780::clear_screen()]);
        assert_eq!(screen.data, vec![]);
    }

    #[test]
    fn write() {
        let mut screen = TestScreen::new();
        screen
            .write("this is very long test string that should be truncated by screen size")
            .unwrap();
        assert_eq!(screen.commands, vec![]);
        assert_eq!(screen.data.as_slice(), b"this is very lon");
    }
}
