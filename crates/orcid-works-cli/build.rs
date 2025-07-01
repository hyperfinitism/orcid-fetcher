fn main() {
    println!("cargo:rustc-env=APP_VERSION={}", env!("CARGO_PKG_VERSION"));
    println!("cargo:rustc-env=APP_NAME={}", env!("CARGO_PKG_NAME"));
    println!("cargo:rustc-env=CARGO_REPOSITORY_URL=https://github.com/hyperfinitism/orcid-fetcher");
}
