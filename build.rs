fn main() {
    let now = time::OffsetDateTime::now_utc();
    let build_date = format!("{}-{:02}-{:02}", now.year(), now.month() as u8, now.day());
    println!("cargo:rustc-env=BUILD_DATE={build_date}");
    println!("cargo:rerun-if-changed=build.rs");
}
