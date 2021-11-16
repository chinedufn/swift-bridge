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

pub fn assert_tokens_contain(outer: &TokenStream, inner: &TokenStream) {
    let outer_vec = token_stream_to_vec(&outer);
    let inner_vec = token_stream_to_vec(inner);

    let is_contained = outer_vec
        .into_iter()
        .collect::<String>()
        .contains(&inner_vec.into_iter().collect::<String>());

    assert!(
        is_contained,
        r#"
Outer tokens do not contain the inner tokens. 

Outer Tokens:
{}

Inner Tokens:
{}
"#,
        outer.to_string(),
        inner.to_string()
    )
}

fn token_stream_to_vec(tokens: &TokenStream) -> Vec<String> {
    tokens
        .clone()
        .into_iter()
        .map(|t| t.to_string().trim().to_string())
        .collect()
}
