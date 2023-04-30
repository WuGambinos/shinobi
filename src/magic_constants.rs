use crate::SMagic;
#[rustfmt::skip]
pub const BISHOP_BITS: [u32; 64] = [
    6, 5, 5, 5, 5, 5, 5, 6, 
    5, 5, 5, 5, 5, 5, 5, 5, 
    5, 5, 7, 7, 7, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 7, 7, 7, 5, 5, 
    5, 5, 5, 5, 5, 5, 5, 5, 
    6, 5, 5, 5, 5, 5, 5, 6,
];

#[rustfmt::skip]
pub const ROOK_BITS: [u32; 64] = [
    12, 11, 11, 11, 11, 11, 11, 12,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11, 
    12, 11, 11, 11, 11, 11, 11, 12,
];

#[rustfmt::skip]
pub const BIT_TABLE: [u64; 64] = [
    63, 30, 3 , 32, 25, 41, 22, 33,
    15, 50, 42, 13, 11, 53, 19, 34,
    61, 29, 2 , 51, 21, 43, 45, 10,
    18, 47, 1 , 54, 9 , 57, 0 , 35,
    62, 31, 40, 4 , 49, 5 , 52, 26,
    60, 6 , 23, 44, 46, 27, 56, 16,
    7 , 39, 48, 24, 59, 14, 12, 55,
    38, 28, 58, 20, 37, 17, 36, 8 ,
];

pub const ROOK_MAGICS: [SMagic; 64] = [
    SMagic {
        mask: 0x000101010101017E,
        magic: 0x8080104000208003,
        shift: 52,
        offset: 1,
    },
    SMagic {
        mask: 0x000202020202027C,
        magic: 0x0040100040002000,
        shift: 53,
        offset: 4096,
    },
    SMagic {
        mask: 0x000404040404047A,
        magic: 0x2200084082002010,
        shift: 53,
        offset: 6144,
    },
    SMagic {
        mask: 0x0008080808080876,
        magic: 0x0200044008220010,
        shift: 53,
        offset: 8192,
    },
    SMagic {
        mask: 0x001010101010106E,
        magic: 0x0300080100240230,
        shift: 53,
        offset: 10240,
    },
    SMagic {
        mask: 0x002020202020205E,
        magic: 0x0200100401020008,
        shift: 53,
        offset: 12288,
    },
    SMagic {
        mask: 0x004040404040403E,
        magic: 0xC480020001000080,
        shift: 53,
        offset: 14336,
    },
    SMagic {
        mask: 0x008080808080807E,
        magic: 0x0200004204008021,
        shift: 52,
        offset: 16384,
    },
    SMagic {
        mask: 0x0001010101017E00,
        magic: 0x1114800040002282,
        shift: 53,
        offset: 20480,
    },
    SMagic {
        mask: 0x0002020202027C00,
        magic: 0x8500C00020005004,
        shift: 54,
        offset: 22528,
    },
    SMagic {
        mask: 0x0004040404047A00,
        magic: 0x0841002000184100,
        shift: 54,
        offset: 23552,
    },
    SMagic {
        mask: 0x0008080808087600,
        magic: 0x0001001000200901,
        shift: 54,
        offset: 24576,
    },
    SMagic {
        mask: 0x0010101010106E00,
        magic: 0x0608800800800400,
        shift: 54,
        offset: 25600,
    },
    SMagic {
        mask: 0x0020202020205E00,
        magic: 0x4000800400800200,
        shift: 54,
        offset: 26624,
    },
    SMagic {
        mask: 0x0040404040403E00,
        magic: 0x2022001A00046801,
        shift: 54,
        offset: 27648,
    },
    SMagic {
        mask: 0x0080808080807E00,
        magic: 0x2003000100038042,
        shift: 53,
        offset: 28672,
    },
    SMagic {
        mask: 0x00010101017E0100,
        magic: 0x8440008020800041,
        shift: 53,
        offset: 30720,
    },
    SMagic {
        mask: 0x00020202027C0200,
        magic: 0x081000C00040A000,
        shift: 54,
        offset: 32768,
    },
    SMagic {
        mask: 0x00040404047A0400,
        magic: 0x0000410020001100,
        shift: 54,
        offset: 33792,
    },
    SMagic {
        mask: 0x0008080808760800,
        magic: 0x0910010010090020,
        shift: 54,
        offset: 34816,
    },
    SMagic {
        mask: 0x00101010106E1000,
        magic: 0x0200050010080100,
        shift: 54,
        offset: 35840,
    },
    SMagic {
        mask: 0x00202020205E2000,
        magic: 0x4000808002000401,
        shift: 54,
        offset: 36864,
    },
    SMagic {
        mask: 0x00404040403E4000,
        magic: 0x2000040002282110,
        shift: 54,
        offset: 37888,
    },
    SMagic {
        mask: 0x00808080807E8000,
        magic: 0x8480020000804124,
        shift: 53,
        offset: 38912,
    },
    SMagic {
        mask: 0x000101017E010100,
        magic: 0x10A0400080002080,
        shift: 53,
        offset: 40960,
    },
    SMagic {
        mask: 0x000202027C020200,
        magic: 0x1800200080400081,
        shift: 54,
        offset: 43008,
    },
    SMagic {
        mask: 0x000404047A040400,
        magic: 0x8420008280100020,
        shift: 54,
        offset: 44032,
    },
    SMagic {
        mask: 0x0008080876080800,
        magic: 0x8010001080800800,
        shift: 54,
        offset: 45056,
    },
    SMagic {
        mask: 0x001010106E101000,
        magic: 0x108E080100110004,
        shift: 54,
        offset: 46080,
    },
    SMagic {
        mask: 0x002020205E202000,
        magic: 0x2400020080040080,
        shift: 54,
        offset: 47104,
    },
    SMagic {
        mask: 0x004040403E404000,
        magic: 0x00C8080400020110,
        shift: 54,
        offset: 48128,
    },
    SMagic {
        mask: 0x008080807E808000,
        magic: 0x10410C22000082C1,
        shift: 53,
        offset: 49152,
    },
    SMagic {
        mask: 0x0001017E01010100,
        magic: 0x0080002010400044,
        shift: 53,
        offset: 51200,
    },
    SMagic {
        mask: 0x0002027C02020200,
        magic: 0x3000200282804000,
        shift: 54,
        offset: 53248,
    },
    SMagic {
        mask: 0x0004047A04040400,
        magic: 0x8640801000802000,
        shift: 54,
        offset: 54272,
    },
    SMagic {
        mask: 0x0008087608080800,
        magic: 0x0102012142000811,
        shift: 54,
        offset: 55296,
    },
    SMagic {
        mask: 0x0010106E10101000,
        magic: 0x0010100501000800,
        shift: 54,
        offset: 56320,
    },
    SMagic {
        mask: 0x0020205E20202000,
        magic: 0x004A400408012010,
        shift: 54,
        offset: 57344,
    },
    SMagic {
        mask: 0x0040403E40404000,
        magic: 0x0002104804000102,
        shift: 54,
        offset: 58368,
    },
    SMagic {
        mask: 0x0080807E80808000,
        magic: 0x4201004082000401,
        shift: 53,
        offset: 59392,
    },
    SMagic {
        mask: 0x00017E0101010100,
        magic: 0x9001C00080228004,
        shift: 53,
        offset: 61440,
    },
    SMagic {
        mask: 0x00027C0202020200,
        magic: 0x00A0400100810028,
        shift: 54,
        offset: 63488,
    },
    SMagic {
        mask: 0x00047A0404040400,
        magic: 0x3000408200220010,
        shift: 54,
        offset: 64512,
    },
    SMagic {
        mask: 0x0008760808080800,
        magic: 0x1010001008008080,
        shift: 54,
        offset: 65536,
    },
    SMagic {
        mask: 0x00106E1010101000,
        magic: 0x80A1000800850010,
        shift: 54,
        offset: 66560,
    },
    SMagic {
        mask: 0x00205E2020202000,
        magic: 0x9800040002008080,
        shift: 54,
        offset: 67584,
    },
    SMagic {
        mask: 0x00403E4040404000,
        magic: 0x0044500801040082,
        shift: 54,
        offset: 68608,
    },
    SMagic {
        mask: 0x00807E8080808000,
        magic: 0x0000009841020004,
        shift: 53,
        offset: 69632,
    },
    SMagic {
        mask: 0x007E010101010100,
        magic: 0x8C04800040210300,
        shift: 53,
        offset: 71680,
    },
    SMagic {
        mask: 0x007C020202020200,
        magic: 0x2030004008200440,
        shift: 54,
        offset: 73728,
    },
    SMagic {
        mask: 0x007A040404040400,
        magic: 0x1081041420024100,
        shift: 54,
        offset: 74752,
    },
    SMagic {
        mask: 0x0076080808080800,
        magic: 0x00880A0020401200,
        shift: 54,
        offset: 75776,
    },
    SMagic {
        mask: 0x006E101010101000,
        magic: 0x0000800800240180,
        shift: 54,
        offset: 76800,
    },
    SMagic {
        mask: 0x005E202020202000,
        magic: 0x1840020080040080,
        shift: 54,
        offset: 77824,
    },
    SMagic {
        mask: 0x003E404040404000,
        magic: 0x0011000200241100,
        shift: 54,
        offset: 78848,
    },
    SMagic {
        mask: 0x007E808080808000,
        magic: 0x00110000B2004100,
        shift: 53,
        offset: 79872,
    },
    SMagic {
        mask: 0x7E01010101010100,
        magic: 0x9080004100208011,
        shift: 52,
        offset: 81920,
    },
    SMagic {
        mask: 0x7C02020202020200,
        magic: 0x0482008021001042,
        shift: 53,
        offset: 86016,
    },
    SMagic {
        mask: 0x7A04040404040400,
        magic: 0x2020802008401202,
        shift: 53,
        offset: 88064,
    },
    SMagic {
        mask: 0x7608080808080800,
        magic: 0x000A090500201001,
        shift: 53,
        offset: 90112,
    },
    SMagic {
        mask: 0x6E10101010101000,
        magic: 0x0411009002080005,
        shift: 53,
        offset: 92160,
    },
    SMagic {
        mask: 0x5E20202020202000,
        magic: 0x8415000204000881,
        shift: 53,
        offset: 94208,
    },
    SMagic {
        mask: 0x3E40404040404000,
        magic: 0x0700115008880204,
        shift: 53,
        offset: 96256,
    },
    SMagic {
        mask: 0x7E80808080808000,
        magic: 0x5802005405002482,
        shift: 52,
        offset: 98304,
    },
];

pub const BISHOP_MAGICS: [SMagic; 64] = [
    SMagic {
        mask: 0x0040201008040200,
        magic: 0x2008100102040818,
        shift: 58,
        offset: 0,
    },
    SMagic {
        mask: 0x0000402010080400,
        magic: 0x00E001890A008880,
        shift: 59,
        offset: 64,
    },
    SMagic {
        mask: 0x0000004020100A00,
        magic: 0x8008080922202025,
        shift: 59,
        offset: 96,
    },
    SMagic {
        mask: 0x0000000040221400,
        magic: 0x2008208220000000,
        shift: 59,
        offset: 128,
    },
    SMagic {
        mask: 0x0000000002442800,
        magic: 0x086612100801010C,
        shift: 59,
        offset: 160,
    },
    SMagic {
        mask: 0x0000000204085000,
        magic: 0x6002028220200008,
        shift: 59,
        offset: 192,
    },
    SMagic {
        mask: 0x0000020408102000,
        magic: 0x2044044C02880400,
        shift: 59,
        offset: 224,
    },
    SMagic {
        mask: 0x0002040810204000,
        magic: 0x0448840158021808,
        shift: 58,
        offset: 256,
    },
    SMagic {
        mask: 0x0020100804020000,
        magic: 0x0100080801084210,
        shift: 59,
        offset: 320,
    },
    SMagic {
        mask: 0x0040201008040000,
        magic: 0x8002844102022200,
        shift: 59,
        offset: 352,
    },
    SMagic {
        mask: 0x00004020100A0000,
        magic: 0x203038229D020182,
        shift: 59,
        offset: 384,
    },
    SMagic {
        mask: 0x0000004022140000,
        magic: 0x0000090409004100,
        shift: 59,
        offset: 416,
    },
    SMagic {
        mask: 0x0000000244280000,
        magic: 0x0000242420042062,
        shift: 59,
        offset: 448,
    },
    SMagic {
        mask: 0x0000020408500000,
        magic: 0x8800020802080100,
        shift: 59,
        offset: 480,
    },
    SMagic {
        mask: 0x0002040810200000,
        magic: 0x0000011088200800,
        shift: 59,
        offset: 512,
    },
    SMagic {
        mask: 0x0004081020400000,
        magic: 0x20A5410101412020,
        shift: 59,
        offset: 544,
    },
    SMagic {
        mask: 0x0010080402000200,
        magic: 0x2008042282040820,
        shift: 59,
        offset: 576,
    },
    SMagic {
        mask: 0x0020100804000400,
        magic: 0x0230002022920844,
        shift: 59,
        offset: 608,
    },
    SMagic {
        mask: 0x004020100A000A00,
        magic: 0x0102009000204500,
        shift: 57,
        offset: 640,
    },
    SMagic {
        mask: 0x0000402214001400,
        magic: 0x8C08004422002000,
        shift: 57,
        offset: 768,
    },
    SMagic {
        mask: 0x0000024428002800,
        magic: 0x810C000080A04000,
        shift: 57,
        offset: 896,
    },
    SMagic {
        mask: 0x0002040850005000,
        magic: 0x0002010088012804,
        shift: 57,
        offset: 1024,
    },
    SMagic {
        mask: 0x0004081020002000,
        magic: 0x0804080244041510,
        shift: 59,
        offset: 1152,
    },
    SMagic {
        mask: 0x0008102040004000,
        magic: 0x04430020410801A0,
        shift: 59,
        offset: 1184,
    },
    SMagic {
        mask: 0x0008040200020400,
        magic: 0x2020040108283821,
        shift: 59,
        offset: 1216,
    },
    SMagic {
        mask: 0x0010080400040800,
        magic: 0x0C10300404B1460C,
        shift: 59,
        offset: 1248,
    },
    SMagic {
        mask: 0x0020100A000A1000,
        magic: 0x2808010088840100,
        shift: 57,
        offset: 1280,
    },
    SMagic {
        mask: 0x0040221400142200,
        magic: 0x4158080014220020,
        shift: 55,
        offset: 1408,
    },
    SMagic {
        mask: 0x0002442800284400,
        magic: 0x040900100B004000,
        shift: 55,
        offset: 1920,
    },
    SMagic {
        mask: 0x0004085000500800,
        magic: 0x8046018004482000,
        shift: 57,
        offset: 2432,
    },
    SMagic {
        mask: 0x0008102000201000,
        magic: 0x2408048801208800,
        shift: 59,
        offset: 2560,
    },
    SMagic {
        mask: 0x0010204000402000,
        magic: 0x0001010000A42700,
        shift: 59,
        offset: 2592,
    },
    SMagic {
        mask: 0x0004020002040800,
        magic: 0x00286008A0040800,
        shift: 59,
        offset: 2624,
    },
    SMagic {
        mask: 0x0008040004081000,
        magic: 0x4802022001100110,
        shift: 59,
        offset: 2656,
    },
    SMagic {
        mask: 0x00100A000A102000,
        magic: 0x2000220121C80800,
        shift: 57,
        offset: 2688,
    },
    SMagic {
        mask: 0x0022140014224000,
        magic: 0x0282010040040040,
        shift: 55,
        offset: 2816,
    },
    SMagic {
        mask: 0x0044280028440200,
        magic: 0x00D8020400441100,
        shift: 55,
        offset: 3328,
    },
    SMagic {
        mask: 0x0008500050080400,
        magic: 0x004A100208810080,
        shift: 57,
        offset: 3840,
    },
    SMagic {
        mask: 0x0010200020100800,
        magic: 0x1041310102040C00,
        shift: 59,
        offset: 3968,
    },
    SMagic {
        mask: 0x0020400040201000,
        magic: 0x0500812044060200,
        shift: 59,
        offset: 4000,
    },
    SMagic {
        mask: 0x0002000204081000,
        magic: 0x095082682000C008,
        shift: 59,
        offset: 4032,
    },
    SMagic {
        mask: 0x0004000408102000,
        magic: 0x0020820803292000,
        shift: 59,
        offset: 4064,
    },
    SMagic {
        mask: 0x000A000A10204000,
        magic: 0x0022050148001100,
        shift: 57,
        offset: 4096,
    },
    SMagic {
        mask: 0x0014001422400000,
        magic: 0x0008204200818800,
        shift: 57,
        offset: 4224,
    },
    SMagic {
        mask: 0x0028002844020000,
        magic: 0x05014402D2000C01,
        shift: 57,
        offset: 4352,
    },
    SMagic {
        mask: 0x0050005008040200,
        magic: 0x0260181010411020,
        shift: 57,
        offset: 4480,
    },
    SMagic {
        mask: 0x0020002010080400,
        magic: 0x2020010901000A18,
        shift: 59,
        offset: 4608,
    },
    SMagic {
        mask: 0x0040004020100800,
        magic: 0x1330089100540102,
        shift: 59,
        offset: 4640,
    },
    SMagic {
        mask: 0x0000020408102000,
        magic: 0x0440840422404014,
        shift: 59,
        offset: 4672,
    },
    SMagic {
        mask: 0x0000040810204000,
        magic: 0x1800220A06210120,
        shift: 59,
        offset: 4704,
    },
    SMagic {
        mask: 0x00000A1020400000,
        magic: 0x2002410101410400,
        shift: 59,
        offset: 4736,
    },
    SMagic {
        mask: 0x0000142240000000,
        magic: 0x0200040484040000,
        shift: 59,
        offset: 4768,
    },
    SMagic {
        mask: 0x0000284402000000,
        magic: 0xA000005022120200,
        shift: 59,
        offset: 4800,
    },
    SMagic {
        mask: 0x0000500804020000,
        magic: 0x0040401002808A00,
        shift: 59,
        offset: 4832,
    },
    SMagic {
        mask: 0x0000201008040200,
        magic: 0x0164180208220000,
        shift: 59,
        offset: 4864,
    },
    SMagic {
        mask: 0x0000402010080400,
        magic: 0x0105104401062000,
        shift: 59,
        offset: 4896,
    },
    SMagic {
        mask: 0x0002040810204000,
        magic: 0x0000441200822000,
        shift: 58,
        offset: 4928,
    },
    SMagic {
        mask: 0x0004081020400000,
        magic: 0x000001004804A420,
        shift: 59,
        offset: 4992,
    },
    SMagic {
        mask: 0x000A102040000000,
        magic: 0xA001000A04620810,
        shift: 59,
        offset: 5024,
    },
    SMagic {
        mask: 0x0014224000000000,
        magic: 0x3001110020208807,
        shift: 59,
        offset: 5056,
    },
    SMagic {
        mask: 0x0028440200000000,
        magic: 0x0A80000840350901,
        shift: 59,
        offset: 5088,
    },
    SMagic {
        mask: 0x0050080402000000,
        magic: 0x0004204004080084,
        shift: 59,
        offset: 5120,
    },
    SMagic {
        mask: 0x0020100804020000,
        magic: 0x0000C10401022200,
        shift: 59,
        offset: 5152,
    },
    SMagic {
        mask: 0x0040201008040200,
        magic: 0x2010901090818A00,
        shift: 58,
        offset: 5184,
    },
];