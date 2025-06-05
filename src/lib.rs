/*
 * @Author: 喜猫 tompanghu@gmail.com
 * @Date: 2025-05-24 22:52:34
 * @LastEditors: 喜猫 tompanghu@gmail.com
 * @LastEditTime: 2025-05-26 10:45:49
 * @FilePath: /Plugins/farm-plugin-html-inject/src/lib.rs
 * @Description: Farm 插件：注入 meta/link/script 标签，支持配置化参数
 */
 #![deny(clippy::all)]

 use farmfe_core::{
     config::Config,
     plugin::{Plugin, PluginTransformHookParam, PluginTransformHookResult},
     context::CompilationContext,
     error::CompilationError,
     module::ModuleType,
 };
 use farmfe_macro_plugin::farm_plugin;
 use std::{collections::HashMap, sync::Arc};
 
 use serde::Deserialize;
 use serde_json::Value;
 
 #[derive(Debug, Clone, Deserialize)]
 #[serde(rename_all = "lowercase", tag = "tag")]
 enum HtmlTag {
     Meta {
         #[serde(default)]
         attrs: HashMap<String, Value>,
     },
     Link {
         #[serde(default)]
         attrs: HashMap<String, Value>,
     },
     Script {
         #[serde(default)]
         attrs: HashMap<String, Value>,
         #[serde(default)]
         content: Option<String>,
     },
 }
 
 #[derive(Debug, Clone, Deserialize, Default)]
 struct HtmlInjectOptions {
     #[serde(default)]
     head: Vec<HtmlTag>,
     #[serde(default)]
     body: Vec<HtmlTag>,
 }
 
 #[farm_plugin]
 pub struct FarmPluginHtmlInject {
     options: HtmlInjectOptions,
 }
 
 impl FarmPluginHtmlInject {
     pub fn new(_config: &Config, options: String) -> Self {
         let parsed = serde_json::from_str::<HtmlInjectOptions>(&options)
             .unwrap_or_else(|e| {
                 HtmlInjectOptions::default()
             });
         Self { options: parsed }
     }
 
     fn tag_to_html(tag: &HtmlTag) -> String {
         match tag {
             HtmlTag::Meta { attrs } => {
                 let attrs = attrs.iter()
                     .map(|(k, v)| format!(r#" {k}="{}""#, v.as_str().unwrap_or(&v.to_string())))
                     .collect::<Vec<_>>().join("");
                 format!(r#"<meta{}/>"#, attrs)
             }
             HtmlTag::Link { attrs } => {
                 let attrs = attrs.iter()
                     .map(|(k, v)| format!(r#" {k}="{}""#, v.as_str().unwrap_or(&v.to_string())))
                     .collect::<Vec<_>>().join("");
                 format!(r#"<link{}/>"#, attrs)
             }
             HtmlTag::Script { attrs, content } => {
                 let attrs = attrs.iter()
                     .map(|(k, v)| {
                         if v == &Value::Bool(true) {
                             format!(r#" {k}"#)
                         } else if v == &Value::Bool(false) {
                             String::new()
                         } else {
                             format!(r#" {k}="{}""#, v.as_str().unwrap_or(&v.to_string()))
                         }
                     })
                     .collect::<Vec<_>>().join("");
                 let c = content.as_deref().unwrap_or("");
                 format!(r#"<script{attrs}>{c}</script>"#)
             }
         }
     }
 
     fn inject_tags(&self, html: &str) -> String {
         let mut result = html.to_string();
         if !self.options.head.is_empty() {
             let head_html = self.options.head.iter().map(Self::tag_to_html).collect::<String>();
             if let Some(pos) = result.find("</head>") {
                 result.insert_str(pos, &head_html);
             }
         }
         if !self.options.body.is_empty() {
             let body_html = self.options.body.iter().map(Self::tag_to_html).collect::<String>();
             if let Some(pos) = result.find("</body>") {
                 result.insert_str(pos, &body_html);
             }
         }
         result
     }
 }
 
impl Plugin for FarmPluginHtmlInject {
     fn name(&self) -> &str {
         "FarmPluginHtmlInject"
     }
 
     fn transform(
         &self,
         param: &PluginTransformHookParam,
         _ctx: &Arc<CompilationContext>,
    ) -> Result<Option<PluginTransformHookResult>, CompilationError> {
        if param.module_type == ModuleType::Html {
            let content = self.inject_tags(&param.content);
            Ok(Some(PluginTransformHookResult { content, ..Default::default() }))
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use serde_json::Value;
    use farmfe_core::config::Config;

    #[test]
    fn tag_to_html_script_bool() {
        let mut attrs = HashMap::new();
        attrs.insert("async".to_string(), Value::Bool(true));
        let tag = HtmlTag::Script { attrs, content: None };
        assert_eq!(FarmPluginHtmlInject::tag_to_html(&tag), "<script async></script>");
    }

    #[test]
    fn new_with_invalid_json() {
        let cfg = Config::default();
        let plugin = FarmPluginHtmlInject::new(&cfg, "not-json".to_string());
        assert!(plugin.options.head.is_empty() && plugin.options.body.is_empty());
    }

    #[test]
    fn inject_tags_inserts_elements() {
        let cfg = Config::default();
        let opts = r#"{\"head\":[{\"tag\":\"meta\",\"attrs\":{\"charset\":\"utf-8\"}}],\"body\":[{\"tag\":\"script\",\"attrs\":{\"src\":\"/main.js\"}}]}"#;
        let plugin = FarmPluginHtmlInject::new(&cfg, opts.to_string());
        let html = "<html><head></head><body></body></html>";
        let result = plugin.inject_tags(html);
        assert!(result.contains("<meta charset=\"utf-8\"/>"));
        assert!(result.contains("<script src=\"/main.js\"></script>"));
    }
}
 
