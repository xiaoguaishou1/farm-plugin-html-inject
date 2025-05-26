# farm-plugin-html-inject

Farm HTML 注入插件 | Farm HTML Inject Plugin

## 📦 功能简介 (Features)

在使用 Farm 构建项目时，该插件可以帮助你自动向 HTML 文件的 `<head>` 和 `<body>` 区域注入各种 HTML 标签（如 `meta`、`link`、`script` 等）。

This plugin helps you automatically inject HTML tags (such as `meta`, `link`, `script`, etc.) into the `<head>` and `<body>` sections of your HTML files when building projects with Farm.

## 📥 安装 (Installation)

```bash
# 使用 pnpm（推荐）
pnpm add farm-plugin-html-inject

# 或使用 npm
npm install farm-plugin-html-inject

# 或使用 yarn
yarn add farm-plugin-html-inject
```

## 🚀 使用方式 (Usage)

在 Farm 配置文件中引入并使用插件：

```typescript
import { defineConfig } from '@farmfe/core';
import farmPlugin from 'farm-plugin-html-inject';

export default defineConfig({
  plugins: [
    farmPlugin({
      // 注入到 <head> 的标签
      head: [
        { tag: 'meta', attrs: { charset: 'utf-8' } },
        { tag: 'link', attrs: { rel: 'stylesheet', href: '/styles.css' } }
      ],
      // 注入到 <body> 的标签
      body: [
        { tag: 'script', attrs: { src: '/main.js', type: 'module', async: true } }
      ]
    })
  ]
});
```

## 🔧 配置选项 (Configuration)

### 主要配置项

| 参数 | 类型 | 说明 |
|------|------|------|
| `head` | `HtmlTag[]` | 需要注入到 `<head>` 的标签数组 |
| `body` | `HtmlTag[]` | 需要注入到 `<body>` 的标签数组 |

### HtmlTag 类型定义

```typescript
type HtmlTag = {
  // 标签名称
  tag: 'meta' | 'link' | 'script' | 'div';
  // 标签属性
  attrs?: Record<string, string | boolean>;
  // 标签内容
  content?: string;
}
```

## 📝 使用示例 (Example)

### 配置示例

```typescript
{
  head: [
    { tag: 'meta', attrs: { charset: 'utf-8' } },
    { tag: 'meta', attrs: { name: 'viewport', content: 'width=device-width, initial-scale=1.0' } },
    { tag: 'link', attrs: { rel: 'stylesheet', href: '/styles.css' } }
  ],
  body: [
    { tag: 'script', attrs: { src: '/main.js', type: 'module', async: true } },
    { tag: 'div', attrs: { id: 'app' } }
  ]
}
```

### 生成结果

```html
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <link rel="stylesheet" href="/styles.css" />
</head>
<body>
  <div id="app"></div>
  <script src="/main.js" type="module" async></script>
</body>
```

## ❓ 常见问题 (FAQ)

### 1. 类型声明找不到？

确认以下几点：
- 检查 `node_modules/farm-plugin-html-inject/scripts/index.d.ts` 文件是否存在
- 尝试重新安装依赖
- 重启 IDE 或编辑器

### 2. 配置更改没有生效？

请尝试以下步骤：
```bash
# 1. 清理项目缓存和构建文件
rm -rf node_modules .farm dist

# 2. 重新安装依赖
pnpm install

# 3. 重启开发服务器
pnpm run dev
```

## 📄 许可证 (License)

MIT