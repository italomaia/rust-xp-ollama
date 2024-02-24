use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::Ollama;
use xp_ollama::consts::{DEFAULT_SYSTEM_MOCK, MODEL};
use xp_ollama::gen::gen_stream_print;
use xp_ollama::Result;


#[tokio::main]
async fn main() -> Result<()> {
    // defaults to localhost:11434
    let ollama = Ollama::default();
    let model = MODEL.to_string();
    let prompt = "What is the best programming language? (be concise)";

    let gen_req = GenerationRequest::new(model, prompt.to_string())
        .system(DEFAULT_SYSTEM_MOCK.to_string())
    ;

    println!("->> ??? {}", prompt);

    // single response generation
    // let res = ollama.generate(gen_req).await?;
    // println!("->> res {}", res.response);

    // -- stream response
    gen_stream_print(&ollama, gen_req).await?;
    
    Ok(())
}
