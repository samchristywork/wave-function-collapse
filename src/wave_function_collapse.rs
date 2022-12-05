use colored::Colorize;
use std::fmt;
use std::fs;

struct Bitmap {
    width: usize,
    height: usize,
    data: Vec<char>,
}

struct BitmapCollection {
    data: Vec<Bitmap>,
}

impl BitmapCollection {
    fn new() -> Self {
        Self { data: Vec::new() }
    }
}

impl fmt::Display for BitmapCollection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.data
                .iter()
                .map(|e| format!("{}\n", e))
                .collect::<String>()
        )
    }
}

impl Bitmap {
    fn from_str(s: &str, width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: s.chars().filter(|e| *e != '\n' && *e != '\r').collect(),
        }
    }

    fn from_file(filename: &str) -> Self {
        let raw_string: String = fs::read_to_string(filename).unwrap().parse().unwrap();

        let inferred_width = raw_string
            .chars()
            .position(|e| e == '\n' || e == '\r')
            .unwrap();

        let inferred_height = raw_string
            .chars()
            .filter(|e| *e != '\n' && *e != '\r')
            .collect::<String>()
            .len()
            / inferred_width;

        Bitmap::from_str(raw_string.as_str(), inferred_width, inferred_height)
    }

    fn slice(&self, x: usize, y: usize, width: usize, height: usize) -> Bitmap {
        let mut s = String::new();
        for y in y..y + height {
            for x in x..x + width {
                s += self.data[x + y * self.width].to_string().as_str();
            }
        }
        Bitmap::from_str(s.as_str(), width, height)
    }

    fn slices(&self, width: usize, height: usize) -> BitmapCollection {
        let mut out = BitmapCollection::new();
        for x in 0..self.width - width + 1 {
            for y in 0..self.height - height + 1 {
                out.data.push(self.slice(x, y, width, height));
            }
        }
        out
    }
}

impl fmt::Display for Bitmap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out = String::new();
        out += "┌";
        for _ in 0..self.width {
            out += "─"
        }
        out += "┐\n";
        for y in 0..self.height {
            out += "│";
            for x in 0..self.width {
                out += self.data[x + y * self.width].to_string().as_str();
            }
            out += "│\n";
        }
        out += "└";
        for _ in 0..self.width {
            out += "─"
        }
        out += "┘";
        write!(f, "{}", out)
    }
}

struct Grid {
    width: usize,
    height: usize,
    cells: Vec<usize>,
}

impl Grid {
    fn new(cells: Vec<usize>, width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells,
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                out += format!("{}", self.cells[x + y * self.width]).as_str();
            }
            out += "\n";
        }
        write!(f, "{}", out)
    }
}

pub fn foo() {
    let a = Bitmap::from_file("text/input2.txt");
    let b = a.slices(3, 3);
    let c = Grid::new(vec![0, 0, 0, 1], 2, 2);
    println!("{}", format!("hi").yellow());
    println!("{}", a);
    println!("{}", b);
    println!("{}", c);
}
