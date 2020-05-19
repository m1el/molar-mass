#[derive(Debug, Clone)]
pub struct Element {
    pub name: &'static str,
    pub z: usize,
    pub sym: &'static str,
    pub mass: f64,
}
pub const PERIODIC_TABLE: &[Element] = &[
    Element {name: "Hydrogen", z: 1, sym: "H", mass: 1.008},
    Element {name: "Helium", z: 2, sym: "He", mass: 4.0026},
    Element {name: "Lithium", z: 3, sym: "Li", mass: 6.94},
    Element {name: "Beryllium", z: 4, sym: "Be", mass: 9.0122},
    Element {name: "Boron", z: 5, sym: "B", mass: 10.81},
    Element {name: "Carbon", z: 6, sym: "C", mass: 12.011},
    Element {name: "Nitrogen", z: 7, sym: "N", mass: 14.007},
    Element {name: "Oxygen", z: 8, sym: "O", mass: 15.999},
    Element {name: "Fluorine", z: 9, sym: "F", mass: 18.998},
    Element {name: "Neon", z: 10, sym: "Ne", mass: 20.18},
    Element {name: "Sodium", z: 11, sym: "Na", mass: 22.99},
    Element {name: "Magnesium", z: 12, sym: "Mg", mass: 24.305},
    Element {name: "Aluminium", z: 13, sym: "Al", mass: 26.982},
    Element {name: "Silicon", z: 14, sym: "Si", mass: 28.085},
    Element {name: "Phosphorus", z: 15, sym: "P", mass: 30.974},
    Element {name: "Sulfur", z: 16, sym: "S", mass: 32.06},
    Element {name: "Chlorine", z: 17, sym: "Cl", mass: 35.45},
    Element {name: "Argon", z: 18, sym: "Ar", mass: 39.95},
    Element {name: "Potassium", z: 19, sym: "K", mass: 39.098},
    Element {name: "Calcium", z: 20, sym: "Ca", mass: 40.078},
    Element {name: "Scandium", z: 21, sym: "Sc", mass: 44.956},
    Element {name: "Titanium", z: 22, sym: "Ti", mass: 47.867},
    Element {name: "Vanadium", z: 23, sym: "V", mass: 50.942},
    Element {name: "Chromium", z: 24, sym: "Cr", mass: 51.996},
    Element {name: "Manganese", z: 25, sym: "Mn", mass: 54.938},
    Element {name: "Iron", z: 26, sym: "Fe", mass: 55.845},
    Element {name: "Cobalt", z: 27, sym: "Co", mass: 58.933},
    Element {name: "Nickel", z: 28, sym: "Ni", mass: 58.693},
    Element {name: "Copper", z: 29, sym: "Cu", mass: 63.546},
    Element {name: "Zinc", z: 30, sym: "Zn", mass: 65.38},
    Element {name: "Gallium", z: 31, sym: "Ga", mass: 69.723},
    Element {name: "Germanium", z: 32, sym: "Ge", mass: 72.63},
    Element {name: "Arsenic", z: 33, sym: "As", mass: 74.922},
    Element {name: "Selenium", z: 34, sym: "Se", mass: 78.971},
    Element {name: "Bromine", z: 35, sym: "Br", mass: 79.904},
    Element {name: "Krypton", z: 36, sym: "Kr", mass: 83.798},
    Element {name: "Rubidium", z: 37, sym: "Rb", mass: 85.468},
    Element {name: "Strontium", z: 38, sym: "Sr", mass: 87.62},
    Element {name: "Yttrium", z: 39, sym: "Y", mass: 88.906},
    Element {name: "Zirconium", z: 40, sym: "Zr", mass: 91.224},
    Element {name: "Niobium", z: 41, sym: "Nb", mass: 92.906},
    Element {name: "Molybdenum", z: 42, sym: "Mo", mass: 95.95},
    Element {name: "Technetium", z: 43, sym: "Tc", mass: 97.0},
    Element {name: "Ruthenium", z: 44, sym: "Ru", mass: 101.07},
    Element {name: "Rhodium", z: 45, sym: "Rh", mass: 102.91},
    Element {name: "Palladium", z: 46, sym: "Pd", mass: 106.42},
    Element {name: "Silver", z: 47, sym: "Ag", mass: 107.87},
    Element {name: "Cadmium", z: 48, sym: "Cd", mass: 112.41},
    Element {name: "Indium", z: 49, sym: "In", mass: 114.82},
    Element {name: "Tin", z: 50, sym: "Sn", mass: 118.71},
    Element {name: "Antimony", z: 51, sym: "Sb", mass: 121.76},
    Element {name: "Tellurium", z: 52, sym: "Te", mass: 127.6},
    Element {name: "Iodine", z: 53, sym: "I", mass: 126.9},
    Element {name: "Xenon", z: 54, sym: "Xe", mass: 131.29},
    Element {name: "Caesium", z: 55, sym: "Cs", mass: 132.91},
    Element {name: "Barium", z: 56, sym: "Ba", mass: 137.33},
    Element {name: "Lanthanum", z: 57, sym: "La", mass: 138.91},
    Element {name: "Cerium", z: 58, sym: "Ce", mass: 140.12},
    Element {name: "Praseodymium", z: 59, sym: "Pr", mass: 140.91},
    Element {name: "Neodymium", z: 60, sym: "Nd", mass: 144.24},
    Element {name: "Promethium", z: 61, sym: "Pm", mass: 145.0},
    Element {name: "Samarium", z: 62, sym: "Sm", mass: 150.36},
    Element {name: "Europium", z: 63, sym: "Eu", mass: 151.96},
    Element {name: "Gadolinium", z: 64, sym: "Gd", mass: 157.25},
    Element {name: "Terbium", z: 65, sym: "Tb", mass: 158.93},
    Element {name: "Dysprosium", z: 66, sym: "Dy", mass: 162.5},
    Element {name: "Holmium", z: 67, sym: "Ho", mass: 164.93},
    Element {name: "Erbium", z: 68, sym: "Er", mass: 167.26},
    Element {name: "Thulium", z: 69, sym: "Tm", mass: 168.93},
    Element {name: "Ytterbium", z: 70, sym: "Yb", mass: 173.05},
    Element {name: "Lutetium", z: 71, sym: "Lu", mass: 174.97},
    Element {name: "Hafnium", z: 72, sym: "Hf", mass: 178.49},
    Element {name: "Tantalum", z: 73, sym: "Ta", mass: 180.95},
    Element {name: "Tungsten", z: 74, sym: "W", mass: 183.84},
    Element {name: "Rhenium", z: 75, sym: "Re", mass: 186.21},
    Element {name: "Osmium", z: 76, sym: "Os", mass: 190.23},
    Element {name: "Iridium", z: 77, sym: "Ir", mass: 192.22},
    Element {name: "Platinum", z: 78, sym: "Pt", mass: 195.08},
    Element {name: "Gold", z: 79, sym: "Au", mass: 196.97},
    Element {name: "Mercury", z: 80, sym: "Hg", mass: 200.59},
    Element {name: "Thallium", z: 81, sym: "Tl", mass: 204.38},
    Element {name: "Lead", z: 82, sym: "Pb", mass: 207.2},
    Element {name: "Bismuth", z: 83, sym: "Bi", mass: 208.98},
    Element {name: "Polonium", z: 84, sym: "Po", mass: 209.0},
    Element {name: "Astatine", z: 85, sym: "At", mass: 210.0},
    Element {name: "Radon", z: 86, sym: "Rn", mass: 222.0},
    Element {name: "Francium", z: 87, sym: "Fr", mass: 223.0},
    Element {name: "Radium", z: 88, sym: "Ra", mass: 226.0},
    Element {name: "Actinium", z: 89, sym: "Ac", mass: 227.0},
    Element {name: "Thorium", z: 90, sym: "Th", mass: 232.04},
    Element {name: "Protactinium", z: 91, sym: "Pa", mass: 231.04},
    Element {name: "Uranium", z: 92, sym: "U", mass: 238.03},
    Element {name: "Neptunium", z: 93, sym: "Np", mass: 237.0},
    Element {name: "Plutonium", z: 94, sym: "Pu", mass: 244.0},
    Element {name: "Americium", z: 95, sym: "Am", mass: 243.0},
    Element {name: "Curium", z: 96, sym: "Cm", mass: 247.0},
    Element {name: "Berkelium", z: 97, sym: "Bk", mass: 247.0},
    Element {name: "Californium", z: 98, sym: "Cf", mass: 251.0},
    Element {name: "Einsteinium", z: 99, sym: "Es", mass: 252.0},
    Element {name: "Fermium", z: 100, sym: "Fm", mass: 257.0},
    Element {name: "Mendelevium", z: 101, sym: "Md", mass: 258.0},
    Element {name: "Nobelium", z: 102, sym: "No", mass: 259.0},
    Element {name: "Lawrencium", z: 103, sym: "Lr", mass: 266.0},
    Element {name: "Rutherfordium", z: 104, sym: "Rf", mass: 267.0},
    Element {name: "Dubnium", z: 105, sym: "Db", mass: 268.0},
    Element {name: "Seaborgium", z: 106, sym: "Sg", mass: 269.0},
    Element {name: "Bohrium", z: 107, sym: "Bh", mass: 270.0},
    Element {name: "Hassium", z: 108, sym: "Hs", mass: 269.0},
    Element {name: "Meitnerium", z: 109, sym: "Mt", mass: 278.0},
    Element {name: "Darmstadtium", z: 110, sym: "Ds", mass: 281.0},
    Element {name: "Roentgenium", z: 111, sym: "Rg", mass: 282.0},
    Element {name: "Copernicium", z: 112, sym: "Cn", mass: 285.0},
    Element {name: "Nihonium", z: 113, sym: "Nh", mass: 286.0},
    Element {name: "Flerovium", z: 114, sym: "Fl", mass: 289.0},
    Element {name: "Moscovium", z: 115, sym: "Mc", mass: 290.0},
    Element {name: "Livermorium", z: 116, sym: "Lv", mass: 293.0},
    Element {name: "Tennessine", z: 117, sym: "Ts", mass: 294.0},
    Element {name: "Oganesson", z: 118, sym: "Og", mass: 294.0},
];

const BLOCK_SIZE: usize = 16;

// The following code was generated by generate_symbol_btree
const SORTED_SYMBOLS: &'static[([u8; 2], u16)] = &[
    (*b"Ac", 89), (*b"Ag", 47), (*b"Al", 13), (*b"Am", 95),
    (*b"Ar", 18), (*b"As", 33), (*b"At", 85), (*b"Au", 79),
    (*b"B\0", 5), (*b"Ba", 56), (*b"Be",  4), (*b"Bh", 107),
    (*b"Bi", 83), (*b"Bk", 97), (*b"Br", 35), (*b"C\0", 6),
    //
    (*b"Ca", 20), (*b"Cd", 48), (*b"Ce", 58), (*b"Cf", 98),
    (*b"Cl", 17), (*b"Cm", 96), (*b"Cn", 112), (*b"Co", 27),
    (*b"Cr", 24), (*b"Cs", 55), (*b"Cu", 29), (*b"Db", 105),
    (*b"Ds", 110), (*b"Dy", 66), (*b"Er", 68), (*b"Es", 99),
    //
    (*b"Eu", 63), (*b"F\0", 9), (*b"Fe", 26), (*b"Fl", 114),
    (*b"Fm", 100), (*b"Fr", 87), (*b"Ga", 31), (*b"Gd", 64),
    (*b"Ge", 32), (*b"H\0", 1), (*b"He", 2), (*b"Hf", 72),
    (*b"Hg", 80), (*b"Ho", 67), (*b"Hs", 108), (*b"I\0", 53),
    //
    (*b"In", 49), (*b"Ir", 77), (*b"K\0", 19), (*b"Kr", 36),
    (*b"La", 57), (*b"Li", 3), (*b"Lr", 103), (*b"Lu", 71),
    (*b"Lv", 116), (*b"Mc", 115), (*b"Md", 101), (*b"Mg", 12),
    (*b"Mn", 25), (*b"Mo", 42), (*b"Mt", 109), (*b"N\0", 7),
    //
    (*b"Na", 11), (*b"Nb", 41), (*b"Nd", 60), (*b"Ne", 10),
    (*b"Nh", 113), (*b"Ni", 28), (*b"No", 102), (*b"Np", 93),
    (*b"O\0", 8), (*b"Og", 118), (*b"Os", 76), (*b"P\0", 15),
    (*b"Pa", 91), (*b"Pb", 82), (*b"Pd", 46), (*b"Pm", 61),
    //
    (*b"Po", 84), (*b"Pr", 59), (*b"Pt", 78), (*b"Pu", 94),
    (*b"Ra", 88), (*b"Rb", 37), (*b"Re", 75), (*b"Rf", 104),
    (*b"Rg", 111), (*b"Rh", 45), (*b"Rn", 86), (*b"Ru", 44),
    (*b"S\0", 16), (*b"Sb", 51), (*b"Sc", 21), (*b"Se", 34),
    //
    (*b"Sg", 106), (*b"Si", 14), (*b"Sm", 62), (*b"Sn", 50),
    (*b"Sr", 38), (*b"Ta", 73), (*b"Tb", 65), (*b"Tc", 43),
    (*b"Te", 52), (*b"Th", 90), (*b"Ti", 22), (*b"Tl", 81),
    (*b"Tm", 69), (*b"Ts", 117), (*b"U\0", 92), (*b"V\0", 23),
    //
    (*b"W\0", 74), (*b"Xe", 54), (*b"Y\0", 39), (*b"Yb", 70),
    (*b"Zn", 30), (*b"Zr", 40),
];

// These indicies point to the start of the block that ends
// with a listed element. (last_key, start_of_block)
// This is basically an implementation of static BTreeMap.
pub const TOP_INDICES: &'static[([u8; 2], u16)] = &[
    (*b"C\0", 0), (*b"Es", 16), (*b"I\0", 32), (*b"N\0", 48),
    (*b"Pm", 64), (*b"Se", 80), (*b"V\0", 96), (*b"Zr", 112),
];

#[allow(dead_code)]
pub fn generate_symbol_btree() {
    let mut indicies = Vec::new();
    let mut table = PERIODIC_TABLE.to_vec();

    println!("// The following code was generated by generate_symbol_btree");
    print!("const SORTED_SYMBOLS: [([u8; 2], u16); {}] = &[", table.len());
    table.sort_by_key(|el| el.sym);
    for (idx, el) in table.iter().enumerate() {
        if idx & 3 == 0 {
            print!("\n   ");
        }
        let head = &el.sym[..1];
        let rest = if el.sym.len() == 1 { "\\0" } else { &el.sym[1..] };
        print!(" (*b\"{}{}\", {}),", head, rest, el.z);
        if idx != 0 && idx & 0xf == 0xf {
            print!("\n    //");
            indicies.push((el.sym, idx));
        }
    }
    let len = table.len();
    if len & 0xf != 0 {
        let last = table.last().unwrap();
        indicies.push((last.sym, len - 1));
    }
    println!("\n];");
    println!("");
    println!("// These indicies point to the start of the block that ends");
    println!("// with a listed element. (last_key, start_of_block)");
    println!("// This is basically an implementation of static BTreeMap.");
    print!("pub const TOP_INDICES: [([u8; 2], u16); {}] = [", indicies.len());
    for (ii, (sym, idx)) in indicies.iter().copied().enumerate() {
        if ii & 3 == 0 {
            print!("\n   ");
        }
        let head = &sym[..1];
        let rest = if sym.len() == 1 { "\\0" } else { &sym[1..] };
        print!(" (*b\"{}{}\", {}),", head, rest, idx);
    }
    println!("\n];");
}

fn sym_to_bytes(symbol: &str) -> Option<[u8; 2]> {
    let symbol = symbol.as_bytes();
    if symbol.len() < 1 || symbol.len() > 2 {
        return None;
    }
    Some([symbol[0], symbol.get(1).copied().unwrap_or(0)])
}

pub fn lookup_charge(symbol: &str) -> Option<usize> {
    use core::cmp::Ordering;

    let sym = sym_to_bytes(symbol)?;

    for (index_sym, idx) in TOP_INDICES.iter().copied() {
        let idx = idx as usize;
        match index_sym.cmp(&sym) {
            Ordering::Equal => {
                let charge = SORTED_SYMBOLS[idx + BLOCK_SIZE - 1].1;
                return Some(charge as usize);
            }
            Ordering::Greater => {
                // -1 because we're checking for last element in ::Equal
                let end = SORTED_SYMBOLS.len().min(idx + BLOCK_SIZE - 1);
                let block = &SORTED_SYMBOLS[idx..end];
                for (s, z) in block.iter().copied() {
                    if s == sym {
                        return Some(z as usize);
                    }
                }
                return None;
            }
            Ordering::Less => {
                // continue;
            }
        }
    }
    return None;
}

pub fn lookup_element(symbol: &str) -> Option<&Element> {
    let charge = lookup_charge(symbol)?;
    Some(&PERIODIC_TABLE[charge - 1])
}
