const GRID_CHARS: [[char; 3]; 3] = [
    ['┌', '┬', '┐'],
    ['├', '┼', '┤'],
    ['└', '┴', '┘'],
];

const HORIZONTAL_LINE: char = '─';
const VERTICAL_LINE: char = '│';

// function print_grid takes an 8 by 8 char array and displays it in a grid
pub fn render_grid(content: &[char; 64], legend: bool) -> String {
    let mut out = String::new();
    for i in 0..8 {
        for j in 0..8 {
            let line: String = std::iter::repeat(HORIZONTAL_LINE).take(3).collect();
            let lead = GRID_CHARS[if i == 0 { 0 } else { 1 }][if j == 0 { 0 } else { 1 }];
            out += format!("{}{}", lead, line).as_str();
        }
        out += format!("{}\n", GRID_CHARS[if i == 0 { 0 } else { 1 }][2]).as_str();

        for j in 0..8 {
            out += format!("{} {} ", VERTICAL_LINE, content[i * 8 + j]).as_str();
        }

        let formatted = if legend {
            format!("{} {}\n", VERTICAL_LINE, 8 - i)
        } else {
            format!("{}\n", VERTICAL_LINE)
        };

        out += formatted.as_str();
    }
    for j in 0..8 {
        let line: String = std::iter::repeat(HORIZONTAL_LINE).take(3).collect();
        let lead = GRID_CHARS[2][if j == 0 { 0 } else { 1 }];
        out += format!("{}{}", lead, line).as_str();
    }
    out += format!("{}\n", GRID_CHARS[2][2]).as_str();

    if legend {
        out += "  A   B   C   D   E   F  G   H\n";
    }

    out
}
