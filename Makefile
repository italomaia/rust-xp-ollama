# model is stored under ./ollama; make sure path exists
llm-up:
	@podman run --rm -d -v ${HOME}/Ollama:/root/.ollama -p 11434:11434 --name ollama docker.io/ollama/ollama

# -- BEG load llm-models

# 3.8gb download...
llm-cli-llama2:
	podman exec -it ollama ollama run llama2

# 26gb download...
llm-cli-mixtral:
	podman exec -it ollama ollama run mixtral

# -- end

# -- run examples
c01:
	@cargo run -q --example 01-simple
c02:
	@cargo run -q --example 02-context
c03:
	@cargo run -q --example 03-chat
c04:
	@cargo run -q --example 04-embeddings
# -- end