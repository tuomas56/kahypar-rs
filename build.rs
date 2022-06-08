fn main() { 
    println!(r"cargo:rustc-link-search=native=C:\Program Files (x86)\KaHyPar\lib");
    println!("cargo:rustc-link-lib=dylib=libkahypar");
}
   