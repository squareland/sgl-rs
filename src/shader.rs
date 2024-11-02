use crate::debug::GlError;
pub use crate::raw::shader::*;

use crate::state::GraphicsContext;
use crate::tessellator::Vertex;

#[macro_export]
macro_rules! uniform {
    ($structure: ident { $($name:ident : $ty:ty),+ }) => {
        pub struct $structure<'a, V> {
            $($name: $crate::shader::UniformLocation<'a, $ty, V>),+
        }

        impl<'a, V> $structure<'a, V> where V: $crate::tessellator::Vertex {
            pub fn new(program: &'a $crate::shader::LinkedProgramId<V>) -> std::result::Result<Self, $crate::shader::ProgramError> {
                Ok(Self {
                    $($name: {
                        const CNAME: &[u8] = concat!(stringify!($name), "\u{0000}").as_bytes();
                        program.uniform::<$ty>(
                            unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(CNAME) }
                        ).ok_or($crate::shader::ProgramError::UniformMissing(stringify!($name)))?
                    }),+
                })
            }
        }
    };
}

#[derive(Debug)]
pub enum ProgramError {
    ShaderCreation(GlError),
    ShaderSource(GlError),
    ShaderCompilation(String),
    ShaderAttach(GlError),
    ProgramCreation(GlError),
    ProgramLinkage(String),
    ProgramValidation(GlError),
    UniformMissing(&'static str),
    AttributeMissing(&'static str),
}

pub fn link_program<V: Vertex>(context: GraphicsContext, vert: &str, frag: &str) -> Result<LinkedProgramId<V>, ProgramError> {
    let vertex = context.create_shader(ShaderType::Vertex).map_err(ProgramError::ShaderCreation)?;
    vertex.source(vert).map_err(ProgramError::ShaderSource)?;
    vertex.compile().map_err(ProgramError::ShaderCompilation)?;

    let fragment = context.create_shader(ShaderType::Fragment).map_err(ProgramError::ShaderCreation)?;
    fragment.source(frag).map_err(ProgramError::ShaderSource)?;
    fragment.compile().map_err(ProgramError::ShaderCompilation)?;

    let program = context.create_program().map_err(ProgramError::ProgramCreation)?;
    program.attach(&vertex).map_err(ProgramError::ShaderAttach)?;
    program.attach(&fragment).map_err(ProgramError::ShaderAttach)?;
    let linked = program.link::<V>().map_err(ProgramError::ProgramLinkage)?;
    V::bind_attributes(&linked)?;
    let info = linked.validate().map_err(ProgramError::ProgramValidation)?;
    eprintln!("{}", info);
    Ok(linked)
}