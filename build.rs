fn main() { 
    println!(r"cargo:rustc-link-search=C:\Program Files (x86)\KaHyPar\lib");
    println!("cargo:rustc-link-lib=dylib=libkahypar");
}
   