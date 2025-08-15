use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, Lit, Meta, MetaNameValue, NestedMeta};

#[proc_macro_attribute]
pub fn traceroot_trace(args: TokenStream, input: TokenStream) -> TokenStream {
    // Parse the attribute arguments as a comma-separated list
    let args_parsed = parse_macro_input!(args as syn::AttributeArgs);

    let mut span_name = String::from("unnamed");
    let mut trace_params = false;

    for arg in args_parsed {
        match arg {
            NestedMeta::Meta(Meta::NameValue(MetaNameValue { path, value, .. })) => {
                if path.is_ident("span_name") {
                    if let Lit::Str(s) = value {
                        span_name = s.value();
                    }
                } else if path.is_ident("trace_params") {
                    if let Lit::Bool(b) = value {
                        trace_params = b.value;
                    }
                }
            }
            _ => {}
        }
    }

    // Parse the function
    let input_fn = parse_macro_input!(input as ItemFn);
    let vis = &input_fn.vis;
    let sig = &input_fn.sig;
    let block = &input_fn.block;

    let params_code = if trace_params {
        // collect params as string
        quote! { format!("{:?}", (#(&#sig.inputs),*)) }
    } else {
        quote! { String::new() }
    };

    let expanded = quote! {
        #vis #sig {
            let span = tracing::info_span!("traceroot", span_name = #span_name.as_str());
            let _enter = span.enter();
            let _params = #params_code;
            if !_params.is_empty() {
                span.record("params", &tracing::field::display(_params));
            }
            #block
        }
    };

    TokenStream::from(expanded)
}
