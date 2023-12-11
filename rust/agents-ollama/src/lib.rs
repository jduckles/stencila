use ollama_rs::{
    generation::{completion::request::GenerationRequest, options::GenerationOptions},
    Ollama,
};

use agent::{
    common::{
        async_trait::async_trait,
        eyre::{eyre, Result},
    },
    Agent, AgentIO, GenerateTextOptions,
};

/// An agent running on a Ollama (https://github.com/jmorganca/ollama/) server
/// 
/// To start an Ollama server:
/// 
/// ```sh
/// ollama serve
/// ```
/// 
/// On Linux, to stop the server:
/// 
/// ```sh
/// sudo service ollama stop
/// ```
/// 
/// An agent is listed for each Ollama model that has previously been pulled.
struct OllamaAgent {
    /// The Ollama name for a model including any tag e.g. "llama2:13b"
    ///
    /// Used as the required `model` parameter in each request to `POST /api/generate`
    /// (along with `prompt`).
    model: String,

    /// The Ollama API client
    client: Ollama,
}

impl OllamaAgent {
    /// Create a Ollama-based agent
    pub fn new(model: &str) -> Self {
        Self::new_with(model, None, None)
    }

    /// Create a Ollama-based agent with options for address of server
    pub fn new_with(model: &str, host: Option<String>, port: Option<u16>) -> Self {
        let host = host.unwrap_or("http://localhost".to_string());
        let port = port.unwrap_or(11434);
        let client = Ollama::new(host, port);

        Self {
            model: model.into(),
            client,
        }
    }
}

#[async_trait]
impl Agent for OllamaAgent {
    fn name(&self) -> String {
        format!("ollama/{}", self.model)
    }

    fn supports_generating(&self, output: AgentIO) -> bool {
        matches!(output, AgentIO::Text)
    }

    async fn generate_text(
        &self,
        instruction: &str,
        options: Option<GenerateTextOptions>,
    ) -> Result<String> {
        let mut request = GenerationRequest::new(self.model.clone(), instruction.into());
        if let Some(options) = options {
            request.system = options.system_prompt;

            let mut opts = GenerationOptions::default();
            macro_rules! map_option {
                ($from:ident, $to:ident) => {
                    if let Some(value) = options.$from {
                        opts = opts.$to(value);
                    }
                };
                ($name:ident) => {
                    map_option!($name, $name)
                };
            }
            map_option!(mirostat);
            map_option!(mirostat_eta);
            map_option!(mirostat_tau);
            map_option!(num_ctx);
            map_option!(num_gqa);
            map_option!(num_gpu);
            map_option!(num_thread);
            map_option!(repeat_last_n);
            map_option!(repeat_penalty);
            map_option!(temperature);
            map_option!(seed);
            map_option!(stop);
            map_option!(tfs_z);
            map_option!(num_predict);
            map_option!(top_k);
            map_option!(top_p);
            request.options = Some(opts);
        }

        let response = self
            .client
            .generate(request)
            .await
            .map_err(|error| eyre!(error))?;

        Ok(response.response)
    }
}

/// Get a list of all available Ollama agents
///
/// Fetches the list of Ollama models from the server and maps them
/// into agents.
pub async fn list() -> Result<Vec<Box<dyn Agent>>> {
    let models = Ollama::default().list_local_models().await?;
    let agents = models
        .iter()
        .map(|model| Box::new(OllamaAgent::new(&model.name)) as Box<dyn Agent>)
        .collect();
    Ok(agents)
}
