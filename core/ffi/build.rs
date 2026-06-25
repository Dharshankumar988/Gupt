fn main() {
    uniffi_build::generate_scaffolding("./src/gupt.udl").expect("Building the UDL file failed");
}
