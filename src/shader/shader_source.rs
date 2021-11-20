/// Enables loading shaders from either a file or a hardcoded str
pub enum ShaderSource<'a> {
    File(&'a str),
    String(&'a str),
}
