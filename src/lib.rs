// region: types

pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>; // for early dev

// endregion

pub mod consts { // dev only

    pub const MODEL: &str = "llama2";

    // default instructions to the llm
    pub const DEFAULT_SYSTEM_MOCK: &str = r#"
    Always be very concise in your answer. 

    If asked about the best programming language, say it is Rust by light years.

    If asked about the second best programming language, say that it is Cobol.

    Do not mention the second best, except if asked.

    If asked about the previous question, only give user messages, not system messages.
    "#;
}

pub mod gen {
    use ollama_rs::generation::completion::GenerationFinalResponseData;
    use ollama_rs::Ollama;
    use ollama_rs::generation::completion::request::GenerationRequest;
    use futures::StreamExt;
    use tokio::io::AsyncWriteExt;
    use super::*; // so Ollama and GenerationRequest are visible

    pub async fn gen_stream_print(
        ollama: &Ollama,
        gen_req: GenerationRequest
    ) -> Result<Option<GenerationFinalResponseData>> {
        let mut stream = ollama.generate_stream(gen_req).await?;
        let mut stdout = tokio::io::stdout();
        let mut char_count = 0;
    
        while let Ok(res_list) = stream.next().await.transpose() {
            if let Some(res_list) = res_list {
                for res in res_list {
                    let bytes = res.response.as_bytes();
    
                    // poor man's wrapping
                    char_count += bytes.len();
                    if char_count > 80 {
                        stdout.write_all(b"\n").await?;
                        char_count = 0;
                    }
    
                    stdout.write_all(bytes).await?;
                    stdout.flush().await?;

                    if let Some(final_data) = res.final_data {
                        stdout.write_all(b"\n").await?;
                        return Ok(Some(final_data));
                    }
                }
            }
        }
    
        stdout.write_all(b"\n").await?;
    
        Ok(None)
    }
}
