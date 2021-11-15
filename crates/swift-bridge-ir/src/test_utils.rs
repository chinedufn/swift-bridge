use proc_macro2::TokenStream;

pub fn assert_tokens_eq(left: &TokenStream, right: &TokenStream) {
    assert_eq!(
        token_stream_to_vec(&left),
        token_stream_to_vec(&right),
        r#"
Left Tokens:
{}

Right Tokens:
{}
"#,
        left.to_string(),
        right.to_string()
    )
}

fn token_stream_to_vec(tokens: &TokenStream) -> Vec<String> {
    tokens
        .clone()
        .into_iter()
        .map(|t| t.to_string().trim().to_string())
        .collect()
}
