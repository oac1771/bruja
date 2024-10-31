use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn test(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;
    let fn_body = &input.block;

    let result = quote! {
        #[tokio::test]
        async fn #fn_name () {
            use ::tracing_subscriber::util::SubscriberInitExt;
            use ::std::sync::{Arc, Mutex};

            let log_buffer: Arc<Mutex<Vec<u8>>> = Arc::new(Mutex::new(Vec::new()));
            let buffer = log_buffer.clone();

            let _guard = tracing_subscriber::fmt()
                .json()
                .with_writer(move || ::tests::test_utils::BufferWriter {
                    buffer: buffer.clone(),
                })
                .set_default();

            #fn_body
        }
    };

    result.into()
}
