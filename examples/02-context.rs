use ollama_rs::generation::completion::GenerationContext;
use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::Ollama;
use xp_ollama::consts::MODEL;
use xp_ollama::gen::gen_stream_print;
use xp_ollama::Result;


#[tokio::main]
async fn main() -> Result<()> {
    // defaults to localhost:11434
    let ollama = Ollama::default();
    let prompts = &[
        "Why the sky is read? (be concise)",
        "What was my first question?"
    ];

    let mut last_ctx: Option<GenerationContext> = None;
    
    for prompt in prompts {
        println!("->> ??? {}", prompt);
        let mut gen_req = GenerationRequest::new(MODEL.to_string(), prompt.to_string());
        
        if let Some(last_ctx) = last_ctx.take() {
            // if you do not provide context, the model will not know what was your last question
            gen_req = gen_req.context(last_ctx);
        }

        // -- stream response
        let final_data = gen_stream_print(&ollama, gen_req).await?;

        if let Some(final_data) = final_data {
            last_ctx = Some(final_data.context);
        }
    }

    
    Ok(())
}
