# Leptos+Trunk+Tailwind

## Tailwind

-   前端怎么配置，这里就怎么配置，https://tailwindcss.com/docs/installation/tailwind-cli
-   如果不想用前端的，可以用可执行文件来编译，https://tailwindcss.com/blog/standalone-cli

-   tailwind.config.js

```js
module.exports = {
    content: ["./index.html", "./src/**/*.rs"],
    theme: {
        extend: {},
    },
    plugins: [],
};
```

-   tailwindcss4, input.css, `@import "tailwindcss";`

-   tailwindcss4 编译命令：

    -   `npx @tailwindcss/cli -i ./input.css -o ./style/output.css --watch`
    -   `tailwindcss -i ./input.css -o ./style/output.css --watch`

-   在index.html引入tailwindcss编译输出的css：`<link data-trunk rel="css" href="/style/output.css" />`


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

## 启动trunk 服务器

-   `trunk serve --open`

### Trunk.toml

- https://trunkrs.dev/configuration/

### 自动编译tailwindcss

- trunk自动编译tailwindcss

```toml
[[hooks]]
stage = "pre_build"  # When to run hook, must be one of "pre_build", "build", "post_build"
command = "tailwindcss"        # Command to run
command_arguments = [
    "-i",
    "./input.css",
    "-o",
    "./style/output.css",
] # Arguments to pass to command
```



- 最新版已内置自动编译tailwindcss：https://trunkrs.dev/assets/#tailwind
  - 只需引入：`<link data-trunk rel="tailwind-css" href="/style/tailwind.css" />`

### watch配置

- https://trunkrs.dev/configuration/#watch-section
- 推荐仅设置监听文件

```toml
[watch]
# Paths to watch. The `build.target`'s parent folder is watched by default.
watch = ["src/", "index.html", "input.css"]  
# Paths to ignore.
ignore = []
```

- 关于tailwindcss：trunk默认监听build.target所在目录，导致tailwind.css修改时重新编译，需要忽视这个tailwind编译输出文件style/tailwind.css
  - 但是更推荐仅设置监听文件

### 取消内置的build.target目录监听

- trunk内置监听build.target，即使设置了watch配置也没用，导致每次都会构建2次

- `trunk serve --enable-cooldown`
  - Allow enabling a cooldown, discarding all change events during the build
  - https://github.com/trunk-rs/trunk/issues/735

# leptos view fromat

- rustfmt.toml
- rust-analyzer.toml
- leptosfmt.toml
