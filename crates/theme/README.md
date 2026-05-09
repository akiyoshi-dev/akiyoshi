# theme

`theme` 是独立主题管理 crate，采用 **shadcn/ui 风格语义令牌**，用于统一管理颜色、间距、圆角、字体和组件状态。

## 核心能力

- 主题模式：`ThemeKind::{Light, Dark, Auto}`
- `Auto` 自动跟随系统明暗；读取失败时回退到 `Dark`
- 语义颜色：`semantic.base`、`semantic.intent`、`semantic.state`、`semantic.interactive`
- 组件状态：`button.variants` 支持 `primary/secondary/destructive/outline/ghost/link`
- 设计令牌：`spacing`、`radius`、`typography`

## 文件结构

- `themes/light.json`：浅色主题
- `themes/dark.json`：深色主题
- `themes/theme.schema.json`：完整 JSON Schema（字段约束 + IDE 提示）

建议在每个主题文件顶部保留：

```json
{
  "$schema": "./theme.schema.json"
}
```

## 配色字段说明（重点）

### 1) `semantic.base`（基础语义色）

- `background`：应用主背景
- `foreground`：主文本颜色
- `card` / `card_foreground`：卡片背景与文本
- `popover` / `popover_foreground`：弹层背景与文本
- `border`：通用边框
- `input`：输入框边框/底色参考
- `ring`：焦点环（focus ring）

### 2) `semantic.intent`（意图色：主/次/危险等）

每个意图色（`primary`、`secondary`、`destructive`、`muted`、`accent`）都包含：

- `default`：默认底色
- `foreground`：与 `default` 配套的文本/图标色
- `hover`：鼠标移入
- `active`：按下/激活
- `disabled`：禁用底色
- `disabled_foreground`：禁用文本/图标色

这组字段可直接驱动按钮、标签、徽标、状态块等组件。

### 3) `semantic.state`（反馈状态色）

- `success`：成功提示
- `warning`：警告提示
- `info`：信息提示

每个状态含：

- `default`：强调状态色
- `foreground`：状态文本色
- `subtle`：浅底提示块（例如 toast 背景）

### 4) `semantic.interactive`（通用交互层）

- `surface_hover`：通用容器 hover 底色
- `surface_active`：通用容器 active 底色
- `overlay`：遮罩层颜色
- `selection`：选区高亮色

### 5) `button`（按钮组件令牌）

- `size`：默认按钮尺寸（当前项目用于按钮基础宽高标尺）
- `variants`：六种变体 `primary/secondary/destructive/outline/ghost/link`
- 每个变体包含：
  - `background`
  - `foreground`
  - `border`
  - `hover_background`
  - `active_background`
  - `disabled_background`
  - `disabled_foreground`

## 与代码的映射

`Theme` 会优先读取上述语义字段，并兼容输出当前 UI 使用的聚合字段（如 `theme.colors.*`、`theme.button.*`），方便逐步迁移组件。

## 使用方式

```rust
use theme::{active_theme, set_active_theme_kind, ThemeKind};

set_active_theme_kind(ThemeKind::Auto);
let theme = active_theme();

println!("background = {:#x}", theme.semantic.base.background);
println!("primary hover = {:#x}", theme.semantic.intent.primary.hover);
```

