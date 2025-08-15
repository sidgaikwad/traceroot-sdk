use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, ItemFn, Lit, Meta, NestedMeta};

#[proc_macro_attribute]
pub fn traceroot_trace(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let mut span_name = "unnamed";
    let mut trace_params = false;

    for arg in args {
        if let NestedMeta::Meta(Meta::NameValue(nv)) = arg {
            if nv.path.is_ident("span_name") {
                if let Lit::Str(s) = nv.lit { span_name = &s.value(); }
            } else if nv.path.is_ident("trace_params") {
                if let Lit::Bool(b) = nv.lit { trace_params = b.value; }
            }
        }
    }

    let input_fn = parse_macro_input!(input as ItemFn);
    let vis = &input_fn.vis;
    let sig = &input_fn.sig;
    let block = &input_fn.block;

    let params = if trace_params {
        quote! { format!("{:?}", (#(&#sig.inputs),*)) }
    } else {
        quote! { String::new() }
    };

    let name_lit = span_name.to_string();

    let expanded = quote! {
        #vis #sig {
            let span = tracing::info_span!("traceroot", span_name = #name_lit.as_str());
            let _enter = span.enter();
            let _params = #params;
            if !_params.is_empty() {
                span.record("params", &tracing::field::display(_params));
            }
            #block
        }
    };
    TokenStream::from(expanded)
}
