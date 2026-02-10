use super::OpenAiCompatibleProvider;

impl OpenAiCompatibleProvider {
    pub(super) fn normalize_model(&self, model: &str) -> String {
        let model = model.to_string();
        let lower = model.to_lowercase();

        if self.api_base.contains("openrouter") && !model.starts_with("openrouter/") {
            return format!("openrouter/{model}");
        }
        if self.api_base.contains("aihubmix") {
            return format!("openai/{}", model.split('/').next_back().unwrap_or(&model));
        }
        if self.api_base.contains("localhost") || self.api_base.contains("127.0.0.1") {
            return format!("hosted_vllm/{model}");
        }

        let platforms = ["iflow", "opencode"];
        for platform in platforms {
            if self.api_base.contains(platform) {
                return model;
            }
        }

        let prefixes = [
            (vec!["glm", "zhipu", "zai"], "zai"),
            (vec!["qwen", "dashscope"], "dashscope"),
            (vec!["moonshot", "kimi"], "moonshot"),
            (vec!["gemini"], "gemini"),
        ];

        for (kws, prefix) in prefixes {
            if kws.iter().any(|kw| lower.contains(kw)) && !model.contains('/') {
                return format!("{prefix}/{model}");
            }
        }

        model
    }
}
