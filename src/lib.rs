#![feature(decl_macro)]

use snap::raw::Decoder;
use std::{
    mem,
    fmt::Debug,
    slice,
    str,
};

pub fn parse<'a>(bytes: Vec<u8>) -> Circuit<'a> {
    assert!(bytes[0] == 6, "unsupported version: {}", bytes[0]);
    let bytes = Decoder::new().decompress_vec(&bytes[1..]).expect("failed to decompress, probably hit end of file");
    Circuit::extract(bytes)
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Point {
    pub x: i16,
    pub y: i16,
}
impl Point {
    fn add(&mut self, other: &Point) {
        self.x += other.x;
        self.y += other.y;
    }
    fn mul(&mut self, other: i16) {
        self.x *= other;
        self.y *= other;
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SyncState {
    Unsynced = 0,
    Synced = 1,
    ChangedAfterSync = 2,
}

#[derive(Debug)]
pub struct Header<'a> {
    pub save_id: u64,
    pub hub_id: u32,
    pub gate: u64,
    pub delay: u64,
    pub menu_visible: bool,
    pub clock_speed: u32,
    pub dependencies: &'a [u64],
    pub description: &'a str,
    pub camera_position: Point,
    pub synced: SyncState,
    pub campaign_bound: bool,
    pub arch_score: u16,
    pub player_data: &'a [u8],
    pub hub_description: &'a str,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u16)]
pub enum ComponentKind {
    Error                   = 0,
    Off                     = 1,
    On                      = 2,
    Buffer1                 = 3,
    Not                     = 4,
    And                     = 5,
    And3                    = 6,
    Nand                    = 7,
    Or                      = 8,
    Or3                     = 9,
    Nor                     = 10,
    Xor                     = 11,
    Xnor                    = 12,
    Counter8                = 13,
    VirtualCounter8         = 14,
    Counter64               = 15,
    VirtualCounter64        = 16,
    Ram8                    = 17,
    VirtualRam8             = 18,
    Deleted0               = 19,
    Deleted1               = 20,
    Stack                   = 21,
    VirtualStack            = 22,
    Register8               = 23,
    VirtualRegister8        = 24,
    Register8Red            = 25,
    VirtualRegister8Red     = 26,
    Register8RedPlus        = 27,
    VirtualRegister8RedPlus = 28,
    Register64              = 29,
    VirtualRegister64       = 30,
    Switch8                 = 31,
    Mux8                    = 32,
    Decoder1                = 33,
    Decoder3                = 34,
    Constant8               = 35,
    Not8                    = 36,
    Or8                     = 37,
    And8                    = 38,
    Xor8                    = 39,
    Equal8                  = 40,
    Deleted2               = 41,
    Deleted3               = 42,
    Neg8                    = 43,
    Add8                    = 44,
    Mul8                    = 45,
    Splitter8               = 46,
    Maker8                  = 47,
    Splitter64              = 48,
    Maker64                 = 49,
    FullAdder               = 50,
    BitMemory               = 51,
    VirtualBitMemory        = 52,
    Deleted10              = 53,
    Decoder2                = 54,
    Timing                  = 55,
    NoteSound               = 56,
    Deleted4               = 57,
    Deleted5               = 58,
    Keyboard                = 59,
    FileLoader              = 60,
    Halt                    = 61,
    WireCluster             = 62,
    LevelScreen             = 63,
    Program8_1              = 64,
    Program8_1Red           = 65,
    Deleted6               = 66,
    Deleted7               = 67,
    Program8_4              = 68,
    LevelGate               = 69,
    Input1                  = 70,
    LevelInput2Pin          = 71,
    LevelInput3Pin          = 72,
    LevelInput4Pin          = 73,
    LevelInputConditions    = 74,
    Input8                  = 75,
    Input64                 = 76,
    LevelInputCode          = 77,
    LevelInputArch          = 78,
    Output1                 = 79,
    LevelOutput1Sum         = 80,
    LevelOutput1Car         = 81,
    Deleted8               = 82,
    Deleted9               = 83,
    LevelOutput2Pin         = 84,
    LevelOutput3Pin         = 85,
    LevelOutput4Pin         = 86,
    Output8                 = 87,
    Output64                = 88,
    LevelOutputArch         = 89,
    LevelOutputCounter      = 90,
    Deleted11              = 91,
    Custom                  = 92,
    VirtualCustom           = 93,
    Program                 = 94,
    DelayLine1              = 95,
    VirtualDelayLine1       = 96,
    Console                 = 97,
    Shl8                    = 98,
    Shr8                    = 99,
  
    Constant64              = 100,
    Not64                   = 101,
    Or64                    = 102,
    And64                   = 103,
    Xor64                   = 104,
    Neg64                   = 105,
    Add64                   = 106,
    Mul64                   = 107,
    Equal64                 = 108,
    LessU64                 = 109,
    LessI64                 = 110,
    Shl64                   = 111,
    Shr64                   = 112,
    Mux64                   = 113,
    Switch64                = 114,
  
    ProbeMemoryBit          = 115,
    ProbeMemoryWord         = 116,
  
    AndOrLatch              = 117,
    NandNandLatch           = 118,
    NorNorLatch             = 119,
  
    LessU8                  = 120,
    LessI8                  = 121,
  
    DotMatrixDisplay        = 122,
    SegmentDisplay          = 123,
  
    Input16                 = 124,
    Input32                 = 125,
  
    Output16                = 126,
    Output32                = 127,
  
    Bidirectional1          = 128,
    Bidirectional8          = 129,
    Bidirectional16         = 130,
    Bidirectional32         = 131,
    Bidirectional64         = 132,
  
    Buffer8                 = 133,
    Buffer16                = 134,
    Buffer32                = 135,
    Buffer64                = 136,
  
    ProbeWireBit            = 137,
    ProbeWireWord           = 138,
  
    Switch1                 = 139,
  
    Output1z                = 140,
    Output8z                = 141,
    Output16z               = 142,
    Output32z               = 143,
    Output64z               = 144,
  
    Constant16              = 145,
    Not16                   = 146,
    Or16                    = 147,
    And16                   = 148,
    Xor16                   = 149,
    Neg16                   = 150,
    Add16                   = 151,
    Mul16                   = 152,
    Equal16                 = 153,
    LessU16                 = 154,
    LessI16                 = 155,
    Shl16                   = 156,
    Shr16                   = 157,
    Mux16                   = 158,
    Switch16                = 159,
    Splitter16              = 160,
    Maker16                 = 161,
    Register16              = 162,
    VirtualRegister16       = 163,
    Counter16               = 164,
    VirtualCounter16        = 165,
  
    Constant32              = 166,
    Not32                   = 167,
    Or32                    = 168,
    And32                   = 169,
    Xor32                   = 170,
    Neg32                   = 171,
    Add32                   = 172,
    Mul32                   = 173,
    Equal32                 = 174,
    LessU32                 = 175,
    LessI32                 = 176,
    Shl32                   = 177,
    Shr32                   = 178,
    Mux32                   = 179,
    Switch32                = 180,
    Splitter32              = 181,
    Maker32                 = 182,
    Register32              = 183,
    VirtualRegister32       = 184,
    Counter32               = 185,
    VirtualCounter32        = 186,
  
    LevelOutput8z           = 187,
  
    Nand8                   = 188,
    Nor8                    = 189,
    Xnor8                   = 190,
    Nand16                  = 191,
    Nor16                   = 192,
    Xnor16                  = 193,
    Nand32                  = 194,
    Nor32                   = 195,
    Xnor32                  = 196,
    Nand64                  = 197,
    Nor64                   = 198,
    Xnor64                  = 199,
  
    Ram                     = 200,
    VirtualRam              = 201,
    RamLatency              = 202,
    VirtualRamLatency       = 203,
  
    RamFast                 = 204,
    VirtualRamFast          = 205,
    Rom                     = 206,
    VirtualRom              = 207,
    SolutionRom             = 208,
    VirtualSolutionRom      = 209,
  
    DelayLine8              = 210,
    VirtualDelayLine8       = 211,
    DelayLine16             = 212,
    VirtualDelayLine16      = 213,
    DelayLine32             = 214,
    VirtualDelayLine32      = 215,
    DelayLine64             = 216,
    VirtualDelayLine64      = 217,
  
    RamDualLoad             = 218,
    VirtualRamDualLoad      = 219,
  
    Hdd                     = 220,
    VirtualHdd              = 221,
  
    Network                 = 222,
  
    Rol8                    = 223,
    Rol16                   = 224,
    Rol32                   = 225,
    Rol64                   = 226,
    Ror8                    = 227,
    Ror16                   = 228,
    Ror32                   = 229,
    Ror64                   = 230,
  
    IndexerBit              = 231,
    IndexerByte             = 232,
  
    DivMod8                 = 233,
    DivMod16                = 234,
    DivMod32                = 235,
    DivMod64                = 236,
  
    SpriteDisplay           = 237,
    ConfigDelay             = 238,
  
    Clock                   = 239,
  
    LevelInput1             = 240,
    LevelInput8             = 241,
    LevelOutput1            = 242,
    LevelOutput8            = 243,

    Ashr8                   = 244,
    Ashr16                  = 245,
    Ashr32                  = 246,
    Ashr64                  = 247,
}

#[derive(Debug)]
pub struct Component<'a> {
    pub kind: ComponentKind,
    pub position: Point,
    pub rotation: u8,
    pub permanent_id: u64,
    pub custom_string: &'a str,
    pub setting_1: u64,
    pub setting_2: u64,
    pub ui_order: i16,
    pub custom_id: u64,
    pub custom_displacement: Point,
    pub selected_programs: Option<Vec<(u64, &'a str)>>
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum WireKind {
    Width1 = 0,
    Width8 = 1,
    Width16 = 2,
    Width32 = 3,
    Width64 = 4,
}

pub struct Wire<'a> {
    pub kind: WireKind,
    pub color: u8,
    pub comment: &'a str,
    pub path: (Point, Point),
}

#[allow(dead_code)]
pub struct Circuit<'a> {
    original_bytes: Vec<u8>,
    pub header: Header<'a>,
    pub components: Vec<Component<'a>>,
    pub wires: Vec<Wire<'a>>
}
impl Circuit<'_> {
    fn extract<'a>(bytes: Vec<u8>) -> Circuit<'a> {
        let ptr = unsafe { bytes.first().unwrap_unchecked() } as *const u8;
        let mut current_offset = 0;

        // macros
        macro simple_extract {
            ($t:ty) => {{
                let res = unsafe { *mem::transmute::<_, &$t>(ptr.offset(current_offset as isize)) };
                current_offset += mem::size_of::<$t>();
                res
            }}
        }
        macro slice_extract {
            ($count:ty[$t:ty]) => {{
                let len = simple_extract!($count);
                let offset = mem::size_of::<$t>() * len as usize;
                let res = unsafe { slice::from_raw_parts::<$t>(mem::transmute::<_, &$t>(ptr.offset(current_offset as isize)), len as usize) };
                current_offset += offset;
                res
            }}
        }
        macro str_extract {
            () => {{
                let bytes = slice_extract!(u16[u8]);
                unsafe { str::from_utf8_unchecked(bytes)}
            }}
        }

        let header = {
            let save_id = simple_extract!(u64);
            let hub_id = simple_extract!(u32);
            let gate = simple_extract!(u64);
            let delay = simple_extract!(u64);
            let menu_visible = simple_extract!(bool);
            let clock_speed = simple_extract!(u32);
            let dependencies = slice_extract!(u16[u64]);
            let description = str_extract!();
            let camera_position = simple_extract!(Point);
            let synced = simple_extract!(SyncState);
            let campaign_bound = simple_extract!(bool);
            let arch_score = simple_extract!(u16);
            let player_data = slice_extract!(u16[u8]);
            let hub_description = str_extract!();
            Header { save_id, hub_id, gate, delay, menu_visible, clock_speed, dependencies, description, camera_position, synced, campaign_bound, arch_score, player_data, hub_description }
        };
        
        let components = {
            let components_count = simple_extract!(u64);
            let mut components = Vec::with_capacity(components_count as usize);
            for _ in 0..components_count {
                let kind = simple_extract!(ComponentKind);
                let position = simple_extract!(Point);
                let rotation = simple_extract!(u8);
                let permanent_id = simple_extract!(u64);
                let custom_string = str_extract!();
                let setting_1 = simple_extract!(u64);
                let setting_2 = simple_extract!(u64);
                let ui_order = simple_extract!(i16);
                let (custom_id, custom_displacement) = if kind == ComponentKind::Custom {
                    (simple_extract!(u64), simple_extract!(Point))
                } else {
                    (0, Point {x: 0, y: 0})
                };
                let selected_programs = if matches!(kind, ComponentKind::Program8_1 | ComponentKind::Program8_4 | ComponentKind::Program) {
                    let len = simple_extract!(u16);
                    let mut res = Vec::with_capacity(len as usize);
                    for _ in 0..len {
                        res.push((simple_extract!(u64), str_extract!()));
                    }
                    Some(res)
                } else {
                    None
                };

                let component = Component { kind, position, rotation, permanent_id, custom_string, setting_1, setting_2, ui_order, custom_id, custom_displacement, selected_programs };
                components.push(component);
            }
            components
        };
        
        let wires = {
            let wires_count = simple_extract!(u64);
            let mut wires = Vec::with_capacity(wires_count as usize);
            const TELEPORT_WIRE: u8 = 0b0010_0000;
            const DIRECTIONS: [Point; 8] = [
                Point{x: 1, y: 0},
                Point{x: 1, y: 1},
                Point{x: 0, y: 1},
                Point{x: -1, y: 1},
                Point{x: -1, y: 0},
                Point{x: -1, y: -1},
                Point{x: 0, y: -1},
                Point{x: 1, y: -1},
            ];
            for _ in 0..wires_count {
                let kind = simple_extract!(WireKind);
                let color = simple_extract!(u8);
                let comment = str_extract!();
                let start = simple_extract!(Point);
                let mut end = start.clone();
                loop {
                    let segment = simple_extract!(u8);
                    if segment == 0 {break;}
                    if segment == TELEPORT_WIRE {
                        end = simple_extract!(Point);
                        break;
                    }
                    let mut direction = DIRECTIONS[(segment >> 5) as usize];
                    let len = (segment & 0b0001_1111) as i16;
                    direction.mul(len);
                    end.add(&direction);
                }
                let path = (start, end);
                let wire = Wire {kind, color, comment, path};
                wires.push(wire)
            }
            wires
        };
        assert_eq!(current_offset, bytes.len(), "the final offset isn't equal to the length of the file");
        Circuit { 
            original_bytes: bytes,
            header,
            components,
            wires,
        }
    }
}
impl Debug for Circuit<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Circuit")
           .field("header", &self.header)
           .field("components", &self.components)
           .finish()
    }
}




#[cfg(test)]
mod tests {
    use std::{fs, time::Instant};
    use super::*;

    fn parse_from_file(path: &str) -> Circuit {
        let bytes = fs::read(path).expect("error reading file");
        parse(bytes)
    }

    #[test]
    fn nandverture() {
        let start = Instant::now();
        let circuit = parse_from_file(r"C:\Users\s17b1\AppData\Roaming\Godot\app_userdata\Turing Complete\schematics\architecture\schematic_hub\NANDverture\circuit.data");
        println!("parsing took {:?}", start.elapsed());
        assert_eq!(circuit.header.clock_speed, 100000);
        assert_eq!(circuit.wires.len(), 6591);
        assert_eq!(circuit.components.len(), 1623);
    }
}

