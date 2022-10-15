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
  sink: Sink,
  currentTime: usize,
  currentLayerIndex: usize,
  tempo: u32,
  cursor: (usize, usize),
  cursorVisible: bool,
  beat: usize,
  msecInBeat: u32,
  shouldExit: bool,
  prevSize: (u16, u16),
}

impl Player {
  fn for_sink(sink: Sink) -> Self {
    Self {
      layers: vec!(GridLayer::default(),
                   GridLayer::default(),
                   GridLayer::default(),
                   GridLayer::default(),
                   GridLayer::default()),
      sink,
      currentTime: 0,
      currentLayerIndex: 0,
      tempo: 200,
      cursor: (3, 5),
      cursorVisible: true,
      beat: 0,
      msecInBeat: 0,
      shouldExit: false,
      prevSize: (0, 0)
    }
  }

  fn draw<W: Write>(&mut self, out: &mut RawTerminal<W>) {
    let layer = &self.layers[self.currentLayerIndex];
    write!(out, "{}", termion::cursor::Hide);
    let (w, h) = termion::terminal_size().unwrap();

    // Clear the whole screen and redraw if we changed size
    if (w, h) != self.prevSize {
      write!(out, "{}", termion::clear::All);
      self.prevSize = (w, h)
    }

    for (n, on) in layer.notes.iter().enumerate() {
      if n % 16 == 0 {
        let row = (n / 16) as u16;
        write!(out, "{}", termion::cursor::Goto(self.boardLeft(), row + self.boardTop()));
      }
      if self.cursorVisible && n % 16 == self.cursor.0 && n / 16 == self.cursor.1 {
        write!(out, "{}", termion::color::Bg(termion::color::Rgb(96, 96, 96)));
      } else if n % 16 == self.beat {
        write!(out, "{}", termion::color::Bg(termion::color::Rgb(96, 96, 128)));
      }
      if *on {
        write!(out, "[]");
      } else {
        write!(out, "--");
      }
      write!(out, "{}", termion::color::Bg(termion::color::Reset));
    }

    write!(out, "{}", termion::cursor::Goto(1, 18));
  }

  fn handleEvent<W: Write>(&mut self, event: Event, stdout: &mut RawTerminal<W>) {
    match event {
      Event::Key(Key::Ctrl('c')) |
      Event::Key(Key::Ctrl('d')) |
      Event::Key(Key::Ctrl('q')) => self.shouldExit = true,

      Event::Key(Key::Left) => if self.cursor.0 > 0 { self.cursor.0 -= 1; },
      Event::Key(Key::Right) => if self.cursor.0 < 15 { self.cursor.0 += 1; },
      Event::Key(Key::Up) => if self.cursor.1 > 0 { self.cursor.1 -= 1; },
      Event::Key(Key::Down) => if self.cursor.1 < 15 { self.cursor.1 += 1; },

      Event::Key(Key::Char('\n')) => self.toggleNote(),

      Event::Mouse(MouseEvent::Press(_, x, y)) =>
        if (x - self.boardLeft()) / 2 <= 16 && y - self.boardTop() <= 16 {
          self.cursor = (((x - self.boardLeft()) / 2) as usize, (y - self.boardTop()) as usize);
          self.toggleNote();
        }
      _ => {}
    }
  }

  fn boardLeft(&self) -> u16 {
    (self.prevSize.0 / 2) - 16 + 1
  }

  fn boardTop(&self) -> u16 { 2 }

  fn toggleNote(&mut self) {
    let mut layer = &mut self.layers[self.currentLayerIndex];
    let n = self.cursor.0 + self.cursor.1 * 16;
    layer.notes[n] = !layer.notes[n];
  }

  fn update(&mut self, dt: u32) {
    self.msecInBeat += dt;

    // The total milliseconds in a minute divided by the bpm
    if self.msecInBeat >= (60000 / self.tempo) {
      self.msecInBeat = 0;
      self.beat = (self.beat + 1) % 16;
      self.playNotes();
    }
  }

  fn playNotes(&self) {
    let (mut controller, mixer) = rodio::dynamic_mixer::mixer(16, 48000);

    let noteNums = [-12, -10, -9, -7, -5, -4, -2, 0, 2, 3, 5, 7, 8, 10, 12, 14];

    for layer in self.layers.iter() {
      for row in 0..16 {
        if layer.notes[self.beat + row * 16] {
          let noteNum = noteNums[15 - row] as f32;
          let freq: f32 = f32::powf(1.0595, noteNum) * 440.0;
          let source = SineWave::new(freq).take_duration(Duration::from_millis(60000 / self.tempo as u64)).amplify(0.30);
          controller.add(source);
        }
      }
    }

    self.sink.append(mixer);
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

  let (_stream, stream_handle) = OutputStream::try_default().unwrap();
  let sink = Sink::try_new(&stream_handle).unwrap();

  let mut player = Player::for_sink(sink);
  let mut n = 0;

  write!(stdout, "{}", termion::clear::All);

  loop {
    player.draw(&mut stdout);

    for c in (&mut stdin).events() {
      player.handleEvent(c.unwrap(), &mut stdout);
    }

    if player.shouldExit { break }
    thread::sleep(Duration::from_millis(25));
    player.update(25);
  }

  write!(stdout, "{}", termion::cursor::Show).unwrap();
}

fn drawScreen() {
  
}
