# model is stored under ./ollama; make sure path exists
llm-up:
	@podman run -d -v ${HOME}/Ollama:/root/.ollama -p 11434:11434 --name ollama docker.io/ollama/ollama

# -- BEG load llm-models

# 3.8gb download...
llm-cli-llama2:
	podman exec -it ollama ollama run llama2

# 26gb download...
llm-cli-mixtral:
	podman exec -it ollama ollama run mixtral

# -- END

e01:
	@cargo run -q --example 01-simple