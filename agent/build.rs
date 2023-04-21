fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("../proto/echo/echo.proto")?;
    tonic_build::compile_protos("../proto/project/project.proto")?;
    Ok(())
}
