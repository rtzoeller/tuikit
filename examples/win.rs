use tuikit::attr::{Attr, Color};
use tuikit::prelude::*;

struct Model(String);

impl Draw for Model {
    fn draw(&self, canvas: &mut Canvas) -> Result<()> {
        let (width, height) = canvas.size()?;
        let message_width = self.0.len();
        let left = (width - message_width) / 2;
        let top = height / 2;
        let _ = canvas.print(top, left, &self.0);
        Ok(())
    }
}

fn main() {
    let term = Term::with_height(TermHeight::Percent(50)).unwrap();
    let model = Model("Hey, I'm in middle!".to_string());

    while let Ok(ev) = term.poll_event() {
        if let Event::Key(Key::Char('q')) = ev {
            break;
        }
        let _ = term.print(0, 0, "press 'q' to exit");

        let inner_win = Win::new(&model).border(true);

        let win = Win::new(&inner_win)
            .margin(Size::Percent(10))
            .padding(Size::Fixed(1))
            .border(true)
            .border_top_attr(Attr {
                fg: Color::BLUE,
                ..Attr::default()
            })
            .border_right_attr(Attr {
                fg: Color::YELLOW,
                ..Attr::default()
            })
            .border_bottom_attr(Attr {
                fg: Color::RED,
                ..Attr::default()
            })
            .border_left_attr(Attr {
                fg: Color::GREEN,
                ..Attr::default()
            });

        let _ = term.draw(&win);
        let _ = term.present();
    }
}