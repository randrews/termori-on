use std::time::Duration;
use rodio::{OutputStream, Sink};
use rodio::source::{SineWave, Source};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::input::{MouseTerminal, TermRead};
use termion::screen::AlternateScreen;
use termion::event::{Event, Key, MouseEvent};
use termion::async_stdin;
use std::io::{Write, stdout, stdin};
use std::{time, thread};

struct Player {
  layers: Vec<GridLayer>,
  currentTime: usize,
  currentLayerIndex: usize,
  tempo: u32,
  cursor: (usize, usize),
  cursorVisible: bool,
  beat: u32,
  msecInBeat: u32,
  shouldExit: bool,
}

impl Default for Player {
  fn default() -> Self {
    Self {
      layers: vec!(GridLayer::default(),
                   GridLayer::default(),
                   GridLayer::default(),
                   GridLayer::default(),
                   GridLayer::default()),
      currentTime: 0,
      currentLayerIndex: 0,
      tempo: 100,
      cursor: (3, 5),
      cursorVisible: true,
      beat: 0,
      msecInBeat: 0,
      shouldExit: false
    }
  }
}

impl Player {
  fn draw<W: Write>(&self, out: &mut RawTerminal<W>) {
    let layer = &self.layers[self.currentLayerIndex];
    write!(out, "{}{}", termion::cursor::Hide, termion::clear::All);

    for (n, on) in layer.notes.iter().enumerate() {
      if n % 16 == 0 {
        let row = (n / 16) as u16;
        write!(out, "{}", termion::cursor::Goto(1, row + 1));
      }
      if self.cursorVisible && n % 16 == self.cursor.0 && n / 16 == self.cursor.1 {
        write!(out, "{}", termion::color::Bg(termion::color::Rgb(96, 96, 96)));
      }
      if *on {
        write!(out, "[]");
      } else {
        write!(out, "--");
      }
      write!(out, "{}", termion::color::Bg(termion::color::Reset));
    }
  }

  fn handleEvent<W: Write>(&mut self, event: Event, stdout: &mut RawTerminal<W>) {
    match event {
      Event::Key(Key::Ctrl('c')) |
      Event::Key(Key::Ctrl('d')) |
      Event::Key(Key::Ctrl('q')) => {
        self.shouldExit = true;
      },
      Event::Key(Key::Left) => {
        if self.cursor.0 > 0 {
          self.cursor.0 -= 1;
          write!(stdout, "blah!");
        }
      },
      Event::Key(Key::Right) => if self.cursor.0 < 15 { self.cursor = (self.cursor.0 + 1, self.cursor.1) },
      Event::Key(Key::Up) => if self.cursor.1 > 0 { self.cursor.1 -= 1 },
      Event::Key(Key::Down) => if self.cursor.1 < 15 { self.cursor.1 += 1 },

      //Event::Key(Key::Char(c)) => write!(stdout, "{}", c).unwrap(),
      //Event::Key(Key::Alt(c)) => write!(stdout, "^{}", c).unwrap(),
      //Event::Key(Key::Ctrl(c)) => write!(stdout, "*{}", c).unwrap(),
      //Event::Key(Key::Esc) => write!(stdout, "ESC").unwrap(),
      //Event::Key(Key::Backspace) => write!(stdout, "×").unwrap(),
      //Event::Mouse(MouseEvent::Press(_, x, y)) => write!(stdout, "x: {}, y: {}", x, y).unwrap(),
      _ => {}
    }
  }
}

struct GridLayer {
  notes: [bool; 16 * 16],
  // TODO: different wave types here
}

impl Default for GridLayer {
  fn default() -> Self {
    Self {
      notes: [false; 16 * 16]
    }
  }
}

fn main() {
  let mut stdout = MouseTerminal::from(stdout().into_raw_mode().expect("Unable to enter raw mode; try a different terminal?"));
  let mut stdin = async_stdin();

  let mut player = Player::default();
  let mut n = 0;

  loop {
    player.draw(&mut stdout);
    stdout.flush().unwrap();

    write!(stdout, "{}{}", termion::cursor::Goto(1, 20), n);
    n += 1;

    write!(stdout, "{}", termion::cursor::Goto(1, 18));
    stdout.flush().unwrap();

    for c in (&mut stdin).events() {
      player.handleEvent(c.unwrap(), &mut stdout);
    }
    if player.shouldExit { break }
    thread::sleep(Duration::from_millis(25));
  }

  write!(stdout, "{}", termion::cursor::Show).unwrap();

  let (_stream, stream_handle) = OutputStream::try_default().unwrap();
  let sink = Sink::try_new(&stream_handle).unwrap();

  // Add a dummy source of the sake of the example.
  let source = SineWave::new(440.0).take_duration(Duration::from_secs_f32(0.25)).amplify(0.20);
  //sink.append(source);

  // The sound plays in a separate thread. This call will block the current thread until the sink
  // has finished playing all its queued sounds.
  sink.sleep_until_end();
}

fn drawScreen() {
  
}
