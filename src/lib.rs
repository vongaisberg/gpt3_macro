#![feature(proc_macro_diagnostic)]

extern crate proc_macro;

use proc_macro::{Diagnostic, Level, TokenStream};
use std::env;

use serde_json::*;

#[proc_macro]
pub fn gpt3(input: TokenStream) -> TokenStream {
    match env::var("OPENAI_KEY") {
        Ok(key) => {
            let input = input.into_iter().collect::<Vec<_>>();
            let prompt = &input[0].to_string();
            let prompt = &prompt[1..prompt.len() - 1]; // Remove leading and trailing quotation marks

            let mut code = String::new();

            for i in 1..input.len() {
                code += &input[i].to_string();
                code += " ";
            }
            let code = code.replace("- >", "->");

            let prompt = format!("// A rust function that {prompt}\\n{code}");
            println!("{prompt}\n\n\n");
            let response =
                ureq::post("https://api.openai.com/v1/engines/davinci-codex/completions")
                    .set("Content-Type", "application/json")
                    .set("Authorization", &format!("Bearer {key}"))
                    .send_string(&format!(
                        "{{
        \"prompt\": \"{prompt}\",
        \"temperature\": 0,
        \"max_tokens\": 320,
        \"top_p\": 1,
        \"frequency_penalty\": 0,
        \"best_of\": 3,
        \"presence_penalty\": 0
      }}"
                    ));

            match response {
                Ok(res) => {
                    let body: Value = serde_json::from_str(&res.into_string().unwrap()).unwrap();
                    let text = &body["choices"][0]["text"].to_string();
                    let text = &text[1..text.len() - 1]; // Remove leading and trailing quotation marks

                    println!("{text}\n\n\n");

                    // Only output everything until the end of the first braces-codeblock
                    let mut text_cut = String::new();
                    let mut brackets = 0;
                    let mut was_in_body = false;
                    for c in text.chars() {
                        if c == '{' {
                            brackets += 1;
                            was_in_body = true;
                        }
                        if c == '}' {
                            brackets -= 1;
                        }
                        text_cut.push(c);
                        if brackets == 0 && was_in_body {
                            break;
                        }
                    }

                    // Put the prompt in front of the result again
                    let text = prompt + &text_cut;

                    // Unescape
                    let text = text
                        .replace("\\n", "\n")
                        .replace("\\\"", "\"")
                        .replace("\\\'", "\'");
                    println!("{text}");

                    text.parse().unwrap()
                }
                Err(err) => {
                    Diagnostic::new(
                        Level::Error,
                        match err {
                            ureq::Error::Status(code, resp) => {
                                format!("Error while calling GPT3 {}: {}", code, resp.status_text())
                            }
                            ureq::Error::Transport(err) => err.to_string(),
                        },
                    )
                    .emit();
                    TokenStream::new()
                }
            }
        }
        Err(_) => {
            Diagnostic::new(
                Level::Error,
                "No OpenAI API key found. Please set the environment variable $OPENAI_KEY.",
            )
            .emit();
            TokenStream::new()
        }
    }
}
