use proc_macro::TokenStream;
use chrono::Utc;

/* No need to include a string for this */
#[proc_macro]
pub fn compile_time(_input: TokenStream) -> TokenStream {
    let build_time = Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string();
    format!(r#""{}".into()"#, build_time).parse().unwrap()
}
