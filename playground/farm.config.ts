/*
 * @Author: 喜猫 tompanghu@gmail.com
 * @Date: 2025-05-24 22:52:34
 * @LastEditors: 喜猫 tompanghu@gmail.com
 * @LastEditTime: 2025-05-26 10:44:00
 * @FilePath: /Plugins/farm-plugin-html-inject/playground/farm.config.ts
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
import { defineConfig } from "@farmfe/core";
import react from '@farmfe/plugin-react';
import farmPlugin from 'farm-plugin-html-inject';

export default defineConfig({
  compilation: {
    input: {
      index: "./index.html",
    },
    persistentCache: false,
    progress: false,
  },
  plugins: [
    react({ runtime: "automatic" }),
    farmPlugin(
      {
        head: [
          { tag: 'meta', attrs: { charset: 'utf-8' } },
          { tag: 'link', attrs: { rel: 'stylesheet', href: '/a111.css' } }
        ],
        body: [
          { tag: 'script', attrs: { src: '/a222.js', type: 'module', async: true } }
        ]
      }
    ),
  ],
});
