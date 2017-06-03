use std::fmt;

use term::Colour as TermColor;
use super::item::Color as ItemColor;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    Blue, Red, Green, Cyan, Magenta, Orange, Pink, Yellow
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Kind {
    Empty, Wall, Agent, Box, Goal
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Item {
    kind: Kind,
    id: u8,
    color: Color
}

const EMPTY_ITEM: Item = Item { kind: Kind::Empty, id: 0, color: Color::Blue };
const WALL_ITEM: Item = Item { kind: Kind::Wall, id: 0, color: Color::Blue };

impl Item {
    pub fn empty() -> Item { EMPTY_ITEM }
    pub fn wall() -> Item { WALL_ITEM }

    pub fn new(chr: u8, clr: Color) -> Item {
        match chr {
            b' '        => EMPTY_ITEM,
            b'+'        => WALL_ITEM,
            b'0'...b'9' => Item { kind: Kind::Agent, id: chr - b'0', color: clr },
            b'A'...b'Z' => Item { kind: Kind::Box,   id: chr - b'A', color: clr },
            b'a'...b'z' => Item { kind: Kind::Goal,  id: chr - b'a', color: clr },
            _           => panic!("invalid char")
        }
    }

    pub fn is_empty(&self) -> bool { self.kind == Kind::Empty }
    pub fn is_wall(&self)  -> bool { self.kind == Kind::Wall }
    pub fn is_agent(&self) -> bool { self.kind == Kind::Agent }
    pub fn is_box(&self)   -> bool { self.kind == Kind::Box }
    pub fn is_goal(&self)  -> bool { self.kind == Kind::Goal }

    pub fn compatible(i: &Item, j: &Item) -> bool {
        if i.is_empty() || j.is_empty() { false }
        else if i.is_wall() || j.is_wall() { false }
        else if i.is_agent() || j.is_agent() { i.color == j.color }
        else { i.color == j.color && i.id == j.id }
    }

    pub fn find_color(name: &str) -> Option<Color> {
        let name_lower = name.to_lowercase();
        match name_lower.as_str() {
            "blue"    => Some(Color::Blue),
            "red"     => Some(Color::Red),
            "green"   => Some(Color::Green),
            "cyan"    => Some(Color::Cyan),
            "magenta" => Some(Color::Magenta),
            "orange"  => Some(Color::Orange),
            "pink"    => Some(Color::Pink),
            "yellow"  => Some(Color::Yellow),
                    _ => None
        }
    }
}

fn map_item_color(c: ItemColor) -> TermColor {
    match c {
        ItemColor::Blue    => TermColor::Fixed(21),
        ItemColor::Red     => TermColor::Fixed(196),
        ItemColor::Green   => TermColor::Fixed(34),
        ItemColor::Cyan    => TermColor::Fixed(44),
        ItemColor::Magenta => TermColor::Fixed(55),
        ItemColor::Orange  => TermColor::Fixed(202),
        ItemColor::Pink    => TermColor::Fixed(199),
        ItemColor::Yellow  => TermColor::Fixed(226)
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use term::Style;

        let st0 = map_item_color(self.color).normal();

        let (st, str) = match self.kind {
            Kind::Empty => (Style::default(), format!(" ")),
            Kind::Wall  => (Style::default().reverse().dimmed(), format!(" ")),
            Kind::Agent => (st0, format!("{}", (b'0'+self.id) as char)),
            Kind::Box   => (st0, format!("{}", (b'A'+self.id) as char)),
            Kind::Goal  => (st0, format!("{}", (b'a'+self.id) as char))
        };

        write!(f, "{}", st.paint(str))
    }
}

impl Default for Item {
    fn default() -> Item { Item::empty() }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test] #[ignore]
    fn display() {
        println!();
        println!("{}", Item::new(b' ', Color::Blue));
        println!("{}", Item::new(b'+', Color::Blue));
        println!("{}", Item::new(b'3', Color::Yellow));
        println!("{}", Item::new(b'E', Color::Blue));
        println!("{}", Item::new(b'e', Color::Green));
        panic!()
    }
}
