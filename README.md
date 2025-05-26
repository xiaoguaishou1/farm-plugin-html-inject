# farm-plugin-html-inject

Farm HTML 注入插件（Farm HTML Inject Plugin）

---

## 📦 功能简介 (Features)

本插件可以帮助你在 Farm 构建项目时，自动往 HTML 文件的 `<head>` 和 `<body>` 区域插入 `meta`、`link`、`script` 等标签。

This plugin allows you to inject tags (`meta`, `link`, `script`, etc.) into the `<head>` and `<body>` of your HTML files automatically when using Farm.

---

## 📥 安装 (Install)

```bash
pnpm add farm-plugin-html-inject
# 或者使用 npm
npm install farm-plugin-html-inject
# 或者 yarn
yarn add farm-plugin-html-inject
🚀 使用方式 (Usage)
在 Farm 配置中引入并使用插件：

ts
复制
编辑
import { defineConfig } from '@farmfe/core';
import farmPlugin from 'farm-plugin-html-inject';

export default defineConfig({
  plugins: [
    farmPlugin({
      head: [
        { tag: 'meta', attrs: { charset: 'utf-8' } },
        { tag: 'link', attrs: { rel: 'stylesheet', href: '/a111.css' } }
      ],
      body: [
        { tag: 'script', attrs: { src: '/a222.js', type: 'module', async: true } }
      ]
    })
  ]
});
🧩 配置项说明 (Options)
字段名	类型	说明
head	HtmlTag[]	插入到 <head> 的标签数组
body	HtmlTag[]	插入到 <body> 的标签数组

HtmlTag 类型：

ts
复制
编辑
type HtmlTag = {
  tag: 'meta' | 'link' | 'script' | 'div';
  attrs?: Record<string, string | boolean>;
  content?: string;
}
✅ 效果示例 (Example Output)
给定配置：

ts
复制
编辑
{
  head: [
    { tag: 'meta', attrs: { charset: 'utf-8' } },
    { tag: 'link', attrs: { rel: 'stylesheet', href: '/a111.css' } }
  ],
  body: [
    { tag: 'script', attrs: { src: '/a222.js', type: 'module', async: true } }
  ]
}
生成的 HTML 注入片段大致如下：

html
复制
编辑
<head>
  <meta charset="utf-8" />
  <link rel="stylesheet" href="/a111.css" />
</head>
<body>
  <script src="/a222.js" type="module" async></script>
</body>
🛠 常见问题 (FAQ)
❓ 找不到类型声明？
请确认 node_modules/farm-plugin-html-inject/scripts/index.d.ts 存在，或尝试重装依赖、重启编辑器。

❓ 配置变更不生效？
建议清理缓存并重启开发服务，例如：
rm -rf node_modules .farm dist
pnpm install
pnpm run dev