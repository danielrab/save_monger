#![allow(dead_code)]

use std::{collections::HashMap, fs, mem::size_of, str::from_utf8};
use snap::{raw::Decoder};

pub fn get_data_from_file(path: &str) -> Circuit {
    let bytes = fs::read(path).expect("error reading file");
    match bytes[0] {
        6 => {
            let uncompressed = Decoder::new().decompress_vec(&bytes[1..]).expect("failed to decompress, probably hit end of file");
            get_data_from_bytes(&uncompressed)
        },
        v => panic!("unsupported version: {v}"),
    }
}

fn get_data_from_bytes(mut bytes: &[u8]) -> Circuit {
    let bytes_ref = &mut bytes;
    Circuit::extract_new(bytes_ref)
}

trait NewExtractable {
    fn extract_new(bytes: &mut &[u8]) -> Self where Self: Sized;
}

macro_rules! extractable_struct {
    ($visibility:vis $name:ident {$($visibility_inner:vis $field:ident: $t:ty),*$(,)?}) => {
        #[derive(Debug)]
        $visibility struct $name {
            $($visibility_inner $field: $t,)*
        }
        impl NewExtractable for $name {
            fn extract_new(bytes: &mut &[u8]) -> Self  {
                Self{
                    $($field: <$t>::extract_new(bytes),)*
                }
            }
        }
    };
}
extractable_struct!{pub Point {
    pub x: i16,
    pub y: i16,
}}
impl Clone for Point {
    fn clone(&self) -> Self {
        Point { x: self.x, y: self.y }
    }
}
macro_rules! extractable_enum {
    ($int:ty : $visibility:vis $name:ident {$($field:ident = $value:literal),*$(,)?}) => {
        #[derive(Debug)]
        $visibility enum $name {$($field = $value,)*}
        impl From<$int> for $name {
            fn from(orig: $int) -> Self {
                match orig {
                    $($value => Self::$field,)*
                    v => panic!("unexpected {} value: {v}", stringify!($name)),
                }
            }
        }
        impl NewExtractable for $name {
            fn extract_new(bytes: &mut &[u8]) -> Self {
                let res = <$int>::extract_new(bytes);
                res.into()
            }
        }
    };
}
extractable_enum! {u8 : pub SyncState {
    Unsynced = 0,
    Synced = 1,
    ChangedAfterSync = 2,
}}

extractable_enum! {u16 : pub ComponentType {
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
}}


extractable_struct! { pub Header {
    pub save_id: u64,
    pub hub_id: u32,
    pub gate: u64,
    pub delay: u64,
    pub menu_visible: bool,
    pub clock_speed: u32,
    pub dependencies: ShortVec<u64>,
    pub description: String,
    pub camera_position: Point,
    pub synced: SyncState,
    pub campaign_bound: bool,
    pub arch_score: u16,
    pub player_data: ShortVec<u8>,
    pub hub_description: String,
}}

extractable_struct! { pub ComponentData {
    pub kind: ComponentType,
    pub position: Point,
    pub rotation: u8,
    pub permanent_id: u64,
    pub custom_string: String,
    pub setting_1: u64,
    pub setting_2: u64,
    pub ui_order: i16,
}}

extractable_struct! { pub CustomData {
    pub custom_id: u64,
    pub custom_displacement: Point,
}}

impl<T: NewExtractable, U: NewExtractable> NewExtractable for (T, U) {
    fn extract_new(bytes_iter: &mut &[u8]) -> Self where Self: Sized {
        (T::extract_new(bytes_iter), U::extract_new(bytes_iter))
    }
}

#[derive(Debug)]
pub enum Component {
    Normal{common_data: ComponentData},
    Custom{common_data: ComponentData, custom_data: CustomData},
    Program{common_data: ComponentData, program_data: HashMap<u64, String>},
}

impl NewExtractable for Component {
    fn extract_new(bytes_iter: &mut &[u8]) -> Self where Self: Sized {
        let common_data = ComponentData::extract_new(bytes_iter);
        let res = match common_data.kind {
            ComponentType::Custom => { Self::Custom {
                common_data,
                custom_data: CustomData::extract_new(bytes_iter),
            } },
            ComponentType::Program8_1 | ComponentType::Program8_4 | ComponentType::Program => {
                let paris = <ShortVec<(u64, String)>>::extract_new(bytes_iter);
                Self::Program {
                    common_data,
                    program_data: paris.0.into_iter().collect(),
                }
                
            },
            _ => Self::Normal { common_data },
        };
        res
    }
}

extractable_enum! { u8: pub WireKind {
    Width1 = 0,
    Width8 = 1,
    Width16 = 2,
    Width32 = 3,
    Width64 = 4,
}}

#[derive(Debug)]
struct WirePath(Vec<Point>);
impl NewExtractable for WirePath {
    fn extract_new(bytes: &mut &[u8]) -> Self {
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
        let mut current_point = <Point>::extract_new(bytes);
        let mut path = vec![current_point.clone()];
        let mut segment = u8::extract_new(bytes);
        if segment == TELEPORT_WIRE {
            path.push(Point::extract_new(bytes));
            return Self(path);
        }
        while segment != 0 {
            let direction = &DIRECTIONS[(segment >> 5) as usize];
            let len = (segment & 0b0001_1111) as i16;
            let offset = Point { x: direction.x * len, y: direction.y * len };
            current_point = Point{ x: current_point.x + offset.x, y: current_point.y + offset.y };
            path.push(current_point.clone());
            segment = u8::extract_new(bytes);
        }
        Self(path)
    }
}

extractable_struct! { pub Wire {
    kind: WireKind,
    color: u8,
    comment: String,
    path: WirePath
}}

extractable_struct!{ pub Circuit {
    pub header: Header,
    pub component: LongVec<Component>,
    pub wires: LongVec<Wire>,
}}

impl NewExtractable for String {
    fn extract_new(bytes: &mut &[u8]) -> Self {
        from_utf8(&<ShortVec<u8>>::extract_new(bytes).0).unwrap().to_owned()
    }
}

impl NewExtractable for bool {
    fn extract_new(bytes: &mut &[u8]) -> Self where Self: Sized {
        let res = bytes[0] > 0;
        *bytes = &bytes[1..];
        res
    }
}

macro_rules! extractable_int {
    ( $t:ty ) => {
        impl NewExtractable for $t {
            fn extract_new(bytes: &mut &[u8]) -> Self  {
                let res = <$t>::from_le_bytes(bytes[0..size_of::<$t>()].try_into().unwrap());
                *bytes = &bytes[size_of::<$t>()..];
                res
            }
        }
    };
}
extractable_int!(i64);
extractable_int!(i32);
extractable_int!(i16);
extractable_int!(i8);
extractable_int!(u64);
extractable_int!(u32);
extractable_int!(u16);
extractable_int!(u8);

macro_rules! extractable_vec {
    ($visibility:vis $name:ident, $size:ty) => {
        $visibility struct $name<T>(Vec<T>);
        impl<T: NewExtractable> NewExtractable for $name<T> {
            fn extract_new(bytes: &mut &[u8]) -> Self {
                let len = <$size>::extract_new(bytes);
                let mut vec = Vec::with_capacity(len as usize);
                for _ in 0..len {
                    vec.push(T::extract_new(bytes));
                }
                Self(vec)
            }
        }
        impl<T: std::fmt::Debug> std::fmt::Debug for $name<T> {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{:?}", self.0)
            }
        }
    };
}

extractable_vec!(pub ShortVec, u16);
extractable_vec!(pub LongVec, u64);
