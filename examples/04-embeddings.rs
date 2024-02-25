use std::fs;
use std::path::Path;

use futures::StreamExt;
use ollama_rs::generation::chat::request::ChatMessageRequest;
use ollama_rs::Ollama;
use simple_fs::{read_to_string, save_be_f64, save_json};
use tokio::io::AsyncWriteExt;
use xp_ollama::consts::MODEL;
use xp_ollama::Result;

const MOCK_DIR: &str = "mock-data";
const C04_DIR: &str = "data";

// here we learn how do create embeddings
// we create a binary vector that maps back to our text

#[tokio::main]
async fn main() -> Result<()> {
    // defaults to localhost:11434
    let ollama = Ollama::default();
    
    let txt = read_to_string(Path::new(MOCK_DIR).join("for-embeddings.txt"))?;
    // simple text splitter on size, only for dev
    let splits = simple_text_splitter(&txt, 500)?;

    println!("->> splits count: {}", splits.len());

    for (i, seg) in splits.into_iter().enumerate() {
        println!();
        let file_name = format!("c04-embeddings-{:0>2}.txt", i);
        let path = Path::new(C04_DIR).join(file_name);
        fs::write(path, &seg)?;

        println!("->> splits count: {}", txt.len());
        let res = ollama.generate_embeddings(
            MODEL.to_string(), seg, None).await?;
        
        println!("->> embeddings lenght: {}", res.embeddings.len());

        let file_name = format!("c04-embeddings-{:0>2}.json", i);
        let file_path = Path::new(C04_DIR).join(file_name);
        save_json(file_path, &res.embeddings)?;

        let file_name = format!("c04-embeddings-{:0>2}.be-f64.bin", i);
        let file_path = Path::new(C04_DIR).join(file_name);
        save_be_f64(file_path, &res.embeddings)?;
    }

    Ok(())
}

fn simple_text_splitter(txt: &str, num: u32) -> Result<Vec<String>> {
    let mut result = Vec::new();
    let mut last = 0;
    let mut count = 0;

    // not unicode safe
    for (idx, _) in txt.char_indices() {
        count += 1;

        if count == num {
            result.push(&txt[last..idx + 1]);
            last = idx + 1;
            count = 0;
        }
    }

    if last < txt.len() {
        result.push(&txt[last..]);
    }


    Ok(result.into_iter().map(String::from).collect())
}

pub async fn run_chat_req(ollama: &Ollama, chat_req: ChatMessageRequest)
-> Result<Option<String>> {
    let mut stream = ollama.send_chat_messages_stream(chat_req).await?;

    let mut stdout = tokio::io::stdout();
    let mut char_count = 0;
    let mut current_asst_msg_elems: Vec<String> = Vec::new();

    // .next() requires StreamExt
    while let Ok(res) = stream.next().await.transpose() {
        if let Some(res) = res {
            if let Some(msg) = res.message {
                let msg_content = msg.content;

                // poor man's wrapping
                char_count += msg_content.len();
                if char_count > 80 {
                    stdout.write_all(b"\n").await?;
                    stdout.flush().await?;
                    char_count = 0;
                }

                stdout.write_all(msg_content.as_bytes()).await?;
                stdout.flush().await?;

                current_asst_msg_elems.push(msg_content);
            }
            
            if let Some(_final_res) = res.final_data {
                stdout.write_all(b"\n").await?;
                stdout.flush().await?;

                let asst_content = current_asst_msg_elems.join("");
                return Ok(Some(asst_content));
            }
        } else { break }
    }

    stdout.write(b"\n").await?;
    stdout.flush().await?;
    
    Ok(None)
}
