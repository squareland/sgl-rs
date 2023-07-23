use super::{GLenum, GLboolean, GLuint, GLuint64};

#[allow(dead_code, non_upper_case_globals)]
pub const ACCUM: GLenum = 0x0100;
#[allow(dead_code, non_upper_case_globals)]
pub const ACCUM_ALPHA_BITS: GLenum = 0x0D5B;
#[allow(dead_code, non_upper_case_globals)]
pub const ACCUM_BLUE_BITS: GLenum = 0x0D5A;
#[allow(dead_code, non_upper_case_globals)]
pub const ACCUM_BUFFER_BIT: GLenum = 0x00000200;
#[allow(dead_code, non_upper_case_globals)]
pub const ACCUM_CLEAR_VALUE: GLenum = 0x0B80;
#[allow(dead_code, non_upper_case_globals)]
pub const ACCUM_GREEN_BITS: GLenum = 0x0D59;
#[allow(dead_code, non_upper_case_globals)]
pub const ACCUM_RED_BITS: GLenum = 0x0D58;
#[allow(dead_code, non_upper_case_globals)]
pub const ACTIVE_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92D9;
#[allow(dead_code, non_upper_case_globals)]
pub const ACTIVE_ATTRIBUTES: GLenum = 0x8B89;
#[allow(dead_code, non_upper_case_globals)]
pub const ACTIVE_ATTRIBUTE_MAX_LENGTH: GLenum = 0x8B8A;
#[allow(dead_code, non_upper_case_globals)]
pub const ACTIVE_PROGRAM: GLenum = 0x8259;
#[allow(dead_code, non_upper_case_globals)]
pub const ACTIVE_RESOURCES: GLenum = 0x92F5;
#[allow(dead_code, non_upper_case_globals)]
pub const ACTIVE_SUBROUTINES: GLenum = 0x8DE5;
#[allow(dead_code, non_upper_case_globals)]
pub const ACTIVE_SUBROUTINE_MAX_LENGTH: GLenum = 0x8E48;
#[allow(dead_code, non_upper_case_globals)]
pub const ACTIVE_SUBROUTINE_UNIFORMS: GLenum = 0x8DE6;
#[allow(dead_code, non_upper_case_globals)]
pub const ACTIVE_SUBROUTINE_UNIFORM_LOCATIONS: GLenum = 0x8E47;
#[allow(dead_code, non_upper_case_globals)]
pub const ACTIVE_SUBROUTINE_UNIFORM_MAX_LENGTH: GLenum = 0x8E49;
#[allow(dead_code, non_upper_case_globals)]
pub const ACTIVE_TEXTURE: GLenum = 0x84E0;
#[allow(dead_code, non_upper_case_globals)]
pub const ACTIVE_UNIFORMS: GLenum = 0x8B86;
#[allow(dead_code, non_upper_case_globals)]
pub const ACTIVE_UNIFORM_BLOCKS: GLenum = 0x8A36;
#[allow(dead_code, non_upper_case_globals)]
pub const ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH: GLenum = 0x8A35;
#[allow(dead_code, non_upper_case_globals)]
pub const ACTIVE_UNIFORM_MAX_LENGTH: GLenum = 0x8B87;
#[allow(dead_code, non_upper_case_globals)]
pub const ACTIVE_VARIABLES: GLenum = 0x9305;
#[allow(dead_code, non_upper_case_globals)]
pub const ADD: GLenum = 0x0104;
#[allow(dead_code, non_upper_case_globals)]
pub const ADD_SIGNED: GLenum = 0x8574;
#[allow(dead_code, non_upper_case_globals)]
pub const ALIASED_LINE_WIDTH_RANGE: GLenum = 0x846E;
#[allow(dead_code, non_upper_case_globals)]
pub const ALIASED_POINT_SIZE_RANGE: GLenum = 0x846D;
#[allow(dead_code, non_upper_case_globals)]
pub const ALL_ATTRIB_BITS: GLenum = 0xFFFFFFFF;
#[allow(dead_code, non_upper_case_globals)]
pub const ALL_BARRIER_BITS: GLenum = 0xFFFFFFFF;
#[allow(dead_code, non_upper_case_globals)]
pub const ALL_SHADER_BITS: GLenum = 0xFFFFFFFF;
#[allow(dead_code, non_upper_case_globals)]
pub const ALPHA: GLenum = 0x1906;
#[allow(dead_code, non_upper_case_globals)]
pub const ALPHA12: GLenum = 0x803D;
#[allow(dead_code, non_upper_case_globals)]
pub const ALPHA16: GLenum = 0x803E;
#[allow(dead_code, non_upper_case_globals)]
pub const ALPHA4: GLenum = 0x803B;
#[allow(dead_code, non_upper_case_globals)]
pub const ALPHA8: GLenum = 0x803C;
#[allow(dead_code, non_upper_case_globals)]
pub const ALPHA_BIAS: GLenum = 0x0D1D;
#[allow(dead_code, non_upper_case_globals)]
pub const ALPHA_BITS: GLenum = 0x0D55;
#[allow(dead_code, non_upper_case_globals)]
pub const ALPHA_INTEGER: GLenum = 0x8D97;
#[allow(dead_code, non_upper_case_globals)]
pub const ALPHA_SCALE: GLenum = 0x0D1C;
#[allow(dead_code, non_upper_case_globals)]
pub const ALPHA_TEST: GLenum = 0x0BC0;
#[allow(dead_code, non_upper_case_globals)]
pub const ALPHA_TEST_FUNC: GLenum = 0x0BC1;
#[allow(dead_code, non_upper_case_globals)]
pub const ALPHA_TEST_REF: GLenum = 0x0BC2;
#[allow(dead_code, non_upper_case_globals)]
pub const ALREADY_SIGNALED: GLenum = 0x911A;
#[allow(dead_code, non_upper_case_globals)]
pub const ALWAYS: GLenum = 0x0207;
#[allow(dead_code, non_upper_case_globals)]
pub const AMBIENT: GLenum = 0x1200;
#[allow(dead_code, non_upper_case_globals)]
pub const AMBIENT_AND_DIFFUSE: GLenum = 0x1602;
#[allow(dead_code, non_upper_case_globals)]
pub const AND: GLenum = 0x1501;
#[allow(dead_code, non_upper_case_globals)]
pub const AND_INVERTED: GLenum = 0x1504;
#[allow(dead_code, non_upper_case_globals)]
pub const AND_REVERSE: GLenum = 0x1502;
#[allow(dead_code, non_upper_case_globals)]
pub const ANY_SAMPLES_PASSED: GLenum = 0x8C2F;
#[allow(dead_code, non_upper_case_globals)]
pub const ANY_SAMPLES_PASSED_CONSERVATIVE: GLenum = 0x8D6A;
#[allow(dead_code, non_upper_case_globals)]
pub const ARRAY_BUFFER: GLenum = 0x8892;
#[allow(dead_code, non_upper_case_globals)]
pub const ARRAY_BUFFER_BINDING: GLenum = 0x8894;
#[allow(dead_code, non_upper_case_globals)]
pub const ARRAY_SIZE: GLenum = 0x92FB;
#[allow(dead_code, non_upper_case_globals)]
pub const ARRAY_STRIDE: GLenum = 0x92FE;
#[allow(dead_code, non_upper_case_globals)]
pub const ATOMIC_COUNTER_BARRIER_BIT: GLenum = 0x00001000;
#[allow(dead_code, non_upper_case_globals)]
pub const ATOMIC_COUNTER_BUFFER: GLenum = 0x92C0;
#[allow(dead_code, non_upper_case_globals)]
pub const ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTERS: GLenum = 0x92C5;
#[allow(dead_code, non_upper_case_globals)]
pub const ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTER_INDICES: GLenum = 0x92C6;
#[allow(dead_code, non_upper_case_globals)]
pub const ATOMIC_COUNTER_BUFFER_BINDING: GLenum = 0x92C1;
#[allow(dead_code, non_upper_case_globals)]
pub const ATOMIC_COUNTER_BUFFER_DATA_SIZE: GLenum = 0x92C4;
#[allow(dead_code, non_upper_case_globals)]
pub const ATOMIC_COUNTER_BUFFER_INDEX: GLenum = 0x9301;
#[allow(dead_code, non_upper_case_globals)]
pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_COMPUTE_SHADER: GLenum = 0x90ED;
#[allow(dead_code, non_upper_case_globals)]
pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_FRAGMENT_SHADER: GLenum = 0x92CB;
#[allow(dead_code, non_upper_case_globals)]
pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_GEOMETRY_SHADER: GLenum = 0x92CA;
#[allow(dead_code, non_upper_case_globals)]
pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_CONTROL_SHADER: GLenum = 0x92C8;
#[allow(dead_code, non_upper_case_globals)]
pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_EVALUATION_SHADER: GLenum = 0x92C9;
#[allow(dead_code, non_upper_case_globals)]
pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_VERTEX_SHADER: GLenum = 0x92C7;
#[allow(dead_code, non_upper_case_globals)]
pub const ATOMIC_COUNTER_BUFFER_SIZE: GLenum = 0x92C3;
#[allow(dead_code, non_upper_case_globals)]
pub const ATOMIC_COUNTER_BUFFER_START: GLenum = 0x92C2;
#[allow(dead_code, non_upper_case_globals)]
pub const ATTACHED_SHADERS: GLenum = 0x8B85;
#[allow(dead_code, non_upper_case_globals)]
pub const ATTRIB_STACK_DEPTH: GLenum = 0x0BB0;
#[allow(dead_code, non_upper_case_globals)]
pub const AUTO_GENERATE_MIPMAP: GLenum = 0x8295;
#[allow(dead_code, non_upper_case_globals)]
pub const AUTO_NORMAL: GLenum = 0x0D80;
#[allow(dead_code, non_upper_case_globals)]
pub const AUX0: GLenum = 0x0409;
#[allow(dead_code, non_upper_case_globals)]
pub const AUX1: GLenum = 0x040A;
#[allow(dead_code, non_upper_case_globals)]
pub const AUX2: GLenum = 0x040B;
#[allow(dead_code, non_upper_case_globals)]
pub const AUX3: GLenum = 0x040C;
#[allow(dead_code, non_upper_case_globals)]
pub const AUX_BUFFERS: GLenum = 0x0C00;
#[allow(dead_code, non_upper_case_globals)]
pub const BACK: GLenum = 0x0405;
#[allow(dead_code, non_upper_case_globals)]
pub const BACK_LEFT: GLenum = 0x0402;
#[allow(dead_code, non_upper_case_globals)]
pub const BACK_RIGHT: GLenum = 0x0403;
#[allow(dead_code, non_upper_case_globals)]
pub const BGR: GLenum = 0x80E0;
#[allow(dead_code, non_upper_case_globals)]
pub const BGRA: GLenum = 0x80E1;
#[allow(dead_code, non_upper_case_globals)]
pub const BGRA_INTEGER: GLenum = 0x8D9B;
#[allow(dead_code, non_upper_case_globals)]
pub const BGR_INTEGER: GLenum = 0x8D9A;
#[allow(dead_code, non_upper_case_globals)]
pub const BITMAP: GLenum = 0x1A00;
#[allow(dead_code, non_upper_case_globals)]
pub const BITMAP_TOKEN: GLenum = 0x0704;
#[allow(dead_code, non_upper_case_globals)]
pub const BLEND: GLenum = 0x0BE2;
#[allow(dead_code, non_upper_case_globals)]
pub const BLEND_COLOR: GLenum = 0x8005;
#[allow(dead_code, non_upper_case_globals)]
pub const BLEND_DST: GLenum = 0x0BE0;
#[allow(dead_code, non_upper_case_globals)]
pub const BLEND_DST_ALPHA: GLenum = 0x80CA;
#[allow(dead_code, non_upper_case_globals)]
pub const BLEND_DST_RGB: GLenum = 0x80C8;
#[allow(dead_code, non_upper_case_globals)]
pub const BLEND_EQUATION: GLenum = 0x8009;
#[allow(dead_code, non_upper_case_globals)]
pub const BLEND_EQUATION_ALPHA: GLenum = 0x883D;
#[allow(dead_code, non_upper_case_globals)]
pub const BLEND_EQUATION_RGB: GLenum = 0x8009;
#[allow(dead_code, non_upper_case_globals)]
pub const BLEND_SRC: GLenum = 0x0BE1;
#[allow(dead_code, non_upper_case_globals)]
pub const BLEND_SRC_ALPHA: GLenum = 0x80CB;
#[allow(dead_code, non_upper_case_globals)]
pub const BLEND_SRC_RGB: GLenum = 0x80C9;
#[allow(dead_code, non_upper_case_globals)]
pub const BLOCK_INDEX: GLenum = 0x92FD;
#[allow(dead_code, non_upper_case_globals)]
pub const BLUE: GLenum = 0x1905;
#[allow(dead_code, non_upper_case_globals)]
pub const BLUE_BIAS: GLenum = 0x0D1B;
#[allow(dead_code, non_upper_case_globals)]
pub const BLUE_BITS: GLenum = 0x0D54;
#[allow(dead_code, non_upper_case_globals)]
pub const BLUE_INTEGER: GLenum = 0x8D96;
#[allow(dead_code, non_upper_case_globals)]
pub const BLUE_SCALE: GLenum = 0x0D1A;
#[allow(dead_code, non_upper_case_globals)]
pub const BOOL: GLenum = 0x8B56;
#[allow(dead_code, non_upper_case_globals)]
pub const BOOL_VEC2: GLenum = 0x8B57;
#[allow(dead_code, non_upper_case_globals)]
pub const BOOL_VEC3: GLenum = 0x8B58;
#[allow(dead_code, non_upper_case_globals)]
pub const BOOL_VEC4: GLenum = 0x8B59;
#[allow(dead_code, non_upper_case_globals)]
pub const BUFFER: GLenum = 0x82E0;
#[allow(dead_code, non_upper_case_globals)]
pub const BUFFER_ACCESS: GLenum = 0x88BB;
#[allow(dead_code, non_upper_case_globals)]
pub const BUFFER_ACCESS_FLAGS: GLenum = 0x911F;
#[allow(dead_code, non_upper_case_globals)]
pub const BUFFER_BINDING: GLenum = 0x9302;
#[allow(dead_code, non_upper_case_globals)]
pub const BUFFER_DATA_SIZE: GLenum = 0x9303;
#[allow(dead_code, non_upper_case_globals)]
pub const BUFFER_IMMUTABLE_STORAGE: GLenum = 0x821F;
#[allow(dead_code, non_upper_case_globals)]
pub const BUFFER_MAPPED: GLenum = 0x88BC;
#[allow(dead_code, non_upper_case_globals)]
pub const BUFFER_MAP_LENGTH: GLenum = 0x9120;
#[allow(dead_code, non_upper_case_globals)]
pub const BUFFER_MAP_OFFSET: GLenum = 0x9121;
#[allow(dead_code, non_upper_case_globals)]
pub const BUFFER_MAP_POINTER: GLenum = 0x88BD;
#[allow(dead_code, non_upper_case_globals)]
pub const BUFFER_SIZE: GLenum = 0x8764;
#[allow(dead_code, non_upper_case_globals)]
pub const BUFFER_STORAGE_FLAGS: GLenum = 0x8220;
#[allow(dead_code, non_upper_case_globals)]
pub const BUFFER_UPDATE_BARRIER_BIT: GLenum = 0x00000200;
#[allow(dead_code, non_upper_case_globals)]
pub const BUFFER_USAGE: GLenum = 0x8765;
#[allow(dead_code, non_upper_case_globals)]
pub const BUFFER_VARIABLE: GLenum = 0x92E5;
#[allow(dead_code, non_upper_case_globals)]
pub const BYTE: GLenum = 0x1400;
#[allow(dead_code, non_upper_case_globals)]
pub const C3F_V3F: GLenum = 0x2A24;
#[allow(dead_code, non_upper_case_globals)]
pub const C4F_N3F_V3F: GLenum = 0x2A26;
#[allow(dead_code, non_upper_case_globals)]
pub const C4UB_V2F: GLenum = 0x2A22;
#[allow(dead_code, non_upper_case_globals)]
pub const C4UB_V3F: GLenum = 0x2A23;
#[allow(dead_code, non_upper_case_globals)]
pub const CAVEAT_SUPPORT: GLenum = 0x82B8;
#[allow(dead_code, non_upper_case_globals)]
pub const CCW: GLenum = 0x0901;
#[allow(dead_code, non_upper_case_globals)]
pub const CLAMP: GLenum = 0x2900;
#[allow(dead_code, non_upper_case_globals)]
pub const CLAMP_FRAGMENT_COLOR: GLenum = 0x891B;
#[allow(dead_code, non_upper_case_globals)]
pub const CLAMP_READ_COLOR: GLenum = 0x891C;
#[allow(dead_code, non_upper_case_globals)]
pub const CLAMP_TO_BORDER: GLenum = 0x812D;
#[allow(dead_code, non_upper_case_globals)]
pub const CLAMP_TO_EDGE: GLenum = 0x812F;
#[allow(dead_code, non_upper_case_globals)]
pub const CLAMP_VERTEX_COLOR: GLenum = 0x891A;
#[allow(dead_code, non_upper_case_globals)]
pub const CLEAR: GLenum = 0x1500;
#[allow(dead_code, non_upper_case_globals)]
pub const CLEAR_BUFFER: GLenum = 0x82B4;
#[allow(dead_code, non_upper_case_globals)]
pub const CLEAR_TEXTURE: GLenum = 0x9365;
#[allow(dead_code, non_upper_case_globals)]
pub const CLIENT_ACTIVE_TEXTURE: GLenum = 0x84E1;
#[allow(dead_code, non_upper_case_globals)]
pub const CLIENT_ALL_ATTRIB_BITS: GLenum = 0xFFFFFFFF;
#[allow(dead_code, non_upper_case_globals)]
pub const CLIENT_ATTRIB_STACK_DEPTH: GLenum = 0x0BB1;
#[allow(dead_code, non_upper_case_globals)]
pub const CLIENT_MAPPED_BUFFER_BARRIER_BIT: GLenum = 0x00004000;
#[allow(dead_code, non_upper_case_globals)]
pub const CLIENT_PIXEL_STORE_BIT: GLenum = 0x00000001;
#[allow(dead_code, non_upper_case_globals)]
pub const CLIENT_STORAGE_BIT: GLenum = 0x0200;
#[allow(dead_code, non_upper_case_globals)]
pub const CLIENT_VERTEX_ARRAY_BIT: GLenum = 0x00000002;
#[allow(dead_code, non_upper_case_globals)]
pub const CLIP_DEPTH_MODE: GLenum = 0x935D;
#[allow(dead_code, non_upper_case_globals)]
pub const CLIP_DISTANCE0: GLenum = 0x3000;
#[allow(dead_code, non_upper_case_globals)]
pub const CLIP_DISTANCE1: GLenum = 0x3001;
#[allow(dead_code, non_upper_case_globals)]
pub const CLIP_DISTANCE2: GLenum = 0x3002;
#[allow(dead_code, non_upper_case_globals)]
pub const CLIP_DISTANCE3: GLenum = 0x3003;
#[allow(dead_code, non_upper_case_globals)]
pub const CLIP_DISTANCE4: GLenum = 0x3004;
#[allow(dead_code, non_upper_case_globals)]
pub const CLIP_DISTANCE5: GLenum = 0x3005;
#[allow(dead_code, non_upper_case_globals)]
pub const CLIP_DISTANCE6: GLenum = 0x3006;
#[allow(dead_code, non_upper_case_globals)]
pub const CLIP_DISTANCE7: GLenum = 0x3007;
#[allow(dead_code, non_upper_case_globals)]
pub const CLIP_ORIGIN: GLenum = 0x935C;
#[allow(dead_code, non_upper_case_globals)]
pub const CLIP_PLANE0: GLenum = 0x3000;
#[allow(dead_code, non_upper_case_globals)]
pub const CLIP_PLANE1: GLenum = 0x3001;
#[allow(dead_code, non_upper_case_globals)]
pub const CLIP_PLANE2: GLenum = 0x3002;
#[allow(dead_code, non_upper_case_globals)]
pub const CLIP_PLANE3: GLenum = 0x3003;
#[allow(dead_code, non_upper_case_globals)]
pub const CLIP_PLANE4: GLenum = 0x3004;
#[allow(dead_code, non_upper_case_globals)]
pub const CLIP_PLANE5: GLenum = 0x3005;
#[allow(dead_code, non_upper_case_globals)]
pub const COEFF: GLenum = 0x0A00;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR: GLenum = 0x1800;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ARRAY: GLenum = 0x8076;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ARRAY_BUFFER_BINDING: GLenum = 0x8898;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ARRAY_POINTER: GLenum = 0x8090;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ARRAY_SIZE: GLenum = 0x8081;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ARRAY_STRIDE: GLenum = 0x8083;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ARRAY_TYPE: GLenum = 0x8082;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT0: GLenum = 0x8CE0;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT1: GLenum = 0x8CE1;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT10: GLenum = 0x8CEA;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT11: GLenum = 0x8CEB;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT12: GLenum = 0x8CEC;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT13: GLenum = 0x8CED;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT14: GLenum = 0x8CEE;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT15: GLenum = 0x8CEF;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT16: GLenum = 0x8CF0;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT17: GLenum = 0x8CF1;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT18: GLenum = 0x8CF2;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT19: GLenum = 0x8CF3;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT2: GLenum = 0x8CE2;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT20: GLenum = 0x8CF4;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT21: GLenum = 0x8CF5;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT22: GLenum = 0x8CF6;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT23: GLenum = 0x8CF7;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT24: GLenum = 0x8CF8;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT25: GLenum = 0x8CF9;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT26: GLenum = 0x8CFA;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT27: GLenum = 0x8CFB;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT28: GLenum = 0x8CFC;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT29: GLenum = 0x8CFD;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT3: GLenum = 0x8CE3;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT30: GLenum = 0x8CFE;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT31: GLenum = 0x8CFF;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT4: GLenum = 0x8CE4;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT5: GLenum = 0x8CE5;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT6: GLenum = 0x8CE6;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT7: GLenum = 0x8CE7;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT8: GLenum = 0x8CE8;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ATTACHMENT9: GLenum = 0x8CE9;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_BUFFER_BIT: GLenum = 0x00004000;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_CLEAR_VALUE: GLenum = 0x0C22;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_COMPONENTS: GLenum = 0x8283;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_ENCODING: GLenum = 0x8296;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_INDEX: GLenum = 0x1900;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_INDEXES: GLenum = 0x1603;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_LOGIC_OP: GLenum = 0x0BF2;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_MATERIAL: GLenum = 0x0B57;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_MATERIAL_FACE: GLenum = 0x0B55;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_MATERIAL_PARAMETER: GLenum = 0x0B56;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_RENDERABLE: GLenum = 0x8286;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_SUM: GLenum = 0x8458;
#[allow(dead_code, non_upper_case_globals)]
pub const COLOR_WRITEMASK: GLenum = 0x0C23;
#[allow(dead_code, non_upper_case_globals)]
pub const COMBINE: GLenum = 0x8570;
#[allow(dead_code, non_upper_case_globals)]
pub const COMBINE_ALPHA: GLenum = 0x8572;
#[allow(dead_code, non_upper_case_globals)]
pub const COMBINE_RGB: GLenum = 0x8571;
#[allow(dead_code, non_upper_case_globals)]
pub const COMMAND_BARRIER_BIT: GLenum = 0x00000040;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPARE_REF_TO_TEXTURE: GLenum = 0x884E;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPARE_R_TO_TEXTURE: GLenum = 0x884E;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPATIBLE_SUBROUTINES: GLenum = 0x8E4B;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPILE: GLenum = 0x1300;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPILE_AND_EXECUTE: GLenum = 0x1301;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPILE_STATUS: GLenum = 0x8B81;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPRESSED_ALPHA: GLenum = 0x84E9;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPRESSED_INTENSITY: GLenum = 0x84EC;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPRESSED_LUMINANCE: GLenum = 0x84EA;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPRESSED_LUMINANCE_ALPHA: GLenum = 0x84EB;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPRESSED_R11_EAC: GLenum = 0x9270;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPRESSED_RED: GLenum = 0x8225;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPRESSED_RED_RGTC1: GLenum = 0x8DBB;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPRESSED_RG: GLenum = 0x8226;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPRESSED_RG11_EAC: GLenum = 0x9272;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPRESSED_RGB: GLenum = 0x84ED;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPRESSED_RGB8_ETC2: GLenum = 0x9274;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2: GLenum = 0x9276;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPRESSED_RGBA: GLenum = 0x84EE;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPRESSED_RGBA8_ETC2_EAC: GLenum = 0x9278;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPRESSED_RGBA_BPTC_UNORM: GLenum = 0x8E8C;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPRESSED_RGB_BPTC_SIGNED_FLOAT: GLenum = 0x8E8E;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPRESSED_RGB_BPTC_UNSIGNED_FLOAT: GLenum = 0x8E8F;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPRESSED_RG_RGTC2: GLenum = 0x8DBD;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPRESSED_SIGNED_R11_EAC: GLenum = 0x9271;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPRESSED_SIGNED_RED_RGTC1: GLenum = 0x8DBC;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPRESSED_SIGNED_RG11_EAC: GLenum = 0x9273;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPRESSED_SIGNED_RG_RGTC2: GLenum = 0x8DBE;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPRESSED_SLUMINANCE: GLenum = 0x8C4A;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPRESSED_SLUMINANCE_ALPHA: GLenum = 0x8C4B;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPRESSED_SRGB: GLenum = 0x8C48;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPRESSED_SRGB8_ALPHA8_ETC2_EAC: GLenum = 0x9279;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPRESSED_SRGB8_ETC2: GLenum = 0x9275;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2: GLenum = 0x9277;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPRESSED_SRGB_ALPHA: GLenum = 0x8C49;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPRESSED_SRGB_ALPHA_BPTC_UNORM: GLenum = 0x8E8D;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPRESSED_TEXTURE_FORMATS: GLenum = 0x86A3;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPUTE_SHADER: GLenum = 0x91B9;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPUTE_SHADER_BIT: GLenum = 0x00000020;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPUTE_SUBROUTINE: GLenum = 0x92ED;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPUTE_SUBROUTINE_UNIFORM: GLenum = 0x92F3;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPUTE_TEXTURE: GLenum = 0x82A0;
#[allow(dead_code, non_upper_case_globals)]
pub const COMPUTE_WORK_GROUP_SIZE: GLenum = 0x8267;
#[allow(dead_code, non_upper_case_globals)]
pub const CONDITION_SATISFIED: GLenum = 0x911C;
#[allow(dead_code, non_upper_case_globals)]
pub const CONSTANT: GLenum = 0x8576;
#[allow(dead_code, non_upper_case_globals)]
pub const CONSTANT_ALPHA: GLenum = 0x8003;
#[allow(dead_code, non_upper_case_globals)]
pub const CONSTANT_ATTENUATION: GLenum = 0x1207;
#[allow(dead_code, non_upper_case_globals)]
pub const CONSTANT_COLOR: GLenum = 0x8001;
#[allow(dead_code, non_upper_case_globals)]
pub const CONTEXT_COMPATIBILITY_PROFILE_BIT: GLenum = 0x00000002;
#[allow(dead_code, non_upper_case_globals)]
pub const CONTEXT_CORE_PROFILE_BIT: GLenum = 0x00000001;
#[allow(dead_code, non_upper_case_globals)]
pub const CONTEXT_FLAGS: GLenum = 0x821E;
#[allow(dead_code, non_upper_case_globals)]
pub const CONTEXT_FLAG_DEBUG_BIT: GLenum = 0x00000002;
#[allow(dead_code, non_upper_case_globals)]
pub const CONTEXT_FLAG_FORWARD_COMPATIBLE_BIT: GLenum = 0x00000001;
#[allow(dead_code, non_upper_case_globals)]
pub const CONTEXT_FLAG_ROBUST_ACCESS_BIT: GLenum = 0x00000004;
#[allow(dead_code, non_upper_case_globals)]
pub const CONTEXT_LOST: GLenum = 0x0507;
#[allow(dead_code, non_upper_case_globals)]
pub const CONTEXT_PROFILE_MASK: GLenum = 0x9126;
#[allow(dead_code, non_upper_case_globals)]
pub const CONTEXT_RELEASE_BEHAVIOR: GLenum = 0x82FB;
#[allow(dead_code, non_upper_case_globals)]
pub const CONTEXT_RELEASE_BEHAVIOR_FLUSH: GLenum = 0x82FC;
#[allow(dead_code, non_upper_case_globals)]
pub const COORD_REPLACE: GLenum = 0x8862;
#[allow(dead_code, non_upper_case_globals)]
pub const COPY: GLenum = 0x1503;
#[allow(dead_code, non_upper_case_globals)]
pub const COPY_INVERTED: GLenum = 0x150C;
#[allow(dead_code, non_upper_case_globals)]
pub const COPY_PIXEL_TOKEN: GLenum = 0x0706;
#[allow(dead_code, non_upper_case_globals)]
pub const COPY_READ_BUFFER: GLenum = 0x8F36;
#[allow(dead_code, non_upper_case_globals)]
pub const COPY_READ_BUFFER_BINDING: GLenum = 0x8F36;
#[allow(dead_code, non_upper_case_globals)]
pub const COPY_WRITE_BUFFER: GLenum = 0x8F37;
#[allow(dead_code, non_upper_case_globals)]
pub const COPY_WRITE_BUFFER_BINDING: GLenum = 0x8F37;
#[allow(dead_code, non_upper_case_globals)]
pub const CULL_FACE: GLenum = 0x0B44;
#[allow(dead_code, non_upper_case_globals)]
pub const CULL_FACE_MODE: GLenum = 0x0B45;
#[allow(dead_code, non_upper_case_globals)]
pub const CURRENT_BIT: GLenum = 0x00000001;
#[allow(dead_code, non_upper_case_globals)]
pub const CURRENT_COLOR: GLenum = 0x0B00;
#[allow(dead_code, non_upper_case_globals)]
pub const CURRENT_FOG_COORD: GLenum = 0x8453;
#[allow(dead_code, non_upper_case_globals)]
pub const CURRENT_FOG_COORDINATE: GLenum = 0x8453;
#[allow(dead_code, non_upper_case_globals)]
pub const CURRENT_INDEX: GLenum = 0x0B01;
#[allow(dead_code, non_upper_case_globals)]
pub const CURRENT_NORMAL: GLenum = 0x0B02;
#[allow(dead_code, non_upper_case_globals)]
pub const CURRENT_PROGRAM: GLenum = 0x8B8D;
#[allow(dead_code, non_upper_case_globals)]
pub const CURRENT_QUERY: GLenum = 0x8865;
#[allow(dead_code, non_upper_case_globals)]
pub const CURRENT_RASTER_COLOR: GLenum = 0x0B04;
#[allow(dead_code, non_upper_case_globals)]
pub const CURRENT_RASTER_DISTANCE: GLenum = 0x0B09;
#[allow(dead_code, non_upper_case_globals)]
pub const CURRENT_RASTER_INDEX: GLenum = 0x0B05;
#[allow(dead_code, non_upper_case_globals)]
pub const CURRENT_RASTER_POSITION: GLenum = 0x0B07;
#[allow(dead_code, non_upper_case_globals)]
pub const CURRENT_RASTER_POSITION_VALID: GLenum = 0x0B08;
#[allow(dead_code, non_upper_case_globals)]
pub const CURRENT_RASTER_SECONDARY_COLOR: GLenum = 0x845F;
#[allow(dead_code, non_upper_case_globals)]
pub const CURRENT_RASTER_TEXTURE_COORDS: GLenum = 0x0B06;
#[allow(dead_code, non_upper_case_globals)]
pub const CURRENT_SECONDARY_COLOR: GLenum = 0x8459;
#[allow(dead_code, non_upper_case_globals)]
pub const CURRENT_TEXTURE_COORDS: GLenum = 0x0B03;
#[allow(dead_code, non_upper_case_globals)]
pub const CURRENT_VERTEX_ATTRIB: GLenum = 0x8626;
#[allow(dead_code, non_upper_case_globals)]
pub const CW: GLenum = 0x0900;
#[allow(dead_code, non_upper_case_globals)]
pub const DEBUG_CALLBACK_FUNCTION: GLenum = 0x8244;
#[allow(dead_code, non_upper_case_globals)]
pub const DEBUG_CALLBACK_USER_PARAM: GLenum = 0x8245;
#[allow(dead_code, non_upper_case_globals)]
pub const DEBUG_GROUP_STACK_DEPTH: GLenum = 0x826D;
#[allow(dead_code, non_upper_case_globals)]
pub const DEBUG_LOGGED_MESSAGES: GLenum = 0x9145;
#[allow(dead_code, non_upper_case_globals)]
pub const DEBUG_NEXT_LOGGED_MESSAGE_LENGTH: GLenum = 0x8243;
#[allow(dead_code, non_upper_case_globals)]
pub const DEBUG_OUTPUT: GLenum = 0x92E0;
#[allow(dead_code, non_upper_case_globals)]
pub const DEBUG_OUTPUT_SYNCHRONOUS: GLenum = 0x8242;
#[allow(dead_code, non_upper_case_globals)]
pub const DEBUG_SEVERITY_HIGH: GLenum = 0x9146;
#[allow(dead_code, non_upper_case_globals)]
pub const DEBUG_SEVERITY_LOW: GLenum = 0x9148;
#[allow(dead_code, non_upper_case_globals)]
pub const DEBUG_SEVERITY_MEDIUM: GLenum = 0x9147;
#[allow(dead_code, non_upper_case_globals)]
pub const DEBUG_SEVERITY_NOTIFICATION: GLenum = 0x826B;
#[allow(dead_code, non_upper_case_globals)]
pub const DEBUG_SOURCE_API: GLenum = 0x8246;
#[allow(dead_code, non_upper_case_globals)]
pub const DEBUG_SOURCE_APPLICATION: GLenum = 0x824A;
#[allow(dead_code, non_upper_case_globals)]
pub const DEBUG_SOURCE_OTHER: GLenum = 0x824B;
#[allow(dead_code, non_upper_case_globals)]
pub const DEBUG_SOURCE_SHADER_COMPILER: GLenum = 0x8248;
#[allow(dead_code, non_upper_case_globals)]
pub const DEBUG_SOURCE_THIRD_PARTY: GLenum = 0x8249;
#[allow(dead_code, non_upper_case_globals)]
pub const DEBUG_SOURCE_WINDOW_SYSTEM: GLenum = 0x8247;
#[allow(dead_code, non_upper_case_globals)]
pub const DEBUG_TYPE_DEPRECATED_BEHAVIOR: GLenum = 0x824D;
#[allow(dead_code, non_upper_case_globals)]
pub const DEBUG_TYPE_ERROR: GLenum = 0x824C;
#[allow(dead_code, non_upper_case_globals)]
pub const DEBUG_TYPE_MARKER: GLenum = 0x8268;
#[allow(dead_code, non_upper_case_globals)]
pub const DEBUG_TYPE_OTHER: GLenum = 0x8251;
#[allow(dead_code, non_upper_case_globals)]
pub const DEBUG_TYPE_PERFORMANCE: GLenum = 0x8250;
#[allow(dead_code, non_upper_case_globals)]
pub const DEBUG_TYPE_POP_GROUP: GLenum = 0x826A;
#[allow(dead_code, non_upper_case_globals)]
pub const DEBUG_TYPE_PORTABILITY: GLenum = 0x824F;
#[allow(dead_code, non_upper_case_globals)]
pub const DEBUG_TYPE_PUSH_GROUP: GLenum = 0x8269;
#[allow(dead_code, non_upper_case_globals)]
pub const DEBUG_TYPE_UNDEFINED_BEHAVIOR: GLenum = 0x824E;
#[allow(dead_code, non_upper_case_globals)]
pub const DECAL: GLenum = 0x2101;
#[allow(dead_code, non_upper_case_globals)]
pub const DECR: GLenum = 0x1E03;
#[allow(dead_code, non_upper_case_globals)]
pub const DECR_WRAP: GLenum = 0x8508;
#[allow(dead_code, non_upper_case_globals)]
pub const DELETE_STATUS: GLenum = 0x8B80;
#[allow(dead_code, non_upper_case_globals)]
pub const DEPTH: GLenum = 0x1801;
#[allow(dead_code, non_upper_case_globals)]
pub const DEPTH24_STENCIL8: GLenum = 0x88F0;
#[allow(dead_code, non_upper_case_globals)]
pub const DEPTH32F_STENCIL8: GLenum = 0x8CAD;
#[allow(dead_code, non_upper_case_globals)]
pub const DEPTH_ATTACHMENT: GLenum = 0x8D00;
#[allow(dead_code, non_upper_case_globals)]
pub const DEPTH_BIAS: GLenum = 0x0D1F;
#[allow(dead_code, non_upper_case_globals)]
pub const DEPTH_BITS: GLenum = 0x0D56;
#[allow(dead_code, non_upper_case_globals)]
pub const DEPTH_BUFFER_BIT: GLenum = 0x00000100;
#[allow(dead_code, non_upper_case_globals)]
pub const DEPTH_CLAMP: GLenum = 0x864F;
#[allow(dead_code, non_upper_case_globals)]
pub const DEPTH_CLEAR_VALUE: GLenum = 0x0B73;
#[allow(dead_code, non_upper_case_globals)]
pub const DEPTH_COMPONENT: GLenum = 0x1902;
#[allow(dead_code, non_upper_case_globals)]
pub const DEPTH_COMPONENT16: GLenum = 0x81A5;
#[allow(dead_code, non_upper_case_globals)]
pub const DEPTH_COMPONENT24: GLenum = 0x81A6;
#[allow(dead_code, non_upper_case_globals)]
pub const DEPTH_COMPONENT32: GLenum = 0x81A7;
#[allow(dead_code, non_upper_case_globals)]
pub const DEPTH_COMPONENT32F: GLenum = 0x8CAC;
#[allow(dead_code, non_upper_case_globals)]
pub const DEPTH_COMPONENTS: GLenum = 0x8284;
#[allow(dead_code, non_upper_case_globals)]
pub const DEPTH_FUNC: GLenum = 0x0B74;
#[allow(dead_code, non_upper_case_globals)]
pub const DEPTH_RANGE: GLenum = 0x0B70;
#[allow(dead_code, non_upper_case_globals)]
pub const DEPTH_RENDERABLE: GLenum = 0x8287;
#[allow(dead_code, non_upper_case_globals)]
pub const DEPTH_SCALE: GLenum = 0x0D1E;
#[allow(dead_code, non_upper_case_globals)]
pub const DEPTH_STENCIL: GLenum = 0x84F9;
#[allow(dead_code, non_upper_case_globals)]
pub const DEPTH_STENCIL_ATTACHMENT: GLenum = 0x821A;
#[allow(dead_code, non_upper_case_globals)]
pub const DEPTH_STENCIL_TEXTURE_MODE: GLenum = 0x90EA;
#[allow(dead_code, non_upper_case_globals)]
pub const DEPTH_TEST: GLenum = 0x0B71;
#[allow(dead_code, non_upper_case_globals)]
pub const DEPTH_TEXTURE_MODE: GLenum = 0x884B;
#[allow(dead_code, non_upper_case_globals)]
pub const DEPTH_WRITEMASK: GLenum = 0x0B72;
#[allow(dead_code, non_upper_case_globals)]
pub const DIFFUSE: GLenum = 0x1201;
#[allow(dead_code, non_upper_case_globals)]
pub const DISPATCH_INDIRECT_BUFFER: GLenum = 0x90EE;
#[allow(dead_code, non_upper_case_globals)]
pub const DISPATCH_INDIRECT_BUFFER_BINDING: GLenum = 0x90EF;
#[allow(dead_code, non_upper_case_globals)]
pub const DISPLAY_LIST: GLenum = 0x82E7;
#[allow(dead_code, non_upper_case_globals)]
pub const DITHER: GLenum = 0x0BD0;
#[allow(dead_code, non_upper_case_globals)]
pub const DOMAIN: GLenum = 0x0A02;
#[allow(dead_code, non_upper_case_globals)]
pub const DONT_CARE: GLenum = 0x1100;
#[allow(dead_code, non_upper_case_globals)]
pub const DOT3_RGB: GLenum = 0x86AE;
#[allow(dead_code, non_upper_case_globals)]
pub const DOT3_RGBA: GLenum = 0x86AF;
#[allow(dead_code, non_upper_case_globals)]
pub const DOUBLE: GLenum = 0x140A;
#[allow(dead_code, non_upper_case_globals)]
pub const DOUBLEBUFFER: GLenum = 0x0C32;
#[allow(dead_code, non_upper_case_globals)]
pub const DOUBLE_MAT2: GLenum = 0x8F46;
#[allow(dead_code, non_upper_case_globals)]
pub const DOUBLE_MAT2x3: GLenum = 0x8F49;
#[allow(dead_code, non_upper_case_globals)]
pub const DOUBLE_MAT2x4: GLenum = 0x8F4A;
#[allow(dead_code, non_upper_case_globals)]
pub const DOUBLE_MAT3: GLenum = 0x8F47;
#[allow(dead_code, non_upper_case_globals)]
pub const DOUBLE_MAT3x2: GLenum = 0x8F4B;
#[allow(dead_code, non_upper_case_globals)]
pub const DOUBLE_MAT3x4: GLenum = 0x8F4C;
#[allow(dead_code, non_upper_case_globals)]
pub const DOUBLE_MAT4: GLenum = 0x8F48;
#[allow(dead_code, non_upper_case_globals)]
pub const DOUBLE_MAT4x2: GLenum = 0x8F4D;
#[allow(dead_code, non_upper_case_globals)]
pub const DOUBLE_MAT4x3: GLenum = 0x8F4E;
#[allow(dead_code, non_upper_case_globals)]
pub const DOUBLE_VEC2: GLenum = 0x8FFC;
#[allow(dead_code, non_upper_case_globals)]
pub const DOUBLE_VEC3: GLenum = 0x8FFD;
#[allow(dead_code, non_upper_case_globals)]
pub const DOUBLE_VEC4: GLenum = 0x8FFE;
#[allow(dead_code, non_upper_case_globals)]
pub const DRAW_BUFFER: GLenum = 0x0C01;
#[allow(dead_code, non_upper_case_globals)]
pub const DRAW_BUFFER0: GLenum = 0x8825;
#[allow(dead_code, non_upper_case_globals)]
pub const DRAW_BUFFER1: GLenum = 0x8826;
#[allow(dead_code, non_upper_case_globals)]
pub const DRAW_BUFFER10: GLenum = 0x882F;
#[allow(dead_code, non_upper_case_globals)]
pub const DRAW_BUFFER11: GLenum = 0x8830;
#[allow(dead_code, non_upper_case_globals)]
pub const DRAW_BUFFER12: GLenum = 0x8831;
#[allow(dead_code, non_upper_case_globals)]
pub const DRAW_BUFFER13: GLenum = 0x8832;
#[allow(dead_code, non_upper_case_globals)]
pub const DRAW_BUFFER14: GLenum = 0x8833;
#[allow(dead_code, non_upper_case_globals)]
pub const DRAW_BUFFER15: GLenum = 0x8834;
#[allow(dead_code, non_upper_case_globals)]
pub const DRAW_BUFFER2: GLenum = 0x8827;
#[allow(dead_code, non_upper_case_globals)]
pub const DRAW_BUFFER3: GLenum = 0x8828;
#[allow(dead_code, non_upper_case_globals)]
pub const DRAW_BUFFER4: GLenum = 0x8829;
#[allow(dead_code, non_upper_case_globals)]
pub const DRAW_BUFFER5: GLenum = 0x882A;
#[allow(dead_code, non_upper_case_globals)]
pub const DRAW_BUFFER6: GLenum = 0x882B;
#[allow(dead_code, non_upper_case_globals)]
pub const DRAW_BUFFER7: GLenum = 0x882C;
#[allow(dead_code, non_upper_case_globals)]
pub const DRAW_BUFFER8: GLenum = 0x882D;
#[allow(dead_code, non_upper_case_globals)]
pub const DRAW_BUFFER9: GLenum = 0x882E;
#[allow(dead_code, non_upper_case_globals)]
pub const DRAW_FRAMEBUFFER: GLenum = 0x8CA9;
#[allow(dead_code, non_upper_case_globals)]
pub const DRAW_FRAMEBUFFER_BINDING: GLenum = 0x8CA6;
#[allow(dead_code, non_upper_case_globals)]
pub const DRAW_INDIRECT_BUFFER: GLenum = 0x8F3F;
#[allow(dead_code, non_upper_case_globals)]
pub const DRAW_INDIRECT_BUFFER_BINDING: GLenum = 0x8F43;
#[allow(dead_code, non_upper_case_globals)]
pub const DRAW_PIXEL_TOKEN: GLenum = 0x0705;
#[allow(dead_code, non_upper_case_globals)]
pub const DST_ALPHA: GLenum = 0x0304;
#[allow(dead_code, non_upper_case_globals)]
pub const DST_COLOR: GLenum = 0x0306;
#[allow(dead_code, non_upper_case_globals)]
pub const DYNAMIC_COPY: GLenum = 0x88EA;
#[allow(dead_code, non_upper_case_globals)]
pub const DYNAMIC_DRAW: GLenum = 0x88E8;
#[allow(dead_code, non_upper_case_globals)]
pub const DYNAMIC_READ: GLenum = 0x88E9;
#[allow(dead_code, non_upper_case_globals)]
pub const DYNAMIC_STORAGE_BIT: GLenum = 0x0100;
#[allow(dead_code, non_upper_case_globals)]
pub const EDGE_FLAG: GLenum = 0x0B43;
#[allow(dead_code, non_upper_case_globals)]
pub const EDGE_FLAG_ARRAY: GLenum = 0x8079;
#[allow(dead_code, non_upper_case_globals)]
pub const EDGE_FLAG_ARRAY_BUFFER_BINDING: GLenum = 0x889B;
#[allow(dead_code, non_upper_case_globals)]
pub const EDGE_FLAG_ARRAY_POINTER: GLenum = 0x8093;
#[allow(dead_code, non_upper_case_globals)]
pub const EDGE_FLAG_ARRAY_STRIDE: GLenum = 0x808C;
#[allow(dead_code, non_upper_case_globals)]
pub const ELEMENT_ARRAY_BARRIER_BIT: GLenum = 0x00000002;
#[allow(dead_code, non_upper_case_globals)]
pub const ELEMENT_ARRAY_BUFFER: GLenum = 0x8893;
#[allow(dead_code, non_upper_case_globals)]
pub const ELEMENT_ARRAY_BUFFER_BINDING: GLenum = 0x8895;
#[allow(dead_code, non_upper_case_globals)]
pub const EMISSION: GLenum = 0x1600;
#[allow(dead_code, non_upper_case_globals)]
pub const ENABLE_BIT: GLenum = 0x00002000;
#[allow(dead_code, non_upper_case_globals)]
pub const EQUAL: GLenum = 0x0202;
#[allow(dead_code, non_upper_case_globals)]
pub const EQUIV: GLenum = 0x1509;
#[allow(dead_code, non_upper_case_globals)]
pub const EVAL_BIT: GLenum = 0x00010000;
#[allow(dead_code, non_upper_case_globals)]
pub const EXP: GLenum = 0x0800;
#[allow(dead_code, non_upper_case_globals)]
pub const EXP2: GLenum = 0x0801;
#[allow(dead_code, non_upper_case_globals)]
pub const EXTENSIONS: GLenum = 0x1F03;
#[allow(dead_code, non_upper_case_globals)]
pub const EYE_LINEAR: GLenum = 0x2400;
#[allow(dead_code, non_upper_case_globals)]
pub const EYE_PLANE: GLenum = 0x2502;
#[allow(dead_code, non_upper_case_globals)]
pub const FALSE: GLboolean = 0;
#[allow(dead_code, non_upper_case_globals)]
pub const FASTEST: GLenum = 0x1101;
#[allow(dead_code, non_upper_case_globals)]
pub const FEEDBACK: GLenum = 0x1C01;
#[allow(dead_code, non_upper_case_globals)]
pub const FEEDBACK_BUFFER_POINTER: GLenum = 0x0DF0;
#[allow(dead_code, non_upper_case_globals)]
pub const FEEDBACK_BUFFER_SIZE: GLenum = 0x0DF1;
#[allow(dead_code, non_upper_case_globals)]
pub const FEEDBACK_BUFFER_TYPE: GLenum = 0x0DF2;
#[allow(dead_code, non_upper_case_globals)]
pub const FILL: GLenum = 0x1B02;
#[allow(dead_code, non_upper_case_globals)]
pub const FILTER: GLenum = 0x829A;
#[allow(dead_code, non_upper_case_globals)]
pub const FIRST_VERTEX_CONVENTION: GLenum = 0x8E4D;
#[allow(dead_code, non_upper_case_globals)]
pub const FIXED: GLenum = 0x140C;
#[allow(dead_code, non_upper_case_globals)]
pub const FIXED_ONLY: GLenum = 0x891D;
#[allow(dead_code, non_upper_case_globals)]
pub const FLAT: GLenum = 0x1D00;
#[allow(dead_code, non_upper_case_globals)]
pub const FLOAT: GLenum = 0x1406;
#[allow(dead_code, non_upper_case_globals)]
pub const FLOAT_32_UNSIGNED_INT_24_8_REV: GLenum = 0x8DAD;
#[allow(dead_code, non_upper_case_globals)]
pub const FLOAT_MAT2: GLenum = 0x8B5A;
#[allow(dead_code, non_upper_case_globals)]
pub const FLOAT_MAT2x3: GLenum = 0x8B65;
#[allow(dead_code, non_upper_case_globals)]
pub const FLOAT_MAT2x4: GLenum = 0x8B66;
#[allow(dead_code, non_upper_case_globals)]
pub const FLOAT_MAT3: GLenum = 0x8B5B;
#[allow(dead_code, non_upper_case_globals)]
pub const FLOAT_MAT3x2: GLenum = 0x8B67;
#[allow(dead_code, non_upper_case_globals)]
pub const FLOAT_MAT3x4: GLenum = 0x8B68;
#[allow(dead_code, non_upper_case_globals)]
pub const FLOAT_MAT4: GLenum = 0x8B5C;
#[allow(dead_code, non_upper_case_globals)]
pub const FLOAT_MAT4x2: GLenum = 0x8B69;
#[allow(dead_code, non_upper_case_globals)]
pub const FLOAT_MAT4x3: GLenum = 0x8B6A;
#[allow(dead_code, non_upper_case_globals)]
pub const FLOAT_VEC2: GLenum = 0x8B50;
#[allow(dead_code, non_upper_case_globals)]
pub const FLOAT_VEC3: GLenum = 0x8B51;
#[allow(dead_code, non_upper_case_globals)]
pub const FLOAT_VEC4: GLenum = 0x8B52;
#[allow(dead_code, non_upper_case_globals)]
pub const FOG: GLenum = 0x0B60;
#[allow(dead_code, non_upper_case_globals)]
pub const FOG_BIT: GLenum = 0x00000080;
#[allow(dead_code, non_upper_case_globals)]
pub const FOG_COLOR: GLenum = 0x0B66;
#[allow(dead_code, non_upper_case_globals)]
pub const FOG_COORD: GLenum = 0x8451;
#[allow(dead_code, non_upper_case_globals)]
pub const FOG_COORDINATE: GLenum = 0x8451;
#[allow(dead_code, non_upper_case_globals)]
pub const FOG_COORDINATE_ARRAY: GLenum = 0x8457;
#[allow(dead_code, non_upper_case_globals)]
pub const FOG_COORDINATE_ARRAY_BUFFER_BINDING: GLenum = 0x889D;
#[allow(dead_code, non_upper_case_globals)]
pub const FOG_COORDINATE_ARRAY_POINTER: GLenum = 0x8456;
#[allow(dead_code, non_upper_case_globals)]
pub const FOG_COORDINATE_ARRAY_STRIDE: GLenum = 0x8455;
#[allow(dead_code, non_upper_case_globals)]
pub const FOG_COORDINATE_ARRAY_TYPE: GLenum = 0x8454;
#[allow(dead_code, non_upper_case_globals)]
pub const FOG_COORDINATE_SOURCE: GLenum = 0x8450;
#[allow(dead_code, non_upper_case_globals)]
pub const FOG_COORD_ARRAY: GLenum = 0x8457;
#[allow(dead_code, non_upper_case_globals)]
pub const FOG_COORD_ARRAY_BUFFER_BINDING: GLenum = 0x889D;
#[allow(dead_code, non_upper_case_globals)]
pub const FOG_COORD_ARRAY_POINTER: GLenum = 0x8456;
#[allow(dead_code, non_upper_case_globals)]
pub const FOG_COORD_ARRAY_STRIDE: GLenum = 0x8455;
#[allow(dead_code, non_upper_case_globals)]
pub const FOG_COORD_ARRAY_TYPE: GLenum = 0x8454;
#[allow(dead_code, non_upper_case_globals)]
pub const FOG_COORD_SRC: GLenum = 0x8450;
#[allow(dead_code, non_upper_case_globals)]
pub const FOG_DENSITY: GLenum = 0x0B62;
#[allow(dead_code, non_upper_case_globals)]
pub const FOG_END: GLenum = 0x0B64;
#[allow(dead_code, non_upper_case_globals)]
pub const FOG_HINT: GLenum = 0x0C54;
#[allow(dead_code, non_upper_case_globals)]
pub const FOG_INDEX: GLenum = 0x0B61;
#[allow(dead_code, non_upper_case_globals)]
pub const FOG_MODE: GLenum = 0x0B65;
#[allow(dead_code, non_upper_case_globals)]
pub const FOG_START: GLenum = 0x0B63;
#[allow(dead_code, non_upper_case_globals)]
pub const FRACTIONAL_EVEN: GLenum = 0x8E7C;
#[allow(dead_code, non_upper_case_globals)]
pub const FRACTIONAL_ODD: GLenum = 0x8E7B;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAGMENT_DEPTH: GLenum = 0x8452;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAGMENT_INTERPOLATION_OFFSET_BITS: GLenum = 0x8E5D;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAGMENT_SHADER: GLenum = 0x8B30;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAGMENT_SHADER_BIT: GLenum = 0x00000002;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAGMENT_SHADER_DERIVATIVE_HINT: GLenum = 0x8B8B;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAGMENT_SUBROUTINE: GLenum = 0x92EC;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAGMENT_SUBROUTINE_UNIFORM: GLenum = 0x92F2;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAGMENT_TEXTURE: GLenum = 0x829F;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER: GLenum = 0x8D40;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE: GLenum = 0x8215;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_ATTACHMENT_BLUE_SIZE: GLenum = 0x8214;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING: GLenum = 0x8210;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE: GLenum = 0x8211;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE: GLenum = 0x8216;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_ATTACHMENT_GREEN_SIZE: GLenum = 0x8213;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_ATTACHMENT_LAYERED: GLenum = 0x8DA7;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_ATTACHMENT_OBJECT_NAME: GLenum = 0x8CD1;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE: GLenum = 0x8CD0;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_ATTACHMENT_RED_SIZE: GLenum = 0x8212;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE: GLenum = 0x8217;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: GLenum = 0x8CD3;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER: GLenum = 0x8CD4;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL: GLenum = 0x8CD2;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_BARRIER_BIT: GLenum = 0x00000400;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_BINDING: GLenum = 0x8CA6;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_BLEND: GLenum = 0x828B;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_COMPLETE: GLenum = 0x8CD5;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_DEFAULT: GLenum = 0x8218;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_DEFAULT_FIXED_SAMPLE_LOCATIONS: GLenum = 0x9314;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_DEFAULT_HEIGHT: GLenum = 0x9311;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_DEFAULT_LAYERS: GLenum = 0x9312;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_DEFAULT_SAMPLES: GLenum = 0x9313;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_DEFAULT_WIDTH: GLenum = 0x9310;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_INCOMPLETE_ATTACHMENT: GLenum = 0x8CD6;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER: GLenum = 0x8CDB;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS: GLenum = 0x8DA8;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: GLenum = 0x8CD7;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_INCOMPLETE_MULTISAMPLE: GLenum = 0x8D56;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_INCOMPLETE_READ_BUFFER: GLenum = 0x8CDC;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_RENDERABLE: GLenum = 0x8289;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_RENDERABLE_LAYERED: GLenum = 0x828A;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_SRGB: GLenum = 0x8DB9;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_UNDEFINED: GLenum = 0x8219;
#[allow(dead_code, non_upper_case_globals)]
pub const FRAMEBUFFER_UNSUPPORTED: GLenum = 0x8CDD;
#[allow(dead_code, non_upper_case_globals)]
pub const FRONT: GLenum = 0x0404;
#[allow(dead_code, non_upper_case_globals)]
pub const FRONT_AND_BACK: GLenum = 0x0408;
#[allow(dead_code, non_upper_case_globals)]
pub const FRONT_FACE: GLenum = 0x0B46;
#[allow(dead_code, non_upper_case_globals)]
pub const FRONT_LEFT: GLenum = 0x0400;
#[allow(dead_code, non_upper_case_globals)]
pub const FRONT_RIGHT: GLenum = 0x0401;
#[allow(dead_code, non_upper_case_globals)]
pub const FULL_SUPPORT: GLenum = 0x82B7;
#[allow(dead_code, non_upper_case_globals)]
pub const FUNC_ADD: GLenum = 0x8006;
#[allow(dead_code, non_upper_case_globals)]
pub const FUNC_REVERSE_SUBTRACT: GLenum = 0x800B;
#[allow(dead_code, non_upper_case_globals)]
pub const FUNC_SUBTRACT: GLenum = 0x800A;
#[allow(dead_code, non_upper_case_globals)]
pub const GENERATE_MIPMAP: GLenum = 0x8191;
#[allow(dead_code, non_upper_case_globals)]
pub const GENERATE_MIPMAP_HINT: GLenum = 0x8192;
#[allow(dead_code, non_upper_case_globals)]
pub const GEOMETRY_INPUT_TYPE: GLenum = 0x8917;
#[allow(dead_code, non_upper_case_globals)]
pub const GEOMETRY_OUTPUT_TYPE: GLenum = 0x8918;
#[allow(dead_code, non_upper_case_globals)]
pub const GEOMETRY_SHADER: GLenum = 0x8DD9;
#[allow(dead_code, non_upper_case_globals)]
pub const GEOMETRY_SHADER_BIT: GLenum = 0x00000004;
#[allow(dead_code, non_upper_case_globals)]
pub const GEOMETRY_SHADER_INVOCATIONS: GLenum = 0x887F;
#[allow(dead_code, non_upper_case_globals)]
pub const GEOMETRY_SUBROUTINE: GLenum = 0x92EB;
#[allow(dead_code, non_upper_case_globals)]
pub const GEOMETRY_SUBROUTINE_UNIFORM: GLenum = 0x92F1;
#[allow(dead_code, non_upper_case_globals)]
pub const GEOMETRY_TEXTURE: GLenum = 0x829E;
#[allow(dead_code, non_upper_case_globals)]
pub const GEOMETRY_VERTICES_OUT: GLenum = 0x8916;
#[allow(dead_code, non_upper_case_globals)]
pub const GEQUAL: GLenum = 0x0206;
#[allow(dead_code, non_upper_case_globals)]
pub const GET_TEXTURE_IMAGE_FORMAT: GLenum = 0x8291;
#[allow(dead_code, non_upper_case_globals)]
pub const GET_TEXTURE_IMAGE_TYPE: GLenum = 0x8292;
#[allow(dead_code, non_upper_case_globals)]
pub const GREATER: GLenum = 0x0204;
#[allow(dead_code, non_upper_case_globals)]
pub const GREEN: GLenum = 0x1904;
#[allow(dead_code, non_upper_case_globals)]
pub const GREEN_BIAS: GLenum = 0x0D19;
#[allow(dead_code, non_upper_case_globals)]
pub const GREEN_BITS: GLenum = 0x0D53;
#[allow(dead_code, non_upper_case_globals)]
pub const GREEN_INTEGER: GLenum = 0x8D95;
#[allow(dead_code, non_upper_case_globals)]
pub const GREEN_SCALE: GLenum = 0x0D18;
#[allow(dead_code, non_upper_case_globals)]
pub const GUILTY_CONTEXT_RESET: GLenum = 0x8253;
#[allow(dead_code, non_upper_case_globals)]
pub const HALF_FLOAT: GLenum = 0x140B;
#[allow(dead_code, non_upper_case_globals)]
pub const HIGH_FLOAT: GLenum = 0x8DF2;
#[allow(dead_code, non_upper_case_globals)]
pub const HIGH_INT: GLenum = 0x8DF5;
#[allow(dead_code, non_upper_case_globals)]
pub const HINT_BIT: GLenum = 0x00008000;
#[allow(dead_code, non_upper_case_globals)]
pub const IMAGE_1D: GLenum = 0x904C;
#[allow(dead_code, non_upper_case_globals)]
pub const IMAGE_1D_ARRAY: GLenum = 0x9052;
#[allow(dead_code, non_upper_case_globals)]
pub const IMAGE_2D: GLenum = 0x904D;
#[allow(dead_code, non_upper_case_globals)]
pub const IMAGE_2D_ARRAY: GLenum = 0x9053;
#[allow(dead_code, non_upper_case_globals)]
pub const IMAGE_2D_MULTISAMPLE: GLenum = 0x9055;
#[allow(dead_code, non_upper_case_globals)]
pub const IMAGE_2D_MULTISAMPLE_ARRAY: GLenum = 0x9056;
#[allow(dead_code, non_upper_case_globals)]
pub const IMAGE_2D_RECT: GLenum = 0x904F;
#[allow(dead_code, non_upper_case_globals)]
pub const IMAGE_3D: GLenum = 0x904E;
#[allow(dead_code, non_upper_case_globals)]
pub const IMAGE_BINDING_ACCESS: GLenum = 0x8F3E;
#[allow(dead_code, non_upper_case_globals)]
pub const IMAGE_BINDING_FORMAT: GLenum = 0x906E;
#[allow(dead_code, non_upper_case_globals)]
pub const IMAGE_BINDING_LAYER: GLenum = 0x8F3D;
#[allow(dead_code, non_upper_case_globals)]
pub const IMAGE_BINDING_LAYERED: GLenum = 0x8F3C;
#[allow(dead_code, non_upper_case_globals)]
pub const IMAGE_BINDING_LEVEL: GLenum = 0x8F3B;
#[allow(dead_code, non_upper_case_globals)]
pub const IMAGE_BINDING_NAME: GLenum = 0x8F3A;
#[allow(dead_code, non_upper_case_globals)]
pub const IMAGE_BUFFER: GLenum = 0x9051;
#[allow(dead_code, non_upper_case_globals)]
pub const IMAGE_CLASS_10_10_10_2: GLenum = 0x82C3;
#[allow(dead_code, non_upper_case_globals)]
pub const IMAGE_CLASS_11_11_10: GLenum = 0x82C2;
#[allow(dead_code, non_upper_case_globals)]
pub const IMAGE_CLASS_1_X_16: GLenum = 0x82BE;
#[allow(dead_code, non_upper_case_globals)]
pub const IMAGE_CLASS_1_X_32: GLenum = 0x82BB;
#[allow(dead_code, non_upper_case_globals)]
pub const IMAGE_CLASS_1_X_8: GLenum = 0x82C1;
#[allow(dead_code, non_upper_case_globals)]
pub const IMAGE_CLASS_2_X_16: GLenum = 0x82BD;
#[allow(dead_code, non_upper_case_globals)]
pub const IMAGE_CLASS_2_X_32: GLenum = 0x82BA;
#[allow(dead_code, non_upper_case_globals)]
pub const IMAGE_CLASS_2_X_8: GLenum = 0x82C0;
#[allow(dead_code, non_upper_case_globals)]
pub const IMAGE_CLASS_4_X_16: GLenum = 0x82BC;
#[allow(dead_code, non_upper_case_globals)]
pub const IMAGE_CLASS_4_X_32: GLenum = 0x82B9;
#[allow(dead_code, non_upper_case_globals)]
pub const IMAGE_CLASS_4_X_8: GLenum = 0x82BF;
#[allow(dead_code, non_upper_case_globals)]
pub const IMAGE_COMPATIBILITY_CLASS: GLenum = 0x82A8;
#[allow(dead_code, non_upper_case_globals)]
pub const IMAGE_CUBE: GLenum = 0x9050;
#[allow(dead_code, non_upper_case_globals)]
pub const IMAGE_CUBE_MAP_ARRAY: GLenum = 0x9054;
#[allow(dead_code, non_upper_case_globals)]
pub const IMAGE_FORMAT_COMPATIBILITY_BY_CLASS: GLenum = 0x90C9;
#[allow(dead_code, non_upper_case_globals)]
pub const IMAGE_FORMAT_COMPATIBILITY_BY_SIZE: GLenum = 0x90C8;
#[allow(dead_code, non_upper_case_globals)]
pub const IMAGE_FORMAT_COMPATIBILITY_TYPE: GLenum = 0x90C7;
#[allow(dead_code, non_upper_case_globals)]
pub const IMAGE_PIXEL_FORMAT: GLenum = 0x82A9;
#[allow(dead_code, non_upper_case_globals)]
pub const IMAGE_PIXEL_TYPE: GLenum = 0x82AA;
#[allow(dead_code, non_upper_case_globals)]
pub const IMAGE_TEXEL_SIZE: GLenum = 0x82A7;
#[allow(dead_code, non_upper_case_globals)]
pub const IMPLEMENTATION_COLOR_READ_FORMAT: GLenum = 0x8B9B;
#[allow(dead_code, non_upper_case_globals)]
pub const IMPLEMENTATION_COLOR_READ_TYPE: GLenum = 0x8B9A;
#[allow(dead_code, non_upper_case_globals)]
pub const INCR: GLenum = 0x1E02;
#[allow(dead_code, non_upper_case_globals)]
pub const INCR_WRAP: GLenum = 0x8507;
#[allow(dead_code, non_upper_case_globals)]
pub const INDEX: GLenum = 0x8222;
#[allow(dead_code, non_upper_case_globals)]
pub const INDEX_ARRAY: GLenum = 0x8077;
#[allow(dead_code, non_upper_case_globals)]
pub const INDEX_ARRAY_BUFFER_BINDING: GLenum = 0x8899;
#[allow(dead_code, non_upper_case_globals)]
pub const INDEX_ARRAY_POINTER: GLenum = 0x8091;
#[allow(dead_code, non_upper_case_globals)]
pub const INDEX_ARRAY_STRIDE: GLenum = 0x8086;
#[allow(dead_code, non_upper_case_globals)]
pub const INDEX_ARRAY_TYPE: GLenum = 0x8085;
#[allow(dead_code, non_upper_case_globals)]
pub const INDEX_BITS: GLenum = 0x0D51;
#[allow(dead_code, non_upper_case_globals)]
pub const INDEX_CLEAR_VALUE: GLenum = 0x0C20;
#[allow(dead_code, non_upper_case_globals)]
pub const INDEX_LOGIC_OP: GLenum = 0x0BF1;
#[allow(dead_code, non_upper_case_globals)]
pub const INDEX_MODE: GLenum = 0x0C30;
#[allow(dead_code, non_upper_case_globals)]
pub const INDEX_OFFSET: GLenum = 0x0D13;
#[allow(dead_code, non_upper_case_globals)]
pub const INDEX_SHIFT: GLenum = 0x0D12;
#[allow(dead_code, non_upper_case_globals)]
pub const INDEX_WRITEMASK: GLenum = 0x0C21;
#[allow(dead_code, non_upper_case_globals)]
pub const INFO_LOG_LENGTH: GLenum = 0x8B84;
#[allow(dead_code, non_upper_case_globals)]
pub const INNOCENT_CONTEXT_RESET: GLenum = 0x8254;
#[allow(dead_code, non_upper_case_globals)]
pub const INT: GLenum = 0x1404;
#[allow(dead_code, non_upper_case_globals)]
pub const INTENSITY: GLenum = 0x8049;
#[allow(dead_code, non_upper_case_globals)]
pub const INTENSITY12: GLenum = 0x804C;
#[allow(dead_code, non_upper_case_globals)]
pub const INTENSITY16: GLenum = 0x804D;
#[allow(dead_code, non_upper_case_globals)]
pub const INTENSITY4: GLenum = 0x804A;
#[allow(dead_code, non_upper_case_globals)]
pub const INTENSITY8: GLenum = 0x804B;
#[allow(dead_code, non_upper_case_globals)]
pub const INTERLEAVED_ATTRIBS: GLenum = 0x8C8C;
#[allow(dead_code, non_upper_case_globals)]
pub const INTERNALFORMAT_ALPHA_SIZE: GLenum = 0x8274;
#[allow(dead_code, non_upper_case_globals)]
pub const INTERNALFORMAT_ALPHA_TYPE: GLenum = 0x827B;
#[allow(dead_code, non_upper_case_globals)]
pub const INTERNALFORMAT_BLUE_SIZE: GLenum = 0x8273;
#[allow(dead_code, non_upper_case_globals)]
pub const INTERNALFORMAT_BLUE_TYPE: GLenum = 0x827A;
#[allow(dead_code, non_upper_case_globals)]
pub const INTERNALFORMAT_DEPTH_SIZE: GLenum = 0x8275;
#[allow(dead_code, non_upper_case_globals)]
pub const INTERNALFORMAT_DEPTH_TYPE: GLenum = 0x827C;
#[allow(dead_code, non_upper_case_globals)]
pub const INTERNALFORMAT_GREEN_SIZE: GLenum = 0x8272;
#[allow(dead_code, non_upper_case_globals)]
pub const INTERNALFORMAT_GREEN_TYPE: GLenum = 0x8279;
#[allow(dead_code, non_upper_case_globals)]
pub const INTERNALFORMAT_PREFERRED: GLenum = 0x8270;
#[allow(dead_code, non_upper_case_globals)]
pub const INTERNALFORMAT_RED_SIZE: GLenum = 0x8271;
#[allow(dead_code, non_upper_case_globals)]
pub const INTERNALFORMAT_RED_TYPE: GLenum = 0x8278;
#[allow(dead_code, non_upper_case_globals)]
pub const INTERNALFORMAT_SHARED_SIZE: GLenum = 0x8277;
#[allow(dead_code, non_upper_case_globals)]
pub const INTERNALFORMAT_STENCIL_SIZE: GLenum = 0x8276;
#[allow(dead_code, non_upper_case_globals)]
pub const INTERNALFORMAT_STENCIL_TYPE: GLenum = 0x827D;
#[allow(dead_code, non_upper_case_globals)]
pub const INTERNALFORMAT_SUPPORTED: GLenum = 0x826F;
#[allow(dead_code, non_upper_case_globals)]
pub const INTERPOLATE: GLenum = 0x8575;
#[allow(dead_code, non_upper_case_globals)]
pub const INT_2_10_10_10_REV: GLenum = 0x8D9F;
#[allow(dead_code, non_upper_case_globals)]
pub const INT_IMAGE_1D: GLenum = 0x9057;
#[allow(dead_code, non_upper_case_globals)]
pub const INT_IMAGE_1D_ARRAY: GLenum = 0x905D;
#[allow(dead_code, non_upper_case_globals)]
pub const INT_IMAGE_2D: GLenum = 0x9058;
#[allow(dead_code, non_upper_case_globals)]
pub const INT_IMAGE_2D_ARRAY: GLenum = 0x905E;
#[allow(dead_code, non_upper_case_globals)]
pub const INT_IMAGE_2D_MULTISAMPLE: GLenum = 0x9060;
#[allow(dead_code, non_upper_case_globals)]
pub const INT_IMAGE_2D_MULTISAMPLE_ARRAY: GLenum = 0x9061;
#[allow(dead_code, non_upper_case_globals)]
pub const INT_IMAGE_2D_RECT: GLenum = 0x905A;
#[allow(dead_code, non_upper_case_globals)]
pub const INT_IMAGE_3D: GLenum = 0x9059;
#[allow(dead_code, non_upper_case_globals)]
pub const INT_IMAGE_BUFFER: GLenum = 0x905C;
#[allow(dead_code, non_upper_case_globals)]
pub const INT_IMAGE_CUBE: GLenum = 0x905B;
#[allow(dead_code, non_upper_case_globals)]
pub const INT_IMAGE_CUBE_MAP_ARRAY: GLenum = 0x905F;
#[allow(dead_code, non_upper_case_globals)]
pub const INT_SAMPLER_1D: GLenum = 0x8DC9;
#[allow(dead_code, non_upper_case_globals)]
pub const INT_SAMPLER_1D_ARRAY: GLenum = 0x8DCE;
#[allow(dead_code, non_upper_case_globals)]
pub const INT_SAMPLER_2D: GLenum = 0x8DCA;
#[allow(dead_code, non_upper_case_globals)]
pub const INT_SAMPLER_2D_ARRAY: GLenum = 0x8DCF;
#[allow(dead_code, non_upper_case_globals)]
pub const INT_SAMPLER_2D_MULTISAMPLE: GLenum = 0x9109;
#[allow(dead_code, non_upper_case_globals)]
pub const INT_SAMPLER_2D_MULTISAMPLE_ARRAY: GLenum = 0x910C;
#[allow(dead_code, non_upper_case_globals)]
pub const INT_SAMPLER_2D_RECT: GLenum = 0x8DCD;
#[allow(dead_code, non_upper_case_globals)]
pub const INT_SAMPLER_3D: GLenum = 0x8DCB;
#[allow(dead_code, non_upper_case_globals)]
pub const INT_SAMPLER_BUFFER: GLenum = 0x8DD0;
#[allow(dead_code, non_upper_case_globals)]
pub const INT_SAMPLER_CUBE: GLenum = 0x8DCC;
#[allow(dead_code, non_upper_case_globals)]
pub const INT_SAMPLER_CUBE_MAP_ARRAY: GLenum = 0x900E;
#[allow(dead_code, non_upper_case_globals)]
pub const INT_VEC2: GLenum = 0x8B53;
#[allow(dead_code, non_upper_case_globals)]
pub const INT_VEC3: GLenum = 0x8B54;
#[allow(dead_code, non_upper_case_globals)]
pub const INT_VEC4: GLenum = 0x8B55;
#[allow(dead_code, non_upper_case_globals)]
pub const INVALID_ENUM: GLenum = 0x0500;
#[allow(dead_code, non_upper_case_globals)]
pub const INVALID_FRAMEBUFFER_OPERATION: GLenum = 0x0506;
#[allow(dead_code, non_upper_case_globals)]
pub const INVALID_INDEX: GLuint = 0xFFFFFFFF;
#[allow(dead_code, non_upper_case_globals)]
pub const INVALID_OPERATION: GLenum = 0x0502;
#[allow(dead_code, non_upper_case_globals)]
pub const INVALID_VALUE: GLenum = 0x0501;
#[allow(dead_code, non_upper_case_globals)]
pub const INVERT: GLenum = 0x150A;
#[allow(dead_code, non_upper_case_globals)]
pub const ISOLINES: GLenum = 0x8E7A;
#[allow(dead_code, non_upper_case_globals)]
pub const IS_PER_PATCH: GLenum = 0x92E7;
#[allow(dead_code, non_upper_case_globals)]
pub const IS_ROW_MAJOR: GLenum = 0x9300;
#[allow(dead_code, non_upper_case_globals)]
pub const KEEP: GLenum = 0x1E00;
#[allow(dead_code, non_upper_case_globals)]
pub const LAST_VERTEX_CONVENTION: GLenum = 0x8E4E;
#[allow(dead_code, non_upper_case_globals)]
pub const LAYER_PROVOKING_VERTEX: GLenum = 0x825E;
#[allow(dead_code, non_upper_case_globals)]
pub const LEFT: GLenum = 0x0406;
#[allow(dead_code, non_upper_case_globals)]
pub const LEQUAL: GLenum = 0x0203;
#[allow(dead_code, non_upper_case_globals)]
pub const LESS: GLenum = 0x0201;
#[allow(dead_code, non_upper_case_globals)]
pub const LIGHT0: GLenum = 0x4000;
#[allow(dead_code, non_upper_case_globals)]
pub const LIGHT1: GLenum = 0x4001;
#[allow(dead_code, non_upper_case_globals)]
pub const LIGHT2: GLenum = 0x4002;
#[allow(dead_code, non_upper_case_globals)]
pub const LIGHT3: GLenum = 0x4003;
#[allow(dead_code, non_upper_case_globals)]
pub const LIGHT4: GLenum = 0x4004;
#[allow(dead_code, non_upper_case_globals)]
pub const LIGHT5: GLenum = 0x4005;
#[allow(dead_code, non_upper_case_globals)]
pub const LIGHT6: GLenum = 0x4006;
#[allow(dead_code, non_upper_case_globals)]
pub const LIGHT7: GLenum = 0x4007;
#[allow(dead_code, non_upper_case_globals)]
pub const LIGHTING: GLenum = 0x0B50;
#[allow(dead_code, non_upper_case_globals)]
pub const LIGHTING_BIT: GLenum = 0x00000040;
#[allow(dead_code, non_upper_case_globals)]
pub const LIGHT_MODEL_AMBIENT: GLenum = 0x0B53;
#[allow(dead_code, non_upper_case_globals)]
pub const LIGHT_MODEL_COLOR_CONTROL: GLenum = 0x81F8;
#[allow(dead_code, non_upper_case_globals)]
pub const LIGHT_MODEL_LOCAL_VIEWER: GLenum = 0x0B51;
#[allow(dead_code, non_upper_case_globals)]
pub const LIGHT_MODEL_TWO_SIDE: GLenum = 0x0B52;
#[allow(dead_code, non_upper_case_globals)]
pub const LINE: GLenum = 0x1B01;
#[allow(dead_code, non_upper_case_globals)]
pub const LINEAR: GLenum = 0x2601;
#[allow(dead_code, non_upper_case_globals)]
pub const LINEAR_ATTENUATION: GLenum = 0x1208;
#[allow(dead_code, non_upper_case_globals)]
pub const LINEAR_MIPMAP_LINEAR: GLenum = 0x2703;
#[allow(dead_code, non_upper_case_globals)]
pub const LINEAR_MIPMAP_NEAREST: GLenum = 0x2701;
#[allow(dead_code, non_upper_case_globals)]
pub const LINES: GLenum = 0x0001;
#[allow(dead_code, non_upper_case_globals)]
pub const LINES_ADJACENCY: GLenum = 0x000A;
#[allow(dead_code, non_upper_case_globals)]
pub const LINE_BIT: GLenum = 0x00000004;
#[allow(dead_code, non_upper_case_globals)]
pub const LINE_LOOP: GLenum = 0x0002;
#[allow(dead_code, non_upper_case_globals)]
pub const LINE_RESET_TOKEN: GLenum = 0x0707;
#[allow(dead_code, non_upper_case_globals)]
pub const LINE_SMOOTH: GLenum = 0x0B20;
#[allow(dead_code, non_upper_case_globals)]
pub const LINE_SMOOTH_HINT: GLenum = 0x0C52;
#[allow(dead_code, non_upper_case_globals)]
pub const LINE_STIPPLE: GLenum = 0x0B24;
#[allow(dead_code, non_upper_case_globals)]
pub const LINE_STIPPLE_PATTERN: GLenum = 0x0B25;
#[allow(dead_code, non_upper_case_globals)]
pub const LINE_STIPPLE_REPEAT: GLenum = 0x0B26;
#[allow(dead_code, non_upper_case_globals)]
pub const LINE_STRIP: GLenum = 0x0003;
#[allow(dead_code, non_upper_case_globals)]
pub const LINE_STRIP_ADJACENCY: GLenum = 0x000B;
#[allow(dead_code, non_upper_case_globals)]
pub const LINE_TOKEN: GLenum = 0x0702;
#[allow(dead_code, non_upper_case_globals)]
pub const LINE_WIDTH: GLenum = 0x0B21;
#[allow(dead_code, non_upper_case_globals)]
pub const LINE_WIDTH_GRANULARITY: GLenum = 0x0B23;
#[allow(dead_code, non_upper_case_globals)]
pub const LINE_WIDTH_RANGE: GLenum = 0x0B22;
#[allow(dead_code, non_upper_case_globals)]
pub const LINK_STATUS: GLenum = 0x8B82;
#[allow(dead_code, non_upper_case_globals)]
pub const LIST_BASE: GLenum = 0x0B32;
#[allow(dead_code, non_upper_case_globals)]
pub const LIST_BIT: GLenum = 0x00020000;
#[allow(dead_code, non_upper_case_globals)]
pub const LIST_INDEX: GLenum = 0x0B33;
#[allow(dead_code, non_upper_case_globals)]
pub const LIST_MODE: GLenum = 0x0B30;
#[allow(dead_code, non_upper_case_globals)]
pub const LOAD: GLenum = 0x0101;
#[allow(dead_code, non_upper_case_globals)]
pub const LOCATION: GLenum = 0x930E;
#[allow(dead_code, non_upper_case_globals)]
pub const LOCATION_COMPONENT: GLenum = 0x934A;
#[allow(dead_code, non_upper_case_globals)]
pub const LOCATION_INDEX: GLenum = 0x930F;
#[allow(dead_code, non_upper_case_globals)]
pub const LOGIC_OP: GLenum = 0x0BF1;
#[allow(dead_code, non_upper_case_globals)]
pub const LOGIC_OP_MODE: GLenum = 0x0BF0;
#[allow(dead_code, non_upper_case_globals)]
pub const LOSE_CONTEXT_ON_RESET: GLenum = 0x8252;
#[allow(dead_code, non_upper_case_globals)]
pub const LOWER_LEFT: GLenum = 0x8CA1;
#[allow(dead_code, non_upper_case_globals)]
pub const LOW_FLOAT: GLenum = 0x8DF0;
#[allow(dead_code, non_upper_case_globals)]
pub const LOW_INT: GLenum = 0x8DF3;
#[allow(dead_code, non_upper_case_globals)]
pub const LUMINANCE: GLenum = 0x1909;
#[allow(dead_code, non_upper_case_globals)]
pub const LUMINANCE12: GLenum = 0x8041;
#[allow(dead_code, non_upper_case_globals)]
pub const LUMINANCE12_ALPHA12: GLenum = 0x8047;
#[allow(dead_code, non_upper_case_globals)]
pub const LUMINANCE12_ALPHA4: GLenum = 0x8046;
#[allow(dead_code, non_upper_case_globals)]
pub const LUMINANCE16: GLenum = 0x8042;
#[allow(dead_code, non_upper_case_globals)]
pub const LUMINANCE16_ALPHA16: GLenum = 0x8048;
#[allow(dead_code, non_upper_case_globals)]
pub const LUMINANCE4: GLenum = 0x803F;
#[allow(dead_code, non_upper_case_globals)]
pub const LUMINANCE4_ALPHA4: GLenum = 0x8043;
#[allow(dead_code, non_upper_case_globals)]
pub const LUMINANCE6_ALPHA2: GLenum = 0x8044;
#[allow(dead_code, non_upper_case_globals)]
pub const LUMINANCE8: GLenum = 0x8040;
#[allow(dead_code, non_upper_case_globals)]
pub const LUMINANCE8_ALPHA8: GLenum = 0x8045;
#[allow(dead_code, non_upper_case_globals)]
pub const LUMINANCE_ALPHA: GLenum = 0x190A;
#[allow(dead_code, non_upper_case_globals)]
pub const MAJOR_VERSION: GLenum = 0x821B;
#[allow(dead_code, non_upper_case_globals)]
pub const MANUAL_GENERATE_MIPMAP: GLenum = 0x8294;
#[allow(dead_code, non_upper_case_globals)]
pub const MAP1_COLOR_4: GLenum = 0x0D90;
#[allow(dead_code, non_upper_case_globals)]
pub const MAP1_GRID_DOMAIN: GLenum = 0x0DD0;
#[allow(dead_code, non_upper_case_globals)]
pub const MAP1_GRID_SEGMENTS: GLenum = 0x0DD1;
#[allow(dead_code, non_upper_case_globals)]
pub const MAP1_INDEX: GLenum = 0x0D91;
#[allow(dead_code, non_upper_case_globals)]
pub const MAP1_NORMAL: GLenum = 0x0D92;
#[allow(dead_code, non_upper_case_globals)]
pub const MAP1_TEXTURE_COORD_1: GLenum = 0x0D93;
#[allow(dead_code, non_upper_case_globals)]
pub const MAP1_TEXTURE_COORD_2: GLenum = 0x0D94;
#[allow(dead_code, non_upper_case_globals)]
pub const MAP1_TEXTURE_COORD_3: GLenum = 0x0D95;
#[allow(dead_code, non_upper_case_globals)]
pub const MAP1_TEXTURE_COORD_4: GLenum = 0x0D96;
#[allow(dead_code, non_upper_case_globals)]
pub const MAP1_VERTEX_3: GLenum = 0x0D97;
#[allow(dead_code, non_upper_case_globals)]
pub const MAP1_VERTEX_4: GLenum = 0x0D98;
#[allow(dead_code, non_upper_case_globals)]
pub const MAP2_COLOR_4: GLenum = 0x0DB0;
#[allow(dead_code, non_upper_case_globals)]
pub const MAP2_GRID_DOMAIN: GLenum = 0x0DD2;
#[allow(dead_code, non_upper_case_globals)]
pub const MAP2_GRID_SEGMENTS: GLenum = 0x0DD3;
#[allow(dead_code, non_upper_case_globals)]
pub const MAP2_INDEX: GLenum = 0x0DB1;
#[allow(dead_code, non_upper_case_globals)]
pub const MAP2_NORMAL: GLenum = 0x0DB2;
#[allow(dead_code, non_upper_case_globals)]
pub const MAP2_TEXTURE_COORD_1: GLenum = 0x0DB3;
#[allow(dead_code, non_upper_case_globals)]
pub const MAP2_TEXTURE_COORD_2: GLenum = 0x0DB4;
#[allow(dead_code, non_upper_case_globals)]
pub const MAP2_TEXTURE_COORD_3: GLenum = 0x0DB5;
#[allow(dead_code, non_upper_case_globals)]
pub const MAP2_TEXTURE_COORD_4: GLenum = 0x0DB6;
#[allow(dead_code, non_upper_case_globals)]
pub const MAP2_VERTEX_3: GLenum = 0x0DB7;
#[allow(dead_code, non_upper_case_globals)]
pub const MAP2_VERTEX_4: GLenum = 0x0DB8;
#[allow(dead_code, non_upper_case_globals)]
pub const MAP_COHERENT_BIT: GLenum = 0x0080;
#[allow(dead_code, non_upper_case_globals)]
pub const MAP_COLOR: GLenum = 0x0D10;
#[allow(dead_code, non_upper_case_globals)]
pub const MAP_FLUSH_EXPLICIT_BIT: GLenum = 0x0010;
#[allow(dead_code, non_upper_case_globals)]
pub const MAP_INVALIDATE_BUFFER_BIT: GLenum = 0x0008;
#[allow(dead_code, non_upper_case_globals)]
pub const MAP_INVALIDATE_RANGE_BIT: GLenum = 0x0004;
#[allow(dead_code, non_upper_case_globals)]
pub const MAP_PERSISTENT_BIT: GLenum = 0x0040;
#[allow(dead_code, non_upper_case_globals)]
pub const MAP_READ_BIT: GLenum = 0x0001;
#[allow(dead_code, non_upper_case_globals)]
pub const MAP_STENCIL: GLenum = 0x0D11;
#[allow(dead_code, non_upper_case_globals)]
pub const MAP_UNSYNCHRONIZED_BIT: GLenum = 0x0020;
#[allow(dead_code, non_upper_case_globals)]
pub const MAP_WRITE_BIT: GLenum = 0x0002;
#[allow(dead_code, non_upper_case_globals)]
pub const MATRIX_MODE: GLenum = 0x0BA0;
#[allow(dead_code, non_upper_case_globals)]
pub const MATRIX_STRIDE: GLenum = 0x92FF;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX: GLenum = 0x8008;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_3D_TEXTURE_SIZE: GLenum = 0x8073;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_ARRAY_TEXTURE_LAYERS: GLenum = 0x88FF;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_ATOMIC_COUNTER_BUFFER_BINDINGS: GLenum = 0x92DC;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_ATOMIC_COUNTER_BUFFER_SIZE: GLenum = 0x92D8;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_ATTRIB_STACK_DEPTH: GLenum = 0x0D35;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_CLIENT_ATTRIB_STACK_DEPTH: GLenum = 0x0D3B;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_CLIP_DISTANCES: GLenum = 0x0D32;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_CLIP_PLANES: GLenum = 0x0D32;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_COLOR_ATTACHMENTS: GLenum = 0x8CDF;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_COLOR_TEXTURE_SAMPLES: GLenum = 0x910E;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_COMBINED_ATOMIC_COUNTERS: GLenum = 0x92D7;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_COMBINED_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92D1;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_COMBINED_CLIP_AND_CULL_DISTANCES: GLenum = 0x82FA;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_COMBINED_COMPUTE_UNIFORM_COMPONENTS: GLenum = 0x8266;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_COMBINED_DIMENSIONS: GLenum = 0x8282;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS: GLenum = 0x8A33;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_COMBINED_GEOMETRY_UNIFORM_COMPONENTS: GLenum = 0x8A32;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_COMBINED_IMAGE_UNIFORMS: GLenum = 0x90CF;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_COMBINED_IMAGE_UNITS_AND_FRAGMENT_OUTPUTS: GLenum = 0x8F39;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_COMBINED_SHADER_OUTPUT_RESOURCES: GLenum = 0x8F39;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_COMBINED_SHADER_STORAGE_BLOCKS: GLenum = 0x90DC;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_COMBINED_TESS_CONTROL_UNIFORM_COMPONENTS: GLenum = 0x8E1E;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_COMBINED_TESS_EVALUATION_UNIFORM_COMPONENTS: GLenum = 0x8E1F;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_COMBINED_TEXTURE_IMAGE_UNITS: GLenum = 0x8B4D;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_COMBINED_UNIFORM_BLOCKS: GLenum = 0x8A2E;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS: GLenum = 0x8A31;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_COMPUTE_ATOMIC_COUNTERS: GLenum = 0x8265;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_COMPUTE_ATOMIC_COUNTER_BUFFERS: GLenum = 0x8264;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_COMPUTE_IMAGE_UNIFORMS: GLenum = 0x91BD;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_COMPUTE_SHADER_STORAGE_BLOCKS: GLenum = 0x90DB;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_COMPUTE_SHARED_MEMORY_SIZE: GLenum = 0x8262;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_COMPUTE_TEXTURE_IMAGE_UNITS: GLenum = 0x91BC;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_COMPUTE_UNIFORM_BLOCKS: GLenum = 0x91BB;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_COMPUTE_UNIFORM_COMPONENTS: GLenum = 0x8263;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_COMPUTE_WORK_GROUP_COUNT: GLenum = 0x91BE;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_COMPUTE_WORK_GROUP_INVOCATIONS: GLenum = 0x90EB;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_COMPUTE_WORK_GROUP_SIZE: GLenum = 0x91BF;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_CUBE_MAP_TEXTURE_SIZE: GLenum = 0x851C;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_CULL_DISTANCES: GLenum = 0x82F9;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_DEBUG_GROUP_STACK_DEPTH: GLenum = 0x826C;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_DEBUG_LOGGED_MESSAGES: GLenum = 0x9144;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_DEBUG_MESSAGE_LENGTH: GLenum = 0x9143;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_DEPTH: GLenum = 0x8280;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_DEPTH_TEXTURE_SAMPLES: GLenum = 0x910F;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_DRAW_BUFFERS: GLenum = 0x8824;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_DUAL_SOURCE_DRAW_BUFFERS: GLenum = 0x88FC;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_ELEMENTS_INDICES: GLenum = 0x80E9;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_ELEMENTS_VERTICES: GLenum = 0x80E8;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_ELEMENT_INDEX: GLenum = 0x8D6B;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_EVAL_ORDER: GLenum = 0x0D30;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_FRAGMENT_ATOMIC_COUNTERS: GLenum = 0x92D6;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_FRAGMENT_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92D0;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_FRAGMENT_IMAGE_UNIFORMS: GLenum = 0x90CE;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_FRAGMENT_INPUT_COMPONENTS: GLenum = 0x9125;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_FRAGMENT_INTERPOLATION_OFFSET: GLenum = 0x8E5C;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_FRAGMENT_SHADER_STORAGE_BLOCKS: GLenum = 0x90DA;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_FRAGMENT_UNIFORM_BLOCKS: GLenum = 0x8A2D;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_FRAGMENT_UNIFORM_COMPONENTS: GLenum = 0x8B49;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_FRAGMENT_UNIFORM_VECTORS: GLenum = 0x8DFD;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_FRAMEBUFFER_HEIGHT: GLenum = 0x9316;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_FRAMEBUFFER_LAYERS: GLenum = 0x9317;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_FRAMEBUFFER_SAMPLES: GLenum = 0x9318;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_FRAMEBUFFER_WIDTH: GLenum = 0x9315;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_GEOMETRY_ATOMIC_COUNTERS: GLenum = 0x92D5;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_GEOMETRY_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92CF;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_GEOMETRY_IMAGE_UNIFORMS: GLenum = 0x90CD;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_GEOMETRY_INPUT_COMPONENTS: GLenum = 0x9123;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_GEOMETRY_OUTPUT_COMPONENTS: GLenum = 0x9124;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_GEOMETRY_OUTPUT_VERTICES: GLenum = 0x8DE0;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_GEOMETRY_SHADER_INVOCATIONS: GLenum = 0x8E5A;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_GEOMETRY_SHADER_STORAGE_BLOCKS: GLenum = 0x90D7;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_GEOMETRY_TEXTURE_IMAGE_UNITS: GLenum = 0x8C29;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_GEOMETRY_TOTAL_OUTPUT_COMPONENTS: GLenum = 0x8DE1;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_GEOMETRY_UNIFORM_BLOCKS: GLenum = 0x8A2C;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_GEOMETRY_UNIFORM_COMPONENTS: GLenum = 0x8DDF;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_HEIGHT: GLenum = 0x827F;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_IMAGE_SAMPLES: GLenum = 0x906D;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_IMAGE_UNITS: GLenum = 0x8F38;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_INTEGER_SAMPLES: GLenum = 0x9110;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_LABEL_LENGTH: GLenum = 0x82E8;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_LAYERS: GLenum = 0x8281;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_LIGHTS: GLenum = 0x0D31;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_LIST_NESTING: GLenum = 0x0B31;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_MODELVIEW_STACK_DEPTH: GLenum = 0x0D36;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_NAME_LENGTH: GLenum = 0x92F6;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_NAME_STACK_DEPTH: GLenum = 0x0D37;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_NUM_ACTIVE_VARIABLES: GLenum = 0x92F7;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_NUM_COMPATIBLE_SUBROUTINES: GLenum = 0x92F8;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_PATCH_VERTICES: GLenum = 0x8E7D;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_PIXEL_MAP_TABLE: GLenum = 0x0D34;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_PROGRAM_TEXEL_OFFSET: GLenum = 0x8905;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_PROGRAM_TEXTURE_GATHER_OFFSET: GLenum = 0x8E5F;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_PROJECTION_STACK_DEPTH: GLenum = 0x0D38;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_RECTANGLE_TEXTURE_SIZE: GLenum = 0x84F8;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_RENDERBUFFER_SIZE: GLenum = 0x84E8;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_SAMPLES: GLenum = 0x8D57;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_SAMPLE_MASK_WORDS: GLenum = 0x8E59;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_SERVER_WAIT_TIMEOUT: GLenum = 0x9111;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_SHADER_STORAGE_BLOCK_SIZE: GLenum = 0x90DE;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_SHADER_STORAGE_BUFFER_BINDINGS: GLenum = 0x90DD;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_SUBROUTINES: GLenum = 0x8DE7;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_SUBROUTINE_UNIFORM_LOCATIONS: GLenum = 0x8DE8;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_TESS_CONTROL_ATOMIC_COUNTERS: GLenum = 0x92D3;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_TESS_CONTROL_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92CD;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_TESS_CONTROL_IMAGE_UNIFORMS: GLenum = 0x90CB;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_TESS_CONTROL_INPUT_COMPONENTS: GLenum = 0x886C;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_TESS_CONTROL_OUTPUT_COMPONENTS: GLenum = 0x8E83;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_TESS_CONTROL_SHADER_STORAGE_BLOCKS: GLenum = 0x90D8;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_TESS_CONTROL_TEXTURE_IMAGE_UNITS: GLenum = 0x8E81;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_TESS_CONTROL_TOTAL_OUTPUT_COMPONENTS: GLenum = 0x8E85;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_TESS_CONTROL_UNIFORM_BLOCKS: GLenum = 0x8E89;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_TESS_CONTROL_UNIFORM_COMPONENTS: GLenum = 0x8E7F;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_TESS_EVALUATION_ATOMIC_COUNTERS: GLenum = 0x92D4;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_TESS_EVALUATION_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92CE;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_TESS_EVALUATION_IMAGE_UNIFORMS: GLenum = 0x90CC;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_TESS_EVALUATION_INPUT_COMPONENTS: GLenum = 0x886D;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_TESS_EVALUATION_OUTPUT_COMPONENTS: GLenum = 0x8E86;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_TESS_EVALUATION_SHADER_STORAGE_BLOCKS: GLenum = 0x90D9;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_TESS_EVALUATION_TEXTURE_IMAGE_UNITS: GLenum = 0x8E82;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_TESS_EVALUATION_UNIFORM_BLOCKS: GLenum = 0x8E8A;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_TESS_EVALUATION_UNIFORM_COMPONENTS: GLenum = 0x8E80;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_TESS_GEN_LEVEL: GLenum = 0x8E7E;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_TESS_PATCH_COMPONENTS: GLenum = 0x8E84;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_TEXTURE_BUFFER_SIZE: GLenum = 0x8C2B;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_TEXTURE_COORDS: GLenum = 0x8871;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_TEXTURE_IMAGE_UNITS: GLenum = 0x8872;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_TEXTURE_LOD_BIAS: GLenum = 0x84FD;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_TEXTURE_SIZE: GLenum = 0x0D33;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_TEXTURE_STACK_DEPTH: GLenum = 0x0D39;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_TEXTURE_UNITS: GLenum = 0x84E2;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_TRANSFORM_FEEDBACK_BUFFERS: GLenum = 0x8E70;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS: GLenum = 0x8C8A;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS: GLenum = 0x8C8B;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS: GLenum = 0x8C80;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_UNIFORM_BLOCK_SIZE: GLenum = 0x8A30;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_UNIFORM_BUFFER_BINDINGS: GLenum = 0x8A2F;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_UNIFORM_LOCATIONS: GLenum = 0x826E;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_VARYING_COMPONENTS: GLenum = 0x8B4B;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_VARYING_FLOATS: GLenum = 0x8B4B;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_VARYING_VECTORS: GLenum = 0x8DFC;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_VERTEX_ATOMIC_COUNTERS: GLenum = 0x92D2;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_VERTEX_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92CC;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_VERTEX_ATTRIBS: GLenum = 0x8869;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_VERTEX_ATTRIB_BINDINGS: GLenum = 0x82DA;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_VERTEX_ATTRIB_RELATIVE_OFFSET: GLenum = 0x82D9;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_VERTEX_ATTRIB_STRIDE: GLenum = 0x82E5;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_VERTEX_IMAGE_UNIFORMS: GLenum = 0x90CA;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_VERTEX_OUTPUT_COMPONENTS: GLenum = 0x9122;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_VERTEX_SHADER_STORAGE_BLOCKS: GLenum = 0x90D6;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_VERTEX_STREAMS: GLenum = 0x8E71;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_VERTEX_TEXTURE_IMAGE_UNITS: GLenum = 0x8B4C;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_VERTEX_UNIFORM_BLOCKS: GLenum = 0x8A2B;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_VERTEX_UNIFORM_COMPONENTS: GLenum = 0x8B4A;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_VERTEX_UNIFORM_VECTORS: GLenum = 0x8DFB;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_VIEWPORTS: GLenum = 0x825B;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_VIEWPORT_DIMS: GLenum = 0x0D3A;
#[allow(dead_code, non_upper_case_globals)]
pub const MAX_WIDTH: GLenum = 0x827E;
#[allow(dead_code, non_upper_case_globals)]
pub const MEDIUM_FLOAT: GLenum = 0x8DF1;
#[allow(dead_code, non_upper_case_globals)]
pub const MEDIUM_INT: GLenum = 0x8DF4;
#[allow(dead_code, non_upper_case_globals)]
pub const MIN: GLenum = 0x8007;
#[allow(dead_code, non_upper_case_globals)]
pub const MINOR_VERSION: GLenum = 0x821C;
#[allow(dead_code, non_upper_case_globals)]
pub const MIN_FRAGMENT_INTERPOLATION_OFFSET: GLenum = 0x8E5B;
#[allow(dead_code, non_upper_case_globals)]
pub const MIN_MAP_BUFFER_ALIGNMENT: GLenum = 0x90BC;
#[allow(dead_code, non_upper_case_globals)]
pub const MIN_PROGRAM_TEXEL_OFFSET: GLenum = 0x8904;
#[allow(dead_code, non_upper_case_globals)]
pub const MIN_PROGRAM_TEXTURE_GATHER_OFFSET: GLenum = 0x8E5E;
#[allow(dead_code, non_upper_case_globals)]
pub const MIN_SAMPLE_SHADING_VALUE: GLenum = 0x8C37;
#[allow(dead_code, non_upper_case_globals)]
pub const MIPMAP: GLenum = 0x8293;
#[allow(dead_code, non_upper_case_globals)]
pub const MIRRORED_REPEAT: GLenum = 0x8370;
#[allow(dead_code, non_upper_case_globals)]
pub const MIRROR_CLAMP_TO_EDGE: GLenum = 0x8743;
#[allow(dead_code, non_upper_case_globals)]
pub const MODELVIEW: GLenum = 0x1700;
#[allow(dead_code, non_upper_case_globals)]
pub const MODELVIEW_MATRIX: GLenum = 0x0BA6;
#[allow(dead_code, non_upper_case_globals)]
pub const MODELVIEW_STACK_DEPTH: GLenum = 0x0BA3;
#[allow(dead_code, non_upper_case_globals)]
pub const MODULATE: GLenum = 0x2100;
#[allow(dead_code, non_upper_case_globals)]
pub const MULT: GLenum = 0x0103;
#[allow(dead_code, non_upper_case_globals)]
pub const MULTISAMPLE: GLenum = 0x809D;
#[allow(dead_code, non_upper_case_globals)]
pub const MULTISAMPLE_BIT: GLenum = 0x20000000;
#[allow(dead_code, non_upper_case_globals)]
pub const N3F_V3F: GLenum = 0x2A25;
#[allow(dead_code, non_upper_case_globals)]
pub const NAME_LENGTH: GLenum = 0x92F9;
#[allow(dead_code, non_upper_case_globals)]
pub const NAME_STACK_DEPTH: GLenum = 0x0D70;
#[allow(dead_code, non_upper_case_globals)]
pub const NAND: GLenum = 0x150E;
#[allow(dead_code, non_upper_case_globals)]
pub const NEAREST: GLenum = 0x2600;
#[allow(dead_code, non_upper_case_globals)]
pub const NEAREST_MIPMAP_LINEAR: GLenum = 0x2702;
#[allow(dead_code, non_upper_case_globals)]
pub const NEAREST_MIPMAP_NEAREST: GLenum = 0x2700;
#[allow(dead_code, non_upper_case_globals)]
pub const NEGATIVE_ONE_TO_ONE: GLenum = 0x935E;
#[allow(dead_code, non_upper_case_globals)]
pub const NEVER: GLenum = 0x0200;
#[allow(dead_code, non_upper_case_globals)]
pub const NICEST: GLenum = 0x1102;
#[allow(dead_code, non_upper_case_globals)]
pub const NONE: GLenum = 0;
#[allow(dead_code, non_upper_case_globals)]
pub const NOOP: GLenum = 0x1505;
#[allow(dead_code, non_upper_case_globals)]
pub const NOR: GLenum = 0x1508;
#[allow(dead_code, non_upper_case_globals)]
pub const NORMALIZE: GLenum = 0x0BA1;
#[allow(dead_code, non_upper_case_globals)]
pub const NORMAL_ARRAY: GLenum = 0x8075;
#[allow(dead_code, non_upper_case_globals)]
pub const NORMAL_ARRAY_BUFFER_BINDING: GLenum = 0x8897;
#[allow(dead_code, non_upper_case_globals)]
pub const NORMAL_ARRAY_POINTER: GLenum = 0x808F;
#[allow(dead_code, non_upper_case_globals)]
pub const NORMAL_ARRAY_STRIDE: GLenum = 0x807F;
#[allow(dead_code, non_upper_case_globals)]
pub const NORMAL_ARRAY_TYPE: GLenum = 0x807E;
#[allow(dead_code, non_upper_case_globals)]
pub const NORMAL_MAP: GLenum = 0x8511;
#[allow(dead_code, non_upper_case_globals)]
pub const NOTEQUAL: GLenum = 0x0205;
#[allow(dead_code, non_upper_case_globals)]
pub const NO_ERROR: GLenum = 0;
#[allow(dead_code, non_upper_case_globals)]
pub const NO_RESET_NOTIFICATION: GLenum = 0x8261;
#[allow(dead_code, non_upper_case_globals)]
pub const NUM_ACTIVE_VARIABLES: GLenum = 0x9304;
#[allow(dead_code, non_upper_case_globals)]
pub const NUM_COMPATIBLE_SUBROUTINES: GLenum = 0x8E4A;
#[allow(dead_code, non_upper_case_globals)]
pub const NUM_COMPRESSED_TEXTURE_FORMATS: GLenum = 0x86A2;
#[allow(dead_code, non_upper_case_globals)]
pub const NUM_EXTENSIONS: GLenum = 0x821D;
#[allow(dead_code, non_upper_case_globals)]
pub const NUM_PROGRAM_BINARY_FORMATS: GLenum = 0x87FE;
#[allow(dead_code, non_upper_case_globals)]
pub const NUM_SAMPLE_COUNTS: GLenum = 0x9380;
#[allow(dead_code, non_upper_case_globals)]
pub const NUM_SHADER_BINARY_FORMATS: GLenum = 0x8DF9;
#[allow(dead_code, non_upper_case_globals)]
pub const NUM_SHADING_LANGUAGE_VERSIONS: GLenum = 0x82E9;
#[allow(dead_code, non_upper_case_globals)]
pub const OBJECT_LINEAR: GLenum = 0x2401;
#[allow(dead_code, non_upper_case_globals)]
pub const OBJECT_PLANE: GLenum = 0x2501;
#[allow(dead_code, non_upper_case_globals)]
pub const OBJECT_TYPE: GLenum = 0x9112;
#[allow(dead_code, non_upper_case_globals)]
pub const OFFSET: GLenum = 0x92FC;
#[allow(dead_code, non_upper_case_globals)]
pub const ONE: GLenum = 1;
#[allow(dead_code, non_upper_case_globals)]
pub const ONE_MINUS_CONSTANT_ALPHA: GLenum = 0x8004;
#[allow(dead_code, non_upper_case_globals)]
pub const ONE_MINUS_CONSTANT_COLOR: GLenum = 0x8002;
#[allow(dead_code, non_upper_case_globals)]
pub const ONE_MINUS_DST_ALPHA: GLenum = 0x0305;
#[allow(dead_code, non_upper_case_globals)]
pub const ONE_MINUS_DST_COLOR: GLenum = 0x0307;
#[allow(dead_code, non_upper_case_globals)]
pub const ONE_MINUS_SRC1_ALPHA: GLenum = 0x88FB;
#[allow(dead_code, non_upper_case_globals)]
pub const ONE_MINUS_SRC1_COLOR: GLenum = 0x88FA;
#[allow(dead_code, non_upper_case_globals)]
pub const ONE_MINUS_SRC_ALPHA: GLenum = 0x0303;
#[allow(dead_code, non_upper_case_globals)]
pub const ONE_MINUS_SRC_COLOR: GLenum = 0x0301;
#[allow(dead_code, non_upper_case_globals)]
pub const OPERAND0_ALPHA: GLenum = 0x8598;
#[allow(dead_code, non_upper_case_globals)]
pub const OPERAND0_RGB: GLenum = 0x8590;
#[allow(dead_code, non_upper_case_globals)]
pub const OPERAND1_ALPHA: GLenum = 0x8599;
#[allow(dead_code, non_upper_case_globals)]
pub const OPERAND1_RGB: GLenum = 0x8591;
#[allow(dead_code, non_upper_case_globals)]
pub const OPERAND2_ALPHA: GLenum = 0x859A;
#[allow(dead_code, non_upper_case_globals)]
pub const OPERAND2_RGB: GLenum = 0x8592;
#[allow(dead_code, non_upper_case_globals)]
pub const OR: GLenum = 0x1507;
#[allow(dead_code, non_upper_case_globals)]
pub const ORDER: GLenum = 0x0A01;
#[allow(dead_code, non_upper_case_globals)]
pub const OR_INVERTED: GLenum = 0x150D;
#[allow(dead_code, non_upper_case_globals)]
pub const OR_REVERSE: GLenum = 0x150B;
#[allow(dead_code, non_upper_case_globals)]
pub const OUT_OF_MEMORY: GLenum = 0x0505;
#[allow(dead_code, non_upper_case_globals)]
pub const PACK_ALIGNMENT: GLenum = 0x0D05;
#[allow(dead_code, non_upper_case_globals)]
pub const PACK_COMPRESSED_BLOCK_DEPTH: GLenum = 0x912D;
#[allow(dead_code, non_upper_case_globals)]
pub const PACK_COMPRESSED_BLOCK_HEIGHT: GLenum = 0x912C;
#[allow(dead_code, non_upper_case_globals)]
pub const PACK_COMPRESSED_BLOCK_SIZE: GLenum = 0x912E;
#[allow(dead_code, non_upper_case_globals)]
pub const PACK_COMPRESSED_BLOCK_WIDTH: GLenum = 0x912B;
#[allow(dead_code, non_upper_case_globals)]
pub const PACK_IMAGE_HEIGHT: GLenum = 0x806C;
#[allow(dead_code, non_upper_case_globals)]
pub const PACK_LSB_FIRST: GLenum = 0x0D01;
#[allow(dead_code, non_upper_case_globals)]
pub const PACK_ROW_LENGTH: GLenum = 0x0D02;
#[allow(dead_code, non_upper_case_globals)]
pub const PACK_SKIP_IMAGES: GLenum = 0x806B;
#[allow(dead_code, non_upper_case_globals)]
pub const PACK_SKIP_PIXELS: GLenum = 0x0D04;
#[allow(dead_code, non_upper_case_globals)]
pub const PACK_SKIP_ROWS: GLenum = 0x0D03;
#[allow(dead_code, non_upper_case_globals)]
pub const PACK_SWAP_BYTES: GLenum = 0x0D00;
#[allow(dead_code, non_upper_case_globals)]
pub const PASS_THROUGH_TOKEN: GLenum = 0x0700;
#[allow(dead_code, non_upper_case_globals)]
pub const PATCHES: GLenum = 0x000E;
#[allow(dead_code, non_upper_case_globals)]
pub const PATCH_DEFAULT_INNER_LEVEL: GLenum = 0x8E73;
#[allow(dead_code, non_upper_case_globals)]
pub const PATCH_DEFAULT_OUTER_LEVEL: GLenum = 0x8E74;
#[allow(dead_code, non_upper_case_globals)]
pub const PATCH_VERTICES: GLenum = 0x8E72;
#[allow(dead_code, non_upper_case_globals)]
pub const PERSPECTIVE_CORRECTION_HINT: GLenum = 0x0C50;
#[allow(dead_code, non_upper_case_globals)]
pub const PIXEL_BUFFER_BARRIER_BIT: GLenum = 0x00000080;
#[allow(dead_code, non_upper_case_globals)]
pub const PIXEL_MAP_A_TO_A: GLenum = 0x0C79;
#[allow(dead_code, non_upper_case_globals)]
pub const PIXEL_MAP_A_TO_A_SIZE: GLenum = 0x0CB9;
#[allow(dead_code, non_upper_case_globals)]
pub const PIXEL_MAP_B_TO_B: GLenum = 0x0C78;
#[allow(dead_code, non_upper_case_globals)]
pub const PIXEL_MAP_B_TO_B_SIZE: GLenum = 0x0CB8;
#[allow(dead_code, non_upper_case_globals)]
pub const PIXEL_MAP_G_TO_G: GLenum = 0x0C77;
#[allow(dead_code, non_upper_case_globals)]
pub const PIXEL_MAP_G_TO_G_SIZE: GLenum = 0x0CB7;
#[allow(dead_code, non_upper_case_globals)]
pub const PIXEL_MAP_I_TO_A: GLenum = 0x0C75;
#[allow(dead_code, non_upper_case_globals)]
pub const PIXEL_MAP_I_TO_A_SIZE: GLenum = 0x0CB5;
#[allow(dead_code, non_upper_case_globals)]
pub const PIXEL_MAP_I_TO_B: GLenum = 0x0C74;
#[allow(dead_code, non_upper_case_globals)]
pub const PIXEL_MAP_I_TO_B_SIZE: GLenum = 0x0CB4;
#[allow(dead_code, non_upper_case_globals)]
pub const PIXEL_MAP_I_TO_G: GLenum = 0x0C73;
#[allow(dead_code, non_upper_case_globals)]
pub const PIXEL_MAP_I_TO_G_SIZE: GLenum = 0x0CB3;
#[allow(dead_code, non_upper_case_globals)]
pub const PIXEL_MAP_I_TO_I: GLenum = 0x0C70;
#[allow(dead_code, non_upper_case_globals)]
pub const PIXEL_MAP_I_TO_I_SIZE: GLenum = 0x0CB0;
#[allow(dead_code, non_upper_case_globals)]
pub const PIXEL_MAP_I_TO_R: GLenum = 0x0C72;
#[allow(dead_code, non_upper_case_globals)]
pub const PIXEL_MAP_I_TO_R_SIZE: GLenum = 0x0CB2;
#[allow(dead_code, non_upper_case_globals)]
pub const PIXEL_MAP_R_TO_R: GLenum = 0x0C76;
#[allow(dead_code, non_upper_case_globals)]
pub const PIXEL_MAP_R_TO_R_SIZE: GLenum = 0x0CB6;
#[allow(dead_code, non_upper_case_globals)]
pub const PIXEL_MAP_S_TO_S: GLenum = 0x0C71;
#[allow(dead_code, non_upper_case_globals)]
pub const PIXEL_MAP_S_TO_S_SIZE: GLenum = 0x0CB1;
#[allow(dead_code, non_upper_case_globals)]
pub const PIXEL_MODE_BIT: GLenum = 0x00000020;
#[allow(dead_code, non_upper_case_globals)]
pub const PIXEL_PACK_BUFFER: GLenum = 0x88EB;
#[allow(dead_code, non_upper_case_globals)]
pub const PIXEL_PACK_BUFFER_BINDING: GLenum = 0x88ED;
#[allow(dead_code, non_upper_case_globals)]
pub const PIXEL_UNPACK_BUFFER: GLenum = 0x88EC;
#[allow(dead_code, non_upper_case_globals)]
pub const PIXEL_UNPACK_BUFFER_BINDING: GLenum = 0x88EF;
#[allow(dead_code, non_upper_case_globals)]
pub const POINT: GLenum = 0x1B00;
#[allow(dead_code, non_upper_case_globals)]
pub const POINTS: GLenum = 0x0000;
#[allow(dead_code, non_upper_case_globals)]
pub const POINT_BIT: GLenum = 0x00000002;
#[allow(dead_code, non_upper_case_globals)]
pub const POINT_DISTANCE_ATTENUATION: GLenum = 0x8129;
#[allow(dead_code, non_upper_case_globals)]
pub const POINT_FADE_THRESHOLD_SIZE: GLenum = 0x8128;
#[allow(dead_code, non_upper_case_globals)]
pub const POINT_SIZE: GLenum = 0x0B11;
#[allow(dead_code, non_upper_case_globals)]
pub const POINT_SIZE_GRANULARITY: GLenum = 0x0B13;
#[allow(dead_code, non_upper_case_globals)]
pub const POINT_SIZE_MAX: GLenum = 0x8127;
#[allow(dead_code, non_upper_case_globals)]
pub const POINT_SIZE_MIN: GLenum = 0x8126;
#[allow(dead_code, non_upper_case_globals)]
pub const POINT_SIZE_RANGE: GLenum = 0x0B12;
#[allow(dead_code, non_upper_case_globals)]
pub const POINT_SMOOTH: GLenum = 0x0B10;
#[allow(dead_code, non_upper_case_globals)]
pub const POINT_SMOOTH_HINT: GLenum = 0x0C51;
#[allow(dead_code, non_upper_case_globals)]
pub const POINT_SPRITE: GLenum = 0x8861;
#[allow(dead_code, non_upper_case_globals)]
pub const POINT_SPRITE_COORD_ORIGIN: GLenum = 0x8CA0;
#[allow(dead_code, non_upper_case_globals)]
pub const POINT_TOKEN: GLenum = 0x0701;
#[allow(dead_code, non_upper_case_globals)]
pub const POLYGON: GLenum = 0x0009;
#[allow(dead_code, non_upper_case_globals)]
pub const POLYGON_BIT: GLenum = 0x00000008;
#[allow(dead_code, non_upper_case_globals)]
pub const POLYGON_MODE: GLenum = 0x0B40;
#[allow(dead_code, non_upper_case_globals)]
pub const POLYGON_OFFSET_FACTOR: GLenum = 0x8038;
#[allow(dead_code, non_upper_case_globals)]
pub const POLYGON_OFFSET_FILL: GLenum = 0x8037;
#[allow(dead_code, non_upper_case_globals)]
pub const POLYGON_OFFSET_LINE: GLenum = 0x2A02;
#[allow(dead_code, non_upper_case_globals)]
pub const POLYGON_OFFSET_POINT: GLenum = 0x2A01;
#[allow(dead_code, non_upper_case_globals)]
pub const POLYGON_OFFSET_UNITS: GLenum = 0x2A00;
#[allow(dead_code, non_upper_case_globals)]
pub const POLYGON_SMOOTH: GLenum = 0x0B41;
#[allow(dead_code, non_upper_case_globals)]
pub const POLYGON_SMOOTH_HINT: GLenum = 0x0C53;
#[allow(dead_code, non_upper_case_globals)]
pub const POLYGON_STIPPLE: GLenum = 0x0B42;
#[allow(dead_code, non_upper_case_globals)]
pub const POLYGON_STIPPLE_BIT: GLenum = 0x00000010;
#[allow(dead_code, non_upper_case_globals)]
pub const POLYGON_TOKEN: GLenum = 0x0703;
#[allow(dead_code, non_upper_case_globals)]
pub const POSITION: GLenum = 0x1203;
#[allow(dead_code, non_upper_case_globals)]
pub const PREVIOUS: GLenum = 0x8578;
#[allow(dead_code, non_upper_case_globals)]
pub const PRIMARY_COLOR: GLenum = 0x8577;
#[allow(dead_code, non_upper_case_globals)]
pub const PRIMITIVES_GENERATED: GLenum = 0x8C87;
#[allow(dead_code, non_upper_case_globals)]
pub const PRIMITIVE_RESTART: GLenum = 0x8F9D;
#[allow(dead_code, non_upper_case_globals)]
pub const PRIMITIVE_RESTART_FIXED_INDEX: GLenum = 0x8D69;
#[allow(dead_code, non_upper_case_globals)]
pub const PRIMITIVE_RESTART_FOR_PATCHES_SUPPORTED: GLenum = 0x8221;
#[allow(dead_code, non_upper_case_globals)]
pub const PRIMITIVE_RESTART_INDEX: GLenum = 0x8F9E;
#[allow(dead_code, non_upper_case_globals)]
pub const PROGRAM: GLenum = 0x82E2;
#[allow(dead_code, non_upper_case_globals)]
pub const PROGRAM_BINARY_FORMATS: GLenum = 0x87FF;
#[allow(dead_code, non_upper_case_globals)]
pub const PROGRAM_BINARY_LENGTH: GLenum = 0x8741;
#[allow(dead_code, non_upper_case_globals)]
pub const PROGRAM_BINARY_RETRIEVABLE_HINT: GLenum = 0x8257;
#[allow(dead_code, non_upper_case_globals)]
pub const PROGRAM_INPUT: GLenum = 0x92E3;
#[allow(dead_code, non_upper_case_globals)]
pub const PROGRAM_OUTPUT: GLenum = 0x92E4;
#[allow(dead_code, non_upper_case_globals)]
pub const PROGRAM_PIPELINE: GLenum = 0x82E4;
#[allow(dead_code, non_upper_case_globals)]
pub const PROGRAM_PIPELINE_BINDING: GLenum = 0x825A;
#[allow(dead_code, non_upper_case_globals)]
pub const PROGRAM_POINT_SIZE: GLenum = 0x8642;
#[allow(dead_code, non_upper_case_globals)]
pub const PROGRAM_SEPARABLE: GLenum = 0x8258;
#[allow(dead_code, non_upper_case_globals)]
pub const PROJECTION: GLenum = 0x1701;
#[allow(dead_code, non_upper_case_globals)]
pub const PROJECTION_MATRIX: GLenum = 0x0BA7;
#[allow(dead_code, non_upper_case_globals)]
pub const PROJECTION_STACK_DEPTH: GLenum = 0x0BA4;
#[allow(dead_code, non_upper_case_globals)]
pub const PROVOKING_VERTEX: GLenum = 0x8E4F;
#[allow(dead_code, non_upper_case_globals)]
pub const PROXY_TEXTURE_1D: GLenum = 0x8063;
#[allow(dead_code, non_upper_case_globals)]
pub const PROXY_TEXTURE_1D_ARRAY: GLenum = 0x8C19;
#[allow(dead_code, non_upper_case_globals)]
pub const PROXY_TEXTURE_2D: GLenum = 0x8064;
#[allow(dead_code, non_upper_case_globals)]
pub const PROXY_TEXTURE_2D_ARRAY: GLenum = 0x8C1B;
#[allow(dead_code, non_upper_case_globals)]
pub const PROXY_TEXTURE_2D_MULTISAMPLE: GLenum = 0x9101;
#[allow(dead_code, non_upper_case_globals)]
pub const PROXY_TEXTURE_2D_MULTISAMPLE_ARRAY: GLenum = 0x9103;
#[allow(dead_code, non_upper_case_globals)]
pub const PROXY_TEXTURE_3D: GLenum = 0x8070;
#[allow(dead_code, non_upper_case_globals)]
pub const PROXY_TEXTURE_CUBE_MAP: GLenum = 0x851B;
#[allow(dead_code, non_upper_case_globals)]
pub const PROXY_TEXTURE_CUBE_MAP_ARRAY: GLenum = 0x900B;
#[allow(dead_code, non_upper_case_globals)]
pub const PROXY_TEXTURE_RECTANGLE: GLenum = 0x84F7;
#[allow(dead_code, non_upper_case_globals)]
pub const Q: GLenum = 0x2003;
#[allow(dead_code, non_upper_case_globals)]
pub const QUADRATIC_ATTENUATION: GLenum = 0x1209;
#[allow(dead_code, non_upper_case_globals)]
pub const QUADS: GLenum = 0x0007;
#[allow(dead_code, non_upper_case_globals)]
pub const QUADS_FOLLOW_PROVOKING_VERTEX_CONVENTION: GLenum = 0x8E4C;
#[allow(dead_code, non_upper_case_globals)]
pub const QUAD_STRIP: GLenum = 0x0008;
#[allow(dead_code, non_upper_case_globals)]
pub const QUERY: GLenum = 0x82E3;
#[allow(dead_code, non_upper_case_globals)]
pub const QUERY_BUFFER: GLenum = 0x9192;
#[allow(dead_code, non_upper_case_globals)]
pub const QUERY_BUFFER_BARRIER_BIT: GLenum = 0x00008000;
#[allow(dead_code, non_upper_case_globals)]
pub const QUERY_BUFFER_BINDING: GLenum = 0x9193;
#[allow(dead_code, non_upper_case_globals)]
pub const QUERY_BY_REGION_NO_WAIT: GLenum = 0x8E16;
#[allow(dead_code, non_upper_case_globals)]
pub const QUERY_BY_REGION_NO_WAIT_INVERTED: GLenum = 0x8E1A;
#[allow(dead_code, non_upper_case_globals)]
pub const QUERY_BY_REGION_WAIT: GLenum = 0x8E15;
#[allow(dead_code, non_upper_case_globals)]
pub const QUERY_BY_REGION_WAIT_INVERTED: GLenum = 0x8E19;
#[allow(dead_code, non_upper_case_globals)]
pub const QUERY_COUNTER_BITS: GLenum = 0x8864;
#[allow(dead_code, non_upper_case_globals)]
pub const QUERY_NO_WAIT: GLenum = 0x8E14;
#[allow(dead_code, non_upper_case_globals)]
pub const QUERY_NO_WAIT_INVERTED: GLenum = 0x8E18;
#[allow(dead_code, non_upper_case_globals)]
pub const QUERY_RESULT: GLenum = 0x8866;
#[allow(dead_code, non_upper_case_globals)]
pub const QUERY_RESULT_AVAILABLE: GLenum = 0x8867;
#[allow(dead_code, non_upper_case_globals)]
pub const QUERY_RESULT_NO_WAIT: GLenum = 0x9194;
#[allow(dead_code, non_upper_case_globals)]
pub const QUERY_TARGET: GLenum = 0x82EA;
#[allow(dead_code, non_upper_case_globals)]
pub const QUERY_WAIT: GLenum = 0x8E13;
#[allow(dead_code, non_upper_case_globals)]
pub const QUERY_WAIT_INVERTED: GLenum = 0x8E17;
#[allow(dead_code, non_upper_case_globals)]
pub const R: GLenum = 0x2002;
#[allow(dead_code, non_upper_case_globals)]
pub const R11F_G11F_B10F: GLenum = 0x8C3A;
#[allow(dead_code, non_upper_case_globals)]
pub const R16: GLenum = 0x822A;
#[allow(dead_code, non_upper_case_globals)]
pub const R16F: GLenum = 0x822D;
#[allow(dead_code, non_upper_case_globals)]
pub const R16I: GLenum = 0x8233;
#[allow(dead_code, non_upper_case_globals)]
pub const R16UI: GLenum = 0x8234;
#[allow(dead_code, non_upper_case_globals)]
pub const R16_SNORM: GLenum = 0x8F98;
#[allow(dead_code, non_upper_case_globals)]
pub const R32F: GLenum = 0x822E;
#[allow(dead_code, non_upper_case_globals)]
pub const R32I: GLenum = 0x8235;
#[allow(dead_code, non_upper_case_globals)]
pub const R32UI: GLenum = 0x8236;
#[allow(dead_code, non_upper_case_globals)]
pub const R3_G3_B2: GLenum = 0x2A10;
#[allow(dead_code, non_upper_case_globals)]
pub const R8: GLenum = 0x8229;
#[allow(dead_code, non_upper_case_globals)]
pub const R8I: GLenum = 0x8231;
#[allow(dead_code, non_upper_case_globals)]
pub const R8UI: GLenum = 0x8232;
#[allow(dead_code, non_upper_case_globals)]
pub const R8_SNORM: GLenum = 0x8F94;
#[allow(dead_code, non_upper_case_globals)]
pub const RASTERIZER_DISCARD: GLenum = 0x8C89;
#[allow(dead_code, non_upper_case_globals)]
pub const READ_BUFFER: GLenum = 0x0C02;
#[allow(dead_code, non_upper_case_globals)]
pub const READ_FRAMEBUFFER: GLenum = 0x8CA8;
#[allow(dead_code, non_upper_case_globals)]
pub const READ_FRAMEBUFFER_BINDING: GLenum = 0x8CAA;
#[allow(dead_code, non_upper_case_globals)]
pub const READ_ONLY: GLenum = 0x88B8;
#[allow(dead_code, non_upper_case_globals)]
pub const READ_PIXELS: GLenum = 0x828C;
#[allow(dead_code, non_upper_case_globals)]
pub const READ_PIXELS_FORMAT: GLenum = 0x828D;
#[allow(dead_code, non_upper_case_globals)]
pub const READ_PIXELS_TYPE: GLenum = 0x828E;
#[allow(dead_code, non_upper_case_globals)]
pub const READ_WRITE: GLenum = 0x88BA;
#[allow(dead_code, non_upper_case_globals)]
pub const RED: GLenum = 0x1903;
#[allow(dead_code, non_upper_case_globals)]
pub const RED_BIAS: GLenum = 0x0D15;
#[allow(dead_code, non_upper_case_globals)]
pub const RED_BITS: GLenum = 0x0D52;
#[allow(dead_code, non_upper_case_globals)]
pub const RED_INTEGER: GLenum = 0x8D94;
#[allow(dead_code, non_upper_case_globals)]
pub const RED_SCALE: GLenum = 0x0D14;
#[allow(dead_code, non_upper_case_globals)]
pub const REFERENCED_BY_COMPUTE_SHADER: GLenum = 0x930B;
#[allow(dead_code, non_upper_case_globals)]
pub const REFERENCED_BY_FRAGMENT_SHADER: GLenum = 0x930A;
#[allow(dead_code, non_upper_case_globals)]
pub const REFERENCED_BY_GEOMETRY_SHADER: GLenum = 0x9309;
#[allow(dead_code, non_upper_case_globals)]
pub const REFERENCED_BY_TESS_CONTROL_SHADER: GLenum = 0x9307;
#[allow(dead_code, non_upper_case_globals)]
pub const REFERENCED_BY_TESS_EVALUATION_SHADER: GLenum = 0x9308;
#[allow(dead_code, non_upper_case_globals)]
pub const REFERENCED_BY_VERTEX_SHADER: GLenum = 0x9306;
#[allow(dead_code, non_upper_case_globals)]
pub const REFLECTION_MAP: GLenum = 0x8512;
#[allow(dead_code, non_upper_case_globals)]
pub const RENDER: GLenum = 0x1C00;
#[allow(dead_code, non_upper_case_globals)]
pub const RENDERBUFFER: GLenum = 0x8D41;
#[allow(dead_code, non_upper_case_globals)]
pub const RENDERBUFFER_ALPHA_SIZE: GLenum = 0x8D53;
#[allow(dead_code, non_upper_case_globals)]
pub const RENDERBUFFER_BINDING: GLenum = 0x8CA7;
#[allow(dead_code, non_upper_case_globals)]
pub const RENDERBUFFER_BLUE_SIZE: GLenum = 0x8D52;
#[allow(dead_code, non_upper_case_globals)]
pub const RENDERBUFFER_DEPTH_SIZE: GLenum = 0x8D54;
#[allow(dead_code, non_upper_case_globals)]
pub const RENDERBUFFER_GREEN_SIZE: GLenum = 0x8D51;
#[allow(dead_code, non_upper_case_globals)]
pub const RENDERBUFFER_HEIGHT: GLenum = 0x8D43;
#[allow(dead_code, non_upper_case_globals)]
pub const RENDERBUFFER_INTERNAL_FORMAT: GLenum = 0x8D44;
#[allow(dead_code, non_upper_case_globals)]
pub const RENDERBUFFER_RED_SIZE: GLenum = 0x8D50;
#[allow(dead_code, non_upper_case_globals)]
pub const RENDERBUFFER_SAMPLES: GLenum = 0x8CAB;
#[allow(dead_code, non_upper_case_globals)]
pub const RENDERBUFFER_STENCIL_SIZE: GLenum = 0x8D55;
#[allow(dead_code, non_upper_case_globals)]
pub const RENDERBUFFER_WIDTH: GLenum = 0x8D42;
#[allow(dead_code, non_upper_case_globals)]
pub const RENDERER: GLenum = 0x1F01;
#[allow(dead_code, non_upper_case_globals)]
pub const RENDER_MODE: GLenum = 0x0C40;
#[allow(dead_code, non_upper_case_globals)]
pub const REPEAT: GLenum = 0x2901;
#[allow(dead_code, non_upper_case_globals)]
pub const REPLACE: GLenum = 0x1E01;
#[allow(dead_code, non_upper_case_globals)]
pub const RESCALE_NORMAL: GLenum = 0x803A;
#[allow(dead_code, non_upper_case_globals)]
pub const RESET_NOTIFICATION_STRATEGY: GLenum = 0x8256;
#[allow(dead_code, non_upper_case_globals)]
pub const RETURN: GLenum = 0x0102;
#[allow(dead_code, non_upper_case_globals)]
pub const RG: GLenum = 0x8227;
#[allow(dead_code, non_upper_case_globals)]
pub const RG16: GLenum = 0x822C;
#[allow(dead_code, non_upper_case_globals)]
pub const RG16F: GLenum = 0x822F;
#[allow(dead_code, non_upper_case_globals)]
pub const RG16I: GLenum = 0x8239;
#[allow(dead_code, non_upper_case_globals)]
pub const RG16UI: GLenum = 0x823A;
#[allow(dead_code, non_upper_case_globals)]
pub const RG16_SNORM: GLenum = 0x8F99;
#[allow(dead_code, non_upper_case_globals)]
pub const RG32F: GLenum = 0x8230;
#[allow(dead_code, non_upper_case_globals)]
pub const RG32I: GLenum = 0x823B;
#[allow(dead_code, non_upper_case_globals)]
pub const RG32UI: GLenum = 0x823C;
#[allow(dead_code, non_upper_case_globals)]
pub const RG8: GLenum = 0x822B;
#[allow(dead_code, non_upper_case_globals)]
pub const RG8I: GLenum = 0x8237;
#[allow(dead_code, non_upper_case_globals)]
pub const RG8UI: GLenum = 0x8238;
#[allow(dead_code, non_upper_case_globals)]
pub const RG8_SNORM: GLenum = 0x8F95;
#[allow(dead_code, non_upper_case_globals)]
pub const RGB: GLenum = 0x1907;
#[allow(dead_code, non_upper_case_globals)]
pub const RGB10: GLenum = 0x8052;
#[allow(dead_code, non_upper_case_globals)]
pub const RGB10_A2: GLenum = 0x8059;
#[allow(dead_code, non_upper_case_globals)]
pub const RGB10_A2UI: GLenum = 0x906F;
#[allow(dead_code, non_upper_case_globals)]
pub const RGB12: GLenum = 0x8053;
#[allow(dead_code, non_upper_case_globals)]
pub const RGB16: GLenum = 0x8054;
#[allow(dead_code, non_upper_case_globals)]
pub const RGB16F: GLenum = 0x881B;
#[allow(dead_code, non_upper_case_globals)]
pub const RGB16I: GLenum = 0x8D89;
#[allow(dead_code, non_upper_case_globals)]
pub const RGB16UI: GLenum = 0x8D77;
#[allow(dead_code, non_upper_case_globals)]
pub const RGB16_SNORM: GLenum = 0x8F9A;
#[allow(dead_code, non_upper_case_globals)]
pub const RGB32F: GLenum = 0x8815;
#[allow(dead_code, non_upper_case_globals)]
pub const RGB32I: GLenum = 0x8D83;
#[allow(dead_code, non_upper_case_globals)]
pub const RGB32UI: GLenum = 0x8D71;
#[allow(dead_code, non_upper_case_globals)]
pub const RGB4: GLenum = 0x804F;
#[allow(dead_code, non_upper_case_globals)]
pub const RGB5: GLenum = 0x8050;
#[allow(dead_code, non_upper_case_globals)]
pub const RGB565: GLenum = 0x8D62;
#[allow(dead_code, non_upper_case_globals)]
pub const RGB5_A1: GLenum = 0x8057;
#[allow(dead_code, non_upper_case_globals)]
pub const RGB8: GLenum = 0x8051;
#[allow(dead_code, non_upper_case_globals)]
pub const RGB8I: GLenum = 0x8D8F;
#[allow(dead_code, non_upper_case_globals)]
pub const RGB8UI: GLenum = 0x8D7D;
#[allow(dead_code, non_upper_case_globals)]
pub const RGB8_SNORM: GLenum = 0x8F96;
#[allow(dead_code, non_upper_case_globals)]
pub const RGB9_E5: GLenum = 0x8C3D;
#[allow(dead_code, non_upper_case_globals)]
pub const RGBA: GLenum = 0x1908;
#[allow(dead_code, non_upper_case_globals)]
pub const RGBA12: GLenum = 0x805A;
#[allow(dead_code, non_upper_case_globals)]
pub const RGBA16: GLenum = 0x805B;
#[allow(dead_code, non_upper_case_globals)]
pub const RGBA16F: GLenum = 0x881A;
#[allow(dead_code, non_upper_case_globals)]
pub const RGBA16I: GLenum = 0x8D88;
#[allow(dead_code, non_upper_case_globals)]
pub const RGBA16UI: GLenum = 0x8D76;
#[allow(dead_code, non_upper_case_globals)]
pub const RGBA16_SNORM: GLenum = 0x8F9B;
#[allow(dead_code, non_upper_case_globals)]
pub const RGBA2: GLenum = 0x8055;
#[allow(dead_code, non_upper_case_globals)]
pub const RGBA32F: GLenum = 0x8814;
#[allow(dead_code, non_upper_case_globals)]
pub const RGBA32I: GLenum = 0x8D82;
#[allow(dead_code, non_upper_case_globals)]
pub const RGBA32UI: GLenum = 0x8D70;
#[allow(dead_code, non_upper_case_globals)]
pub const RGBA4: GLenum = 0x8056;
#[allow(dead_code, non_upper_case_globals)]
pub const RGBA8: GLenum = 0x8058;
#[allow(dead_code, non_upper_case_globals)]
pub const RGBA8I: GLenum = 0x8D8E;
#[allow(dead_code, non_upper_case_globals)]
pub const RGBA8UI: GLenum = 0x8D7C;
#[allow(dead_code, non_upper_case_globals)]
pub const RGBA8_SNORM: GLenum = 0x8F97;
#[allow(dead_code, non_upper_case_globals)]
pub const RGBA_INTEGER: GLenum = 0x8D99;
#[allow(dead_code, non_upper_case_globals)]
pub const RGBA_MODE: GLenum = 0x0C31;
#[allow(dead_code, non_upper_case_globals)]
pub const RGB_INTEGER: GLenum = 0x8D98;
#[allow(dead_code, non_upper_case_globals)]
pub const RGB_SCALE: GLenum = 0x8573;
#[allow(dead_code, non_upper_case_globals)]
pub const RG_INTEGER: GLenum = 0x8228;
#[allow(dead_code, non_upper_case_globals)]
pub const RIGHT: GLenum = 0x0407;
#[allow(dead_code, non_upper_case_globals)]
pub const S: GLenum = 0x2000;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLER: GLenum = 0x82E6;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLER_1D: GLenum = 0x8B5D;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLER_1D_ARRAY: GLenum = 0x8DC0;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLER_1D_ARRAY_SHADOW: GLenum = 0x8DC3;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLER_1D_SHADOW: GLenum = 0x8B61;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLER_2D: GLenum = 0x8B5E;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLER_2D_ARRAY: GLenum = 0x8DC1;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLER_2D_ARRAY_SHADOW: GLenum = 0x8DC4;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLER_2D_MULTISAMPLE: GLenum = 0x9108;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLER_2D_MULTISAMPLE_ARRAY: GLenum = 0x910B;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLER_2D_RECT: GLenum = 0x8B63;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLER_2D_RECT_SHADOW: GLenum = 0x8B64;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLER_2D_SHADOW: GLenum = 0x8B62;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLER_3D: GLenum = 0x8B5F;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLER_BINDING: GLenum = 0x8919;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLER_BUFFER: GLenum = 0x8DC2;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLER_CUBE: GLenum = 0x8B60;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLER_CUBE_MAP_ARRAY: GLenum = 0x900C;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLER_CUBE_MAP_ARRAY_SHADOW: GLenum = 0x900D;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLER_CUBE_SHADOW: GLenum = 0x8DC5;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLES: GLenum = 0x80A9;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLES_PASSED: GLenum = 0x8914;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLE_ALPHA_TO_COVERAGE: GLenum = 0x809E;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLE_ALPHA_TO_ONE: GLenum = 0x809F;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLE_BUFFERS: GLenum = 0x80A8;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLE_COVERAGE: GLenum = 0x80A0;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLE_COVERAGE_INVERT: GLenum = 0x80AB;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLE_COVERAGE_VALUE: GLenum = 0x80AA;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLE_MASK: GLenum = 0x8E51;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLE_MASK_VALUE: GLenum = 0x8E52;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLE_POSITION: GLenum = 0x8E50;
#[allow(dead_code, non_upper_case_globals)]
pub const SAMPLE_SHADING: GLenum = 0x8C36;
#[allow(dead_code, non_upper_case_globals)]
pub const SCISSOR_BIT: GLenum = 0x00080000;
#[allow(dead_code, non_upper_case_globals)]
pub const SCISSOR_BOX: GLenum = 0x0C10;
#[allow(dead_code, non_upper_case_globals)]
pub const SCISSOR_TEST: GLenum = 0x0C11;
#[allow(dead_code, non_upper_case_globals)]
pub const SECONDARY_COLOR_ARRAY: GLenum = 0x845E;
#[allow(dead_code, non_upper_case_globals)]
pub const SECONDARY_COLOR_ARRAY_BUFFER_BINDING: GLenum = 0x889C;
#[allow(dead_code, non_upper_case_globals)]
pub const SECONDARY_COLOR_ARRAY_POINTER: GLenum = 0x845D;
#[allow(dead_code, non_upper_case_globals)]
pub const SECONDARY_COLOR_ARRAY_SIZE: GLenum = 0x845A;
#[allow(dead_code, non_upper_case_globals)]
pub const SECONDARY_COLOR_ARRAY_STRIDE: GLenum = 0x845C;
#[allow(dead_code, non_upper_case_globals)]
pub const SECONDARY_COLOR_ARRAY_TYPE: GLenum = 0x845B;
#[allow(dead_code, non_upper_case_globals)]
pub const SELECT: GLenum = 0x1C02;
#[allow(dead_code, non_upper_case_globals)]
pub const SELECTION_BUFFER_POINTER: GLenum = 0x0DF3;
#[allow(dead_code, non_upper_case_globals)]
pub const SELECTION_BUFFER_SIZE: GLenum = 0x0DF4;
#[allow(dead_code, non_upper_case_globals)]
pub const SEPARATE_ATTRIBS: GLenum = 0x8C8D;
#[allow(dead_code, non_upper_case_globals)]
pub const SEPARATE_SPECULAR_COLOR: GLenum = 0x81FA;
#[allow(dead_code, non_upper_case_globals)]
pub const SET: GLenum = 0x150F;
#[allow(dead_code, non_upper_case_globals)]
pub const SHADER: GLenum = 0x82E1;
#[allow(dead_code, non_upper_case_globals)]
pub const SHADER_BINARY_FORMATS: GLenum = 0x8DF8;
#[allow(dead_code, non_upper_case_globals)]
pub const SHADER_COMPILER: GLenum = 0x8DFA;
#[allow(dead_code, non_upper_case_globals)]
pub const SHADER_IMAGE_ACCESS_BARRIER_BIT: GLenum = 0x00000020;
#[allow(dead_code, non_upper_case_globals)]
pub const SHADER_IMAGE_ATOMIC: GLenum = 0x82A6;
#[allow(dead_code, non_upper_case_globals)]
pub const SHADER_IMAGE_LOAD: GLenum = 0x82A4;
#[allow(dead_code, non_upper_case_globals)]
pub const SHADER_IMAGE_STORE: GLenum = 0x82A5;
#[allow(dead_code, non_upper_case_globals)]
pub const SHADER_SOURCE_LENGTH: GLenum = 0x8B88;
#[allow(dead_code, non_upper_case_globals)]
pub const SHADER_STORAGE_BARRIER_BIT: GLenum = 0x00002000;
#[allow(dead_code, non_upper_case_globals)]
pub const SHADER_STORAGE_BLOCK: GLenum = 0x92E6;
#[allow(dead_code, non_upper_case_globals)]
pub const SHADER_STORAGE_BUFFER: GLenum = 0x90D2;
#[allow(dead_code, non_upper_case_globals)]
pub const SHADER_STORAGE_BUFFER_BINDING: GLenum = 0x90D3;
#[allow(dead_code, non_upper_case_globals)]
pub const SHADER_STORAGE_BUFFER_OFFSET_ALIGNMENT: GLenum = 0x90DF;
#[allow(dead_code, non_upper_case_globals)]
pub const SHADER_STORAGE_BUFFER_SIZE: GLenum = 0x90D5;
#[allow(dead_code, non_upper_case_globals)]
pub const SHADER_STORAGE_BUFFER_START: GLenum = 0x90D4;
#[allow(dead_code, non_upper_case_globals)]
pub const SHADER_TYPE: GLenum = 0x8B4F;
#[allow(dead_code, non_upper_case_globals)]
pub const SHADE_MODEL: GLenum = 0x0B54;
#[allow(dead_code, non_upper_case_globals)]
pub const SHADING_LANGUAGE_VERSION: GLenum = 0x8B8C;
#[allow(dead_code, non_upper_case_globals)]
pub const SHININESS: GLenum = 0x1601;
#[allow(dead_code, non_upper_case_globals)]
pub const SHORT: GLenum = 0x1402;
#[allow(dead_code, non_upper_case_globals)]
pub const SIGNALED: GLenum = 0x9119;
#[allow(dead_code, non_upper_case_globals)]
pub const SIGNED_NORMALIZED: GLenum = 0x8F9C;
#[allow(dead_code, non_upper_case_globals)]
pub const SIMULTANEOUS_TEXTURE_AND_DEPTH_TEST: GLenum = 0x82AC;
#[allow(dead_code, non_upper_case_globals)]
pub const SIMULTANEOUS_TEXTURE_AND_DEPTH_WRITE: GLenum = 0x82AE;
#[allow(dead_code, non_upper_case_globals)]
pub const SIMULTANEOUS_TEXTURE_AND_STENCIL_TEST: GLenum = 0x82AD;
#[allow(dead_code, non_upper_case_globals)]
pub const SIMULTANEOUS_TEXTURE_AND_STENCIL_WRITE: GLenum = 0x82AF;
#[allow(dead_code, non_upper_case_globals)]
pub const SINGLE_COLOR: GLenum = 0x81F9;
#[allow(dead_code, non_upper_case_globals)]
pub const SLUMINANCE: GLenum = 0x8C46;
#[allow(dead_code, non_upper_case_globals)]
pub const SLUMINANCE8: GLenum = 0x8C47;
#[allow(dead_code, non_upper_case_globals)]
pub const SLUMINANCE8_ALPHA8: GLenum = 0x8C45;
#[allow(dead_code, non_upper_case_globals)]
pub const SLUMINANCE_ALPHA: GLenum = 0x8C44;
#[allow(dead_code, non_upper_case_globals)]
pub const SMOOTH: GLenum = 0x1D01;
#[allow(dead_code, non_upper_case_globals)]
pub const SMOOTH_LINE_WIDTH_GRANULARITY: GLenum = 0x0B23;
#[allow(dead_code, non_upper_case_globals)]
pub const SMOOTH_LINE_WIDTH_RANGE: GLenum = 0x0B22;
#[allow(dead_code, non_upper_case_globals)]
pub const SMOOTH_POINT_SIZE_GRANULARITY: GLenum = 0x0B13;
#[allow(dead_code, non_upper_case_globals)]
pub const SMOOTH_POINT_SIZE_RANGE: GLenum = 0x0B12;
#[allow(dead_code, non_upper_case_globals)]
pub const SOURCE0_ALPHA: GLenum = 0x8588;
#[allow(dead_code, non_upper_case_globals)]
pub const SOURCE0_RGB: GLenum = 0x8580;
#[allow(dead_code, non_upper_case_globals)]
pub const SOURCE1_ALPHA: GLenum = 0x8589;
#[allow(dead_code, non_upper_case_globals)]
pub const SOURCE1_RGB: GLenum = 0x8581;
#[allow(dead_code, non_upper_case_globals)]
pub const SOURCE2_ALPHA: GLenum = 0x858A;
#[allow(dead_code, non_upper_case_globals)]
pub const SOURCE2_RGB: GLenum = 0x8582;
#[allow(dead_code, non_upper_case_globals)]
pub const SPECULAR: GLenum = 0x1202;
#[allow(dead_code, non_upper_case_globals)]
pub const SPHERE_MAP: GLenum = 0x2402;
#[allow(dead_code, non_upper_case_globals)]
pub const SPOT_CUTOFF: GLenum = 0x1206;
#[allow(dead_code, non_upper_case_globals)]
pub const SPOT_DIRECTION: GLenum = 0x1204;
#[allow(dead_code, non_upper_case_globals)]
pub const SPOT_EXPONENT: GLenum = 0x1205;
#[allow(dead_code, non_upper_case_globals)]
pub const SRC0_ALPHA: GLenum = 0x8588;
#[allow(dead_code, non_upper_case_globals)]
pub const SRC0_RGB: GLenum = 0x8580;
#[allow(dead_code, non_upper_case_globals)]
pub const SRC1_ALPHA: GLenum = 0x8589;
#[allow(dead_code, non_upper_case_globals)]
pub const SRC1_COLOR: GLenum = 0x88F9;
#[allow(dead_code, non_upper_case_globals)]
pub const SRC1_RGB: GLenum = 0x8581;
#[allow(dead_code, non_upper_case_globals)]
pub const SRC2_ALPHA: GLenum = 0x858A;
#[allow(dead_code, non_upper_case_globals)]
pub const SRC2_RGB: GLenum = 0x8582;
#[allow(dead_code, non_upper_case_globals)]
pub const SRC_ALPHA: GLenum = 0x0302;
#[allow(dead_code, non_upper_case_globals)]
pub const SRC_ALPHA_SATURATE: GLenum = 0x0308;
#[allow(dead_code, non_upper_case_globals)]
pub const SRC_COLOR: GLenum = 0x0300;
#[allow(dead_code, non_upper_case_globals)]
pub const SRGB: GLenum = 0x8C40;
#[allow(dead_code, non_upper_case_globals)]
pub const SRGB8: GLenum = 0x8C41;
#[allow(dead_code, non_upper_case_globals)]
pub const SRGB8_ALPHA8: GLenum = 0x8C43;
#[allow(dead_code, non_upper_case_globals)]
pub const SRGB_ALPHA: GLenum = 0x8C42;
#[allow(dead_code, non_upper_case_globals)]
pub const SRGB_READ: GLenum = 0x8297;
#[allow(dead_code, non_upper_case_globals)]
pub const SRGB_WRITE: GLenum = 0x8298;
#[allow(dead_code, non_upper_case_globals)]
pub const STACK_OVERFLOW: GLenum = 0x0503;
#[allow(dead_code, non_upper_case_globals)]
pub const STACK_UNDERFLOW: GLenum = 0x0504;
#[allow(dead_code, non_upper_case_globals)]
pub const STATIC_COPY: GLenum = 0x88E6;
#[allow(dead_code, non_upper_case_globals)]
pub const STATIC_DRAW: GLenum = 0x88E4;
#[allow(dead_code, non_upper_case_globals)]
pub const STATIC_READ: GLenum = 0x88E5;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL: GLenum = 0x1802;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_ATTACHMENT: GLenum = 0x8D20;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_BACK_FAIL: GLenum = 0x8801;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_BACK_FUNC: GLenum = 0x8800;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_BACK_PASS_DEPTH_FAIL: GLenum = 0x8802;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_BACK_PASS_DEPTH_PASS: GLenum = 0x8803;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_BACK_REF: GLenum = 0x8CA3;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_BACK_VALUE_MASK: GLenum = 0x8CA4;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_BACK_WRITEMASK: GLenum = 0x8CA5;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_BITS: GLenum = 0x0D57;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_BUFFER_BIT: GLenum = 0x00000400;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_CLEAR_VALUE: GLenum = 0x0B91;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_COMPONENTS: GLenum = 0x8285;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_FAIL: GLenum = 0x0B94;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_FUNC: GLenum = 0x0B92;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_INDEX: GLenum = 0x1901;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_INDEX1: GLenum = 0x8D46;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_INDEX16: GLenum = 0x8D49;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_INDEX4: GLenum = 0x8D47;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_INDEX8: GLenum = 0x8D48;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_PASS_DEPTH_FAIL: GLenum = 0x0B95;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_PASS_DEPTH_PASS: GLenum = 0x0B96;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_REF: GLenum = 0x0B97;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_RENDERABLE: GLenum = 0x8288;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_TEST: GLenum = 0x0B90;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_VALUE_MASK: GLenum = 0x0B93;
#[allow(dead_code, non_upper_case_globals)]
pub const STENCIL_WRITEMASK: GLenum = 0x0B98;
#[allow(dead_code, non_upper_case_globals)]
pub const STEREO: GLenum = 0x0C33;
#[allow(dead_code, non_upper_case_globals)]
pub const STREAM_COPY: GLenum = 0x88E2;
#[allow(dead_code, non_upper_case_globals)]
pub const STREAM_DRAW: GLenum = 0x88E0;
#[allow(dead_code, non_upper_case_globals)]
pub const STREAM_READ: GLenum = 0x88E1;
#[allow(dead_code, non_upper_case_globals)]
pub const SUBPIXEL_BITS: GLenum = 0x0D50;
#[allow(dead_code, non_upper_case_globals)]
pub const SUBTRACT: GLenum = 0x84E7;
#[allow(dead_code, non_upper_case_globals)]
pub const SYNC_CONDITION: GLenum = 0x9113;
#[allow(dead_code, non_upper_case_globals)]
pub const SYNC_FENCE: GLenum = 0x9116;
#[allow(dead_code, non_upper_case_globals)]
pub const SYNC_FLAGS: GLenum = 0x9115;
#[allow(dead_code, non_upper_case_globals)]
pub const SYNC_FLUSH_COMMANDS_BIT: GLenum = 0x00000001;
#[allow(dead_code, non_upper_case_globals)]
pub const SYNC_GPU_COMMANDS_COMPLETE: GLenum = 0x9117;
#[allow(dead_code, non_upper_case_globals)]
pub const SYNC_STATUS: GLenum = 0x9114;
#[allow(dead_code, non_upper_case_globals)]
pub const T: GLenum = 0x2001;
#[allow(dead_code, non_upper_case_globals)]
pub const T2F_C3F_V3F: GLenum = 0x2A2A;
#[allow(dead_code, non_upper_case_globals)]
pub const T2F_C4F_N3F_V3F: GLenum = 0x2A2C;
#[allow(dead_code, non_upper_case_globals)]
pub const T2F_C4UB_V3F: GLenum = 0x2A29;
#[allow(dead_code, non_upper_case_globals)]
pub const T2F_N3F_V3F: GLenum = 0x2A2B;
#[allow(dead_code, non_upper_case_globals)]
pub const T2F_V3F: GLenum = 0x2A27;
#[allow(dead_code, non_upper_case_globals)]
pub const T4F_C4F_N3F_V4F: GLenum = 0x2A2D;
#[allow(dead_code, non_upper_case_globals)]
pub const T4F_V4F: GLenum = 0x2A28;
#[allow(dead_code, non_upper_case_globals)]
pub const TABLE_TOO_LARGE: GLenum = 0x8031;
#[allow(dead_code, non_upper_case_globals)]
pub const TESS_CONTROL_OUTPUT_VERTICES: GLenum = 0x8E75;
#[allow(dead_code, non_upper_case_globals)]
pub const TESS_CONTROL_SHADER: GLenum = 0x8E88;
#[allow(dead_code, non_upper_case_globals)]
pub const TESS_CONTROL_SHADER_BIT: GLenum = 0x00000008;
#[allow(dead_code, non_upper_case_globals)]
pub const TESS_CONTROL_SUBROUTINE: GLenum = 0x92E9;
#[allow(dead_code, non_upper_case_globals)]
pub const TESS_CONTROL_SUBROUTINE_UNIFORM: GLenum = 0x92EF;
#[allow(dead_code, non_upper_case_globals)]
pub const TESS_CONTROL_TEXTURE: GLenum = 0x829C;
#[allow(dead_code, non_upper_case_globals)]
pub const TESS_EVALUATION_SHADER: GLenum = 0x8E87;
#[allow(dead_code, non_upper_case_globals)]
pub const TESS_EVALUATION_SHADER_BIT: GLenum = 0x00000010;
#[allow(dead_code, non_upper_case_globals)]
pub const TESS_EVALUATION_SUBROUTINE: GLenum = 0x92EA;
#[allow(dead_code, non_upper_case_globals)]
pub const TESS_EVALUATION_SUBROUTINE_UNIFORM: GLenum = 0x92F0;
#[allow(dead_code, non_upper_case_globals)]
pub const TESS_EVALUATION_TEXTURE: GLenum = 0x829D;
#[allow(dead_code, non_upper_case_globals)]
pub const TESS_GEN_MODE: GLenum = 0x8E76;
#[allow(dead_code, non_upper_case_globals)]
pub const TESS_GEN_POINT_MODE: GLenum = 0x8E79;
#[allow(dead_code, non_upper_case_globals)]
pub const TESS_GEN_SPACING: GLenum = 0x8E77;
#[allow(dead_code, non_upper_case_globals)]
pub const TESS_GEN_VERTEX_ORDER: GLenum = 0x8E78;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE: GLenum = 0x1702;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE0: GLenum = 0x84C0;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE1: GLenum = 0x84C1;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE10: GLenum = 0x84CA;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE11: GLenum = 0x84CB;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE12: GLenum = 0x84CC;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE13: GLenum = 0x84CD;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE14: GLenum = 0x84CE;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE15: GLenum = 0x84CF;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE16: GLenum = 0x84D0;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE17: GLenum = 0x84D1;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE18: GLenum = 0x84D2;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE19: GLenum = 0x84D3;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE2: GLenum = 0x84C2;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE20: GLenum = 0x84D4;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE21: GLenum = 0x84D5;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE22: GLenum = 0x84D6;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE23: GLenum = 0x84D7;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE24: GLenum = 0x84D8;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE25: GLenum = 0x84D9;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE26: GLenum = 0x84DA;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE27: GLenum = 0x84DB;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE28: GLenum = 0x84DC;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE29: GLenum = 0x84DD;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE3: GLenum = 0x84C3;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE30: GLenum = 0x84DE;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE31: GLenum = 0x84DF;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE4: GLenum = 0x84C4;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE5: GLenum = 0x84C5;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE6: GLenum = 0x84C6;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE7: GLenum = 0x84C7;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE8: GLenum = 0x84C8;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE9: GLenum = 0x84C9;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_1D: GLenum = 0x0DE0;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_1D_ARRAY: GLenum = 0x8C18;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_2D: GLenum = 0x0DE1;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_2D_ARRAY: GLenum = 0x8C1A;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_2D_MULTISAMPLE: GLenum = 0x9100;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_2D_MULTISAMPLE_ARRAY: GLenum = 0x9102;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_3D: GLenum = 0x806F;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_ALPHA_SIZE: GLenum = 0x805F;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_ALPHA_TYPE: GLenum = 0x8C13;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_BASE_LEVEL: GLenum = 0x813C;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_BINDING_1D: GLenum = 0x8068;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_BINDING_1D_ARRAY: GLenum = 0x8C1C;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_BINDING_2D: GLenum = 0x8069;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_BINDING_2D_ARRAY: GLenum = 0x8C1D;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_BINDING_2D_MULTISAMPLE: GLenum = 0x9104;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY: GLenum = 0x9105;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_BINDING_3D: GLenum = 0x806A;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_BINDING_BUFFER: GLenum = 0x8C2C;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_BINDING_CUBE_MAP: GLenum = 0x8514;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_BINDING_CUBE_MAP_ARRAY: GLenum = 0x900A;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_BINDING_RECTANGLE: GLenum = 0x84F6;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_BIT: GLenum = 0x00040000;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_BLUE_SIZE: GLenum = 0x805E;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_BLUE_TYPE: GLenum = 0x8C12;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_BORDER: GLenum = 0x1005;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_BORDER_COLOR: GLenum = 0x1004;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_BUFFER: GLenum = 0x8C2A;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_BUFFER_BINDING: GLenum = 0x8C2A;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_BUFFER_DATA_STORE_BINDING: GLenum = 0x8C2D;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_BUFFER_OFFSET: GLenum = 0x919D;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_BUFFER_OFFSET_ALIGNMENT: GLenum = 0x919F;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_BUFFER_SIZE: GLenum = 0x919E;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_COMPARE_FUNC: GLenum = 0x884D;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_COMPARE_MODE: GLenum = 0x884C;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_COMPONENTS: GLenum = 0x1003;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_COMPRESSED: GLenum = 0x86A1;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_COMPRESSED_BLOCK_HEIGHT: GLenum = 0x82B2;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_COMPRESSED_BLOCK_SIZE: GLenum = 0x82B3;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_COMPRESSED_BLOCK_WIDTH: GLenum = 0x82B1;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_COMPRESSED_IMAGE_SIZE: GLenum = 0x86A0;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_COMPRESSION_HINT: GLenum = 0x84EF;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_COORD_ARRAY: GLenum = 0x8078;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_COORD_ARRAY_BUFFER_BINDING: GLenum = 0x889A;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_COORD_ARRAY_POINTER: GLenum = 0x8092;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_COORD_ARRAY_SIZE: GLenum = 0x8088;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_COORD_ARRAY_STRIDE: GLenum = 0x808A;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_COORD_ARRAY_TYPE: GLenum = 0x8089;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_CUBE_MAP: GLenum = 0x8513;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_CUBE_MAP_ARRAY: GLenum = 0x9009;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_CUBE_MAP_NEGATIVE_X: GLenum = 0x8516;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_CUBE_MAP_NEGATIVE_Y: GLenum = 0x8518;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_CUBE_MAP_NEGATIVE_Z: GLenum = 0x851A;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_CUBE_MAP_POSITIVE_X: GLenum = 0x8515;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_CUBE_MAP_POSITIVE_Y: GLenum = 0x8517;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_CUBE_MAP_POSITIVE_Z: GLenum = 0x8519;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_CUBE_MAP_SEAMLESS: GLenum = 0x884F;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_DEPTH: GLenum = 0x8071;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_DEPTH_SIZE: GLenum = 0x884A;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_DEPTH_TYPE: GLenum = 0x8C16;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_ENV: GLenum = 0x2300;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_ENV_COLOR: GLenum = 0x2201;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_ENV_MODE: GLenum = 0x2200;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_FETCH_BARRIER_BIT: GLenum = 0x00000008;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_FILTER_CONTROL: GLenum = 0x8500;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_FIXED_SAMPLE_LOCATIONS: GLenum = 0x9107;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_GATHER: GLenum = 0x82A2;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_GATHER_SHADOW: GLenum = 0x82A3;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_GEN_MODE: GLenum = 0x2500;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_GEN_Q: GLenum = 0x0C63;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_GEN_R: GLenum = 0x0C62;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_GEN_S: GLenum = 0x0C60;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_GEN_T: GLenum = 0x0C61;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_GREEN_SIZE: GLenum = 0x805D;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_GREEN_TYPE: GLenum = 0x8C11;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_HEIGHT: GLenum = 0x1001;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_IMAGE_FORMAT: GLenum = 0x828F;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_IMAGE_TYPE: GLenum = 0x8290;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_IMMUTABLE_FORMAT: GLenum = 0x912F;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_IMMUTABLE_LEVELS: GLenum = 0x82DF;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_INTENSITY_SIZE: GLenum = 0x8061;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_INTENSITY_TYPE: GLenum = 0x8C15;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_INTERNAL_FORMAT: GLenum = 0x1003;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_LOD_BIAS: GLenum = 0x8501;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_LUMINANCE_SIZE: GLenum = 0x8060;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_LUMINANCE_TYPE: GLenum = 0x8C14;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_MAG_FILTER: GLenum = 0x2800;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_MATRIX: GLenum = 0x0BA8;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_MAX_LEVEL: GLenum = 0x813D;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_MAX_LOD: GLenum = 0x813B;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_MIN_FILTER: GLenum = 0x2801;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_MIN_LOD: GLenum = 0x813A;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_PRIORITY: GLenum = 0x8066;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_RECTANGLE: GLenum = 0x84F5;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_RED_SIZE: GLenum = 0x805C;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_RED_TYPE: GLenum = 0x8C10;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_RESIDENT: GLenum = 0x8067;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_SAMPLES: GLenum = 0x9106;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_SHADOW: GLenum = 0x82A1;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_SHARED_SIZE: GLenum = 0x8C3F;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_STACK_DEPTH: GLenum = 0x0BA5;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_STENCIL_SIZE: GLenum = 0x88F1;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_SWIZZLE_A: GLenum = 0x8E45;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_SWIZZLE_B: GLenum = 0x8E44;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_SWIZZLE_G: GLenum = 0x8E43;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_SWIZZLE_R: GLenum = 0x8E42;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_SWIZZLE_RGBA: GLenum = 0x8E46;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_TARGET: GLenum = 0x1006;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_UPDATE_BARRIER_BIT: GLenum = 0x00000100;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_VIEW: GLenum = 0x82B5;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_VIEW_MIN_LAYER: GLenum = 0x82DD;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_VIEW_MIN_LEVEL: GLenum = 0x82DB;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_VIEW_NUM_LAYERS: GLenum = 0x82DE;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_VIEW_NUM_LEVELS: GLenum = 0x82DC;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_WIDTH: GLenum = 0x1000;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_WRAP_R: GLenum = 0x8072;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_WRAP_S: GLenum = 0x2802;
#[allow(dead_code, non_upper_case_globals)]
pub const TEXTURE_WRAP_T: GLenum = 0x2803;
#[allow(dead_code, non_upper_case_globals)]
pub const TIMEOUT_EXPIRED: GLenum = 0x911B;
#[allow(dead_code, non_upper_case_globals)]
pub const TIMEOUT_IGNORED: GLuint64 = 0xFFFFFFFFFFFFFFFF;
#[allow(dead_code, non_upper_case_globals)]
pub const TIMESTAMP: GLenum = 0x8E28;
#[allow(dead_code, non_upper_case_globals)]
pub const TIME_ELAPSED: GLenum = 0x88BF;
#[allow(dead_code, non_upper_case_globals)]
pub const TOP_LEVEL_ARRAY_SIZE: GLenum = 0x930C;
#[allow(dead_code, non_upper_case_globals)]
pub const TOP_LEVEL_ARRAY_STRIDE: GLenum = 0x930D;
#[allow(dead_code, non_upper_case_globals)]
pub const TRANSFORM_BIT: GLenum = 0x00001000;
#[allow(dead_code, non_upper_case_globals)]
pub const TRANSFORM_FEEDBACK: GLenum = 0x8E22;
#[allow(dead_code, non_upper_case_globals)]
pub const TRANSFORM_FEEDBACK_ACTIVE: GLenum = 0x8E24;
#[allow(dead_code, non_upper_case_globals)]
pub const TRANSFORM_FEEDBACK_BARRIER_BIT: GLenum = 0x00000800;
#[allow(dead_code, non_upper_case_globals)]
pub const TRANSFORM_FEEDBACK_BINDING: GLenum = 0x8E25;
#[allow(dead_code, non_upper_case_globals)]
pub const TRANSFORM_FEEDBACK_BUFFER: GLenum = 0x8C8E;
#[allow(dead_code, non_upper_case_globals)]
pub const TRANSFORM_FEEDBACK_BUFFER_ACTIVE: GLenum = 0x8E24;
#[allow(dead_code, non_upper_case_globals)]
pub const TRANSFORM_FEEDBACK_BUFFER_BINDING: GLenum = 0x8C8F;
#[allow(dead_code, non_upper_case_globals)]
pub const TRANSFORM_FEEDBACK_BUFFER_INDEX: GLenum = 0x934B;
#[allow(dead_code, non_upper_case_globals)]
pub const TRANSFORM_FEEDBACK_BUFFER_MODE: GLenum = 0x8C7F;
#[allow(dead_code, non_upper_case_globals)]
pub const TRANSFORM_FEEDBACK_BUFFER_PAUSED: GLenum = 0x8E23;
#[allow(dead_code, non_upper_case_globals)]
pub const TRANSFORM_FEEDBACK_BUFFER_SIZE: GLenum = 0x8C85;
#[allow(dead_code, non_upper_case_globals)]
pub const TRANSFORM_FEEDBACK_BUFFER_START: GLenum = 0x8C84;
#[allow(dead_code, non_upper_case_globals)]
pub const TRANSFORM_FEEDBACK_BUFFER_STRIDE: GLenum = 0x934C;
#[allow(dead_code, non_upper_case_globals)]
pub const TRANSFORM_FEEDBACK_PAUSED: GLenum = 0x8E23;
#[allow(dead_code, non_upper_case_globals)]
pub const TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN: GLenum = 0x8C88;
#[allow(dead_code, non_upper_case_globals)]
pub const TRANSFORM_FEEDBACK_VARYING: GLenum = 0x92F4;
#[allow(dead_code, non_upper_case_globals)]
pub const TRANSFORM_FEEDBACK_VARYINGS: GLenum = 0x8C83;
#[allow(dead_code, non_upper_case_globals)]
pub const TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH: GLenum = 0x8C76;
#[allow(dead_code, non_upper_case_globals)]
pub const TRANSPOSE_COLOR_MATRIX: GLenum = 0x84E6;
#[allow(dead_code, non_upper_case_globals)]
pub const TRANSPOSE_MODELVIEW_MATRIX: GLenum = 0x84E3;
#[allow(dead_code, non_upper_case_globals)]
pub const TRANSPOSE_PROJECTION_MATRIX: GLenum = 0x84E4;
#[allow(dead_code, non_upper_case_globals)]
pub const TRANSPOSE_TEXTURE_MATRIX: GLenum = 0x84E5;
#[allow(dead_code, non_upper_case_globals)]
pub const TRIANGLES: GLenum = 0x0004;
#[allow(dead_code, non_upper_case_globals)]
pub const TRIANGLES_ADJACENCY: GLenum = 0x000C;
#[allow(dead_code, non_upper_case_globals)]
pub const TRIANGLE_FAN: GLenum = 0x0006;
#[allow(dead_code, non_upper_case_globals)]
pub const TRIANGLE_STRIP: GLenum = 0x0005;
#[allow(dead_code, non_upper_case_globals)]
pub const TRIANGLE_STRIP_ADJACENCY: GLenum = 0x000D;
#[allow(dead_code, non_upper_case_globals)]
pub const TRUE: GLboolean = 1;
#[allow(dead_code, non_upper_case_globals)]
pub const TYPE: GLenum = 0x92FA;
#[allow(dead_code, non_upper_case_globals)]
pub const UNDEFINED_VERTEX: GLenum = 0x8260;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM: GLenum = 0x92E1;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_ARRAY_STRIDE: GLenum = 0x8A3C;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_ATOMIC_COUNTER_BUFFER_INDEX: GLenum = 0x92DA;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_BARRIER_BIT: GLenum = 0x00000004;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_BLOCK: GLenum = 0x92E2;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_BLOCK_ACTIVE_UNIFORMS: GLenum = 0x8A42;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES: GLenum = 0x8A43;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_BLOCK_BINDING: GLenum = 0x8A3F;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_BLOCK_DATA_SIZE: GLenum = 0x8A40;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_BLOCK_INDEX: GLenum = 0x8A3A;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_BLOCK_NAME_LENGTH: GLenum = 0x8A41;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_BLOCK_REFERENCED_BY_COMPUTE_SHADER: GLenum = 0x90EC;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER: GLenum = 0x8A46;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_BLOCK_REFERENCED_BY_GEOMETRY_SHADER: GLenum = 0x8A45;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_BLOCK_REFERENCED_BY_TESS_CONTROL_SHADER: GLenum = 0x84F0;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_BLOCK_REFERENCED_BY_TESS_EVALUATION_SHADER: GLenum = 0x84F1;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER: GLenum = 0x8A44;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_BUFFER: GLenum = 0x8A11;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_BUFFER_BINDING: GLenum = 0x8A28;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_BUFFER_OFFSET_ALIGNMENT: GLenum = 0x8A34;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_BUFFER_SIZE: GLenum = 0x8A2A;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_BUFFER_START: GLenum = 0x8A29;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_IS_ROW_MAJOR: GLenum = 0x8A3E;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_MATRIX_STRIDE: GLenum = 0x8A3D;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_NAME_LENGTH: GLenum = 0x8A39;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_OFFSET: GLenum = 0x8A3B;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_SIZE: GLenum = 0x8A38;
#[allow(dead_code, non_upper_case_globals)]
pub const UNIFORM_TYPE: GLenum = 0x8A37;
#[allow(dead_code, non_upper_case_globals)]
pub const UNKNOWN_CONTEXT_RESET: GLenum = 0x8255;
#[allow(dead_code, non_upper_case_globals)]
pub const UNPACK_ALIGNMENT: GLenum = 0x0CF5;
#[allow(dead_code, non_upper_case_globals)]
pub const UNPACK_COMPRESSED_BLOCK_DEPTH: GLenum = 0x9129;
#[allow(dead_code, non_upper_case_globals)]
pub const UNPACK_COMPRESSED_BLOCK_HEIGHT: GLenum = 0x9128;
#[allow(dead_code, non_upper_case_globals)]
pub const UNPACK_COMPRESSED_BLOCK_SIZE: GLenum = 0x912A;
#[allow(dead_code, non_upper_case_globals)]
pub const UNPACK_COMPRESSED_BLOCK_WIDTH: GLenum = 0x9127;
#[allow(dead_code, non_upper_case_globals)]
pub const UNPACK_IMAGE_HEIGHT: GLenum = 0x806E;
#[allow(dead_code, non_upper_case_globals)]
pub const UNPACK_LSB_FIRST: GLenum = 0x0CF1;
#[allow(dead_code, non_upper_case_globals)]
pub const UNPACK_ROW_LENGTH: GLenum = 0x0CF2;
#[allow(dead_code, non_upper_case_globals)]
pub const UNPACK_SKIP_IMAGES: GLenum = 0x806D;
#[allow(dead_code, non_upper_case_globals)]
pub const UNPACK_SKIP_PIXELS: GLenum = 0x0CF4;
#[allow(dead_code, non_upper_case_globals)]
pub const UNPACK_SKIP_ROWS: GLenum = 0x0CF3;
#[allow(dead_code, non_upper_case_globals)]
pub const UNPACK_SWAP_BYTES: GLenum = 0x0CF0;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNALED: GLenum = 0x9118;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_BYTE: GLenum = 0x1401;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_BYTE_2_3_3_REV: GLenum = 0x8362;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_BYTE_3_3_2: GLenum = 0x8032;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT: GLenum = 0x1405;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_10F_11F_11F_REV: GLenum = 0x8C3B;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_10_10_10_2: GLenum = 0x8036;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_24_8: GLenum = 0x84FA;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_2_10_10_10_REV: GLenum = 0x8368;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_5_9_9_9_REV: GLenum = 0x8C3E;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_8_8_8_8: GLenum = 0x8035;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_8_8_8_8_REV: GLenum = 0x8367;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_ATOMIC_COUNTER: GLenum = 0x92DB;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_IMAGE_1D: GLenum = 0x9062;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_IMAGE_1D_ARRAY: GLenum = 0x9068;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_IMAGE_2D: GLenum = 0x9063;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_IMAGE_2D_ARRAY: GLenum = 0x9069;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_IMAGE_2D_MULTISAMPLE: GLenum = 0x906B;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_IMAGE_2D_MULTISAMPLE_ARRAY: GLenum = 0x906C;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_IMAGE_2D_RECT: GLenum = 0x9065;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_IMAGE_3D: GLenum = 0x9064;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_IMAGE_BUFFER: GLenum = 0x9067;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_IMAGE_CUBE: GLenum = 0x9066;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_IMAGE_CUBE_MAP_ARRAY: GLenum = 0x906A;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_SAMPLER_1D: GLenum = 0x8DD1;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_SAMPLER_1D_ARRAY: GLenum = 0x8DD6;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_SAMPLER_2D: GLenum = 0x8DD2;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_SAMPLER_2D_ARRAY: GLenum = 0x8DD7;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE: GLenum = 0x910A;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE_ARRAY: GLenum = 0x910D;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_SAMPLER_2D_RECT: GLenum = 0x8DD5;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_SAMPLER_3D: GLenum = 0x8DD3;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_SAMPLER_BUFFER: GLenum = 0x8DD8;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_SAMPLER_CUBE: GLenum = 0x8DD4;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_SAMPLER_CUBE_MAP_ARRAY: GLenum = 0x900F;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_VEC2: GLenum = 0x8DC6;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_VEC3: GLenum = 0x8DC7;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_INT_VEC4: GLenum = 0x8DC8;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_NORMALIZED: GLenum = 0x8C17;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_SHORT: GLenum = 0x1403;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_SHORT_1_5_5_5_REV: GLenum = 0x8366;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_SHORT_4_4_4_4: GLenum = 0x8033;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_SHORT_4_4_4_4_REV: GLenum = 0x8365;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_SHORT_5_5_5_1: GLenum = 0x8034;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_SHORT_5_6_5: GLenum = 0x8363;
#[allow(dead_code, non_upper_case_globals)]
pub const UNSIGNED_SHORT_5_6_5_REV: GLenum = 0x8364;
#[allow(dead_code, non_upper_case_globals)]
pub const UPPER_LEFT: GLenum = 0x8CA2;
#[allow(dead_code, non_upper_case_globals)]
pub const V2F: GLenum = 0x2A20;
#[allow(dead_code, non_upper_case_globals)]
pub const V3F: GLenum = 0x2A21;
#[allow(dead_code, non_upper_case_globals)]
pub const VALIDATE_STATUS: GLenum = 0x8B83;
#[allow(dead_code, non_upper_case_globals)]
pub const VENDOR: GLenum = 0x1F00;
#[allow(dead_code, non_upper_case_globals)]
pub const VERSION: GLenum = 0x1F02;
#[allow(dead_code, non_upper_case_globals)]
pub const VERTEX_ARRAY: GLenum = 0x8074;
#[allow(dead_code, non_upper_case_globals)]
pub const VERTEX_ARRAY_BINDING: GLenum = 0x85B5;
#[allow(dead_code, non_upper_case_globals)]
pub const VERTEX_ARRAY_BUFFER_BINDING: GLenum = 0x8896;
#[allow(dead_code, non_upper_case_globals)]
pub const VERTEX_ARRAY_POINTER: GLenum = 0x808E;
#[allow(dead_code, non_upper_case_globals)]
pub const VERTEX_ARRAY_SIZE: GLenum = 0x807A;
#[allow(dead_code, non_upper_case_globals)]
pub const VERTEX_ARRAY_STRIDE: GLenum = 0x807C;
#[allow(dead_code, non_upper_case_globals)]
pub const VERTEX_ARRAY_TYPE: GLenum = 0x807B;
#[allow(dead_code, non_upper_case_globals)]
pub const VERTEX_ATTRIB_ARRAY_BARRIER_BIT: GLenum = 0x00000001;
#[allow(dead_code, non_upper_case_globals)]
pub const VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: GLenum = 0x889F;
#[allow(dead_code, non_upper_case_globals)]
pub const VERTEX_ATTRIB_ARRAY_DIVISOR: GLenum = 0x88FE;
#[allow(dead_code, non_upper_case_globals)]
pub const VERTEX_ATTRIB_ARRAY_ENABLED: GLenum = 0x8622;
#[allow(dead_code, non_upper_case_globals)]
pub const VERTEX_ATTRIB_ARRAY_INTEGER: GLenum = 0x88FD;
#[allow(dead_code, non_upper_case_globals)]
pub const VERTEX_ATTRIB_ARRAY_LONG: GLenum = 0x874E;
#[allow(dead_code, non_upper_case_globals)]
pub const VERTEX_ATTRIB_ARRAY_NORMALIZED: GLenum = 0x886A;
#[allow(dead_code, non_upper_case_globals)]
pub const VERTEX_ATTRIB_ARRAY_POINTER: GLenum = 0x8645;
#[allow(dead_code, non_upper_case_globals)]
pub const VERTEX_ATTRIB_ARRAY_SIZE: GLenum = 0x8623;
#[allow(dead_code, non_upper_case_globals)]
pub const VERTEX_ATTRIB_ARRAY_STRIDE: GLenum = 0x8624;
#[allow(dead_code, non_upper_case_globals)]
pub const VERTEX_ATTRIB_ARRAY_TYPE: GLenum = 0x8625;
#[allow(dead_code, non_upper_case_globals)]
pub const VERTEX_ATTRIB_BINDING: GLenum = 0x82D4;
#[allow(dead_code, non_upper_case_globals)]
pub const VERTEX_ATTRIB_RELATIVE_OFFSET: GLenum = 0x82D5;
#[allow(dead_code, non_upper_case_globals)]
pub const VERTEX_BINDING_BUFFER: GLenum = 0x8F4F;
#[allow(dead_code, non_upper_case_globals)]
pub const VERTEX_BINDING_DIVISOR: GLenum = 0x82D6;
#[allow(dead_code, non_upper_case_globals)]
pub const VERTEX_BINDING_OFFSET: GLenum = 0x82D7;
#[allow(dead_code, non_upper_case_globals)]
pub const VERTEX_BINDING_STRIDE: GLenum = 0x82D8;
#[allow(dead_code, non_upper_case_globals)]
pub const VERTEX_PROGRAM_POINT_SIZE: GLenum = 0x8642;
#[allow(dead_code, non_upper_case_globals)]
pub const VERTEX_PROGRAM_TWO_SIDE: GLenum = 0x8643;
#[allow(dead_code, non_upper_case_globals)]
pub const VERTEX_SHADER: GLenum = 0x8B31;
#[allow(dead_code, non_upper_case_globals)]
pub const VERTEX_SHADER_BIT: GLenum = 0x00000001;
#[allow(dead_code, non_upper_case_globals)]
pub const VERTEX_SUBROUTINE: GLenum = 0x92E8;
#[allow(dead_code, non_upper_case_globals)]
pub const VERTEX_SUBROUTINE_UNIFORM: GLenum = 0x92EE;
#[allow(dead_code, non_upper_case_globals)]
pub const VERTEX_TEXTURE: GLenum = 0x829B;
#[allow(dead_code, non_upper_case_globals)]
pub const VIEWPORT: GLenum = 0x0BA2;
#[allow(dead_code, non_upper_case_globals)]
pub const VIEWPORT_BIT: GLenum = 0x00000800;
#[allow(dead_code, non_upper_case_globals)]
pub const VIEWPORT_BOUNDS_RANGE: GLenum = 0x825D;
#[allow(dead_code, non_upper_case_globals)]
pub const VIEWPORT_INDEX_PROVOKING_VERTEX: GLenum = 0x825F;
#[allow(dead_code, non_upper_case_globals)]
pub const VIEWPORT_SUBPIXEL_BITS: GLenum = 0x825C;
#[allow(dead_code, non_upper_case_globals)]
pub const VIEW_CLASS_128_BITS: GLenum = 0x82C4;
#[allow(dead_code, non_upper_case_globals)]
pub const VIEW_CLASS_16_BITS: GLenum = 0x82CA;
#[allow(dead_code, non_upper_case_globals)]
pub const VIEW_CLASS_24_BITS: GLenum = 0x82C9;
#[allow(dead_code, non_upper_case_globals)]
pub const VIEW_CLASS_32_BITS: GLenum = 0x82C8;
#[allow(dead_code, non_upper_case_globals)]
pub const VIEW_CLASS_48_BITS: GLenum = 0x82C7;
#[allow(dead_code, non_upper_case_globals)]
pub const VIEW_CLASS_64_BITS: GLenum = 0x82C6;
#[allow(dead_code, non_upper_case_globals)]
pub const VIEW_CLASS_8_BITS: GLenum = 0x82CB;
#[allow(dead_code, non_upper_case_globals)]
pub const VIEW_CLASS_96_BITS: GLenum = 0x82C5;
#[allow(dead_code, non_upper_case_globals)]
pub const VIEW_CLASS_BPTC_FLOAT: GLenum = 0x82D3;
#[allow(dead_code, non_upper_case_globals)]
pub const VIEW_CLASS_BPTC_UNORM: GLenum = 0x82D2;
#[allow(dead_code, non_upper_case_globals)]
pub const VIEW_CLASS_RGTC1_RED: GLenum = 0x82D0;
#[allow(dead_code, non_upper_case_globals)]
pub const VIEW_CLASS_RGTC2_RG: GLenum = 0x82D1;
#[allow(dead_code, non_upper_case_globals)]
pub const VIEW_CLASS_S3TC_DXT1_RGB: GLenum = 0x82CC;
#[allow(dead_code, non_upper_case_globals)]
pub const VIEW_CLASS_S3TC_DXT1_RGBA: GLenum = 0x82CD;
#[allow(dead_code, non_upper_case_globals)]
pub const VIEW_CLASS_S3TC_DXT3_RGBA: GLenum = 0x82CE;
#[allow(dead_code, non_upper_case_globals)]
pub const VIEW_CLASS_S3TC_DXT5_RGBA: GLenum = 0x82CF;
#[allow(dead_code, non_upper_case_globals)]
pub const VIEW_COMPATIBILITY_CLASS: GLenum = 0x82B6;
#[allow(dead_code, non_upper_case_globals)]
pub const WAIT_FAILED: GLenum = 0x911D;
#[allow(dead_code, non_upper_case_globals)]
pub const WEIGHT_ARRAY_BUFFER_BINDING: GLenum = 0x889E;
#[allow(dead_code, non_upper_case_globals)]
pub const WRITE_ONLY: GLenum = 0x88B9;
#[allow(dead_code, non_upper_case_globals)]
pub const XOR: GLenum = 0x1506;
#[allow(dead_code, non_upper_case_globals)]
pub const ZERO: GLenum = 0;
#[allow(dead_code, non_upper_case_globals)]
pub const ZERO_TO_ONE: GLenum = 0x935F;
#[allow(dead_code, non_upper_case_globals)]
pub const ZOOM_X: GLenum = 0x0D16;
#[allow(dead_code, non_upper_case_globals)]
pub const ZOOM_Y: GLenum = 0x0D17;