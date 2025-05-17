# Leptos+Trunk+Tailwind

## Tailwind

-   前端怎么配置，这里就怎么配置，https://tailwindcss.com/docs/installation/tailwind-cli
-   如果不想用前端的，可以用可执行文件来编译，https://tailwindcss.com/blog/standalone-cli

```css
/* tailwindcss4, input.css */
@import "tailwindcss" source(none);
/* 无法使用glob模式 */
@source "../src/";
```

-   tailwindcss4 编译命令：
    -   不推荐这样，因为这样需要监听输出，因为需要把这个输出复制到 dist 目录，tailwindcss 构建不支持 hash 文件名，如果直接输出到 dist 目录，不利于删除客户端缓存，由于 trunk 有 hooks 功能（在构建阶段只需命令）或者最新的 trunk 内置 tailwind 编译，因此非常不推荐自己使用命令。
    -   `npx @tailwindcss/cli -i ./style.css -o ./build/style.css --watch`
    -   `tailwindcss -i ./style.css -o ./build/style.css --watch`
-   在 index.html 引入 tailwindcss 构建的 css：`<link data-trunk rel="css" href="/build/style.css" />`
    -   会在 dist 目录复制一份，但是文件名是 hash 后缀名
    -   tailwindcss 本来就是只需要一个文件，且在 index.html 引入的，即全局使用，因此不需要在其他位置再引入

## VSCode Tailwindcss 配置 settings.json

```json
  "emmet.includeLanguages": {
    "rust": "html",
    "*.rs": "html"
  },
  "tailwindCSS.includeLanguages": {
      "rust": "html",
      "*.rs": "html"
  },
  "files.associations": {
      "*.rs": "rust"
  },
  "editor.quickSuggestions": {
    "other": "on",
    "comments": "on",
    "strings": true
  },
  "css.validate": false,
```

## 启动 trunk 服务器

-   `trunk serve --open`

### Trunk.toml

-   https://trunkrs.dev/configuration/

### 自动编译 tailwindcss

-   trunk 自动编译 tailwindcss

```toml
[[hooks]]
stage = "pre_build"  # When to run hook, must be one of "pre_build", "build", "post_build"
command = "tailwindcss"        # Command to run
command_arguments = [
    "-i",
    "./style.css",
    "-o",
    "./build/style.css",
] # Arguments to pass to command
```

-   最新版已内置自动编译 tailwindcss：https://trunkrs.dev/assets/#tailwind
    -   推荐使用这个，这样可以避免保留 tailwindcss 构建输出文件，直接输出 hash 文件到/dist 目录中
    -   只需在 index.html 引入：`<link data-trunk rel="tailwind-css" href="/style.css" />`
        -   需要下载 tailwindcss 可执行文件
        -   href 指向 tailwindcss 的 input 文件

### watch 配置

-   https://trunkrs.dev/configuration/#watch-section
-   推荐仅设置监听文件

```toml
[watch]
# Paths to watch. The `build.target`'s parent folder is watched by default.
watch = ["src/", "index.html", "css/"]
# Paths to ignore.
ignore = []
```

### 取消内置的 build.target 目录监听

-   trunk 内置监听 build.target，即使设置了 watch 配置也没用，导致每次都会构建 2 次

-   `trunk serve --enable-cooldown`
    -   Allow enabling a cooldown, discarding all change events during the build
    -   https://github.com/trunk-rs/trunk/issues/735

# leptos view fromat

-   rustfmt.toml
-   rust-analyzer.toml
-   leptosfmt.toml
