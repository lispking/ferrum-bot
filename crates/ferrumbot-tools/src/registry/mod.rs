mod validate;

use std::collections::HashMap;
use std::sync::Arc;

use serde_json::{Value, json};

use crate::{Tool, ToolContext};

#[derive(Default, Clone)]
pub struct ToolRegistry {
    tools: HashMap<String, Arc<dyn Tool>>,
}

impl ToolRegistry {
    pub fn register<T>(&mut self, tool: T)
    where
        T: Tool + 'static,
    {
        self.tools.insert(tool.name().to_string(), Arc::new(tool));
    }

    pub fn unregister(&mut self, name: &str) {
        self.tools.remove(name);
    }

    pub fn definitions(&self) -> Vec<Value> {
        self.tools
            .values()
            .map(|tool| {
                json!({
                    "type": "function",
                    "function": {
                        "name": tool.name(),
                        "description": tool.description(),
                        "parameters": tool.parameters(),
                    }
                })
            })
            .collect()
    }

    pub async fn execute(&self, name: &str, args: Value, ctx: ToolContext) -> String {
        let Some(tool) = self.tools.get(name) else {
            return format!("Error: Tool '{name}' not found");
        };

        let errors = validate::validate_params(tool.parameters(), &args);
        if !errors.is_empty() {
            return format!(
                "Error: Invalid parameters for tool '{}': {}",
                name,
                errors.join("; ")
            );
        }

        match tool.execute(args, ctx).await {
            Ok(out) => out,
            Err(err) => format!("Error executing {name}: {err}"),
        }
    }
}
