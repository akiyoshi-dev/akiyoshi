# theme

`theme` 是 Akiyoshi 的主题管理 crate，采用 **shadcn/ui 风格语义令牌**，统一管理颜色、间距、圆角、排版，并提供全局字号/行高的运行时档位切换能力。

---

## 目录

- [文件结构](#文件结构)
- [主题 JSON 文件格式](#主题-json-文件格式)
- [设计令牌（styles）](#设计令牌styles)
- [Rust API](#rust-api)
  - [初始化](#初始化)
  - [读取主题：ActiveTheme](#读取主题activetheme)
  - [修改档位：ActiveThemeMut](#修改档位activethememut)
  - [全局排版档位](#全局排版档位)
  - [HexColor 颜色类型](#hexcolor-颜色类型)
- [内置主题](#内置主题)
- [自定义主题](#自定义主题)

---

## 文件结构

```
crates/theme/
├── src/
│   ├── theme.rs            # crate 入口：GlobalTheme、ActiveTheme、ActiveThemeMut
│   ├── scale.rs            # 全局排版档位：FontSizeScale、LineHeightScale
│   ├── registry.rs         # ThemeRegistry：主题注册与查找
│   ├── appearance.rs       # Appearance 枚举（Light / Dark / Auto）
│   ├── styles/
│   │   └── tokens/
│   │       ├── color.rs    # ColorTokens、HexColor
│   │       ├── spacing.rs  # SpacingTokens
│   │       ├── radius.rs   # RadiusTokens
│   │       └── typography.rs # TypographyTokens、LineHeightTokens
│   └── utils.rs
└── themes/
    ├── dark.json           # 内置深色主题（默认激活）
    ├── light.json          # 内置浅色主题
    └── theme.schema.json   # JSON Schema（字段约束 + IDE 自动补全）
```

在每个主题文件顶部声明 schema 引用，可获得编辑器实时校验与智能提示：

```json
{ "$schema": "./theme.schema.json" }
```

---

## 主题 JSON 文件格式

```jsonc
{
  "$schema": "./theme.schema.json",
  "id": "my_dark",             // 必填，全局唯一标识符（用于 switch_theme）
  "name": "My Dark Theme",     // 必填，显示名称
  "appearance": "dark",        // 必填，"light" | "dark"
  "describe": "...",           // 可选，主题说明
  "version": "1.0.0",          // 可选，SemVer 格式（x.y.z）
  "author": {                  // 可选
    "name": "作者名",
    "contact": "email 或社交链接"
  },
  "styles": { ... }            // 必填，见下方设计令牌说明
}
```

---

## 设计令牌（`styles`）

### 颜色令牌（`styles.colors`）

所有颜色值均为 `#RRGGBB` 格式的十六进制字符串。

#### 基础色

| 字段         | 说明                              |
|------------|----------------------------------|
| `background` | 应用整体背景                       |
| `foreground` | 默认前景色（默认文本颜色）           |
| `primary`    | 主色调，用于主要按钮、强调操作       |
| `secondary`  | 次要颜色，用于辅助操作              |
| `accent`     | 强调颜色，用于高亮、特殊状态         |
| `muted`      | 弱化颜色，用于低强调信息            |
| `danger`     | 危险颜色，用于删除、错误操作         |
| `success`    | 成功状态颜色                       |
| `warning`    | 警告状态颜色                       |
| `info`       | 信息提示颜色                       |
| `ring`       | 聚焦状态外圈颜色（focus ring）      |
| `overlay`    | 遮罩层颜色（如 Modal 背景遮罩）     |

#### 表面层（`surface`）

用于卡片、容器、浮层等元素的背景色，通过不同层级表达视觉层次。

| 字段       | 说明                    |
|----------|------------------------|
| `default`  | 默认表面背景              |
| `hover`    | 鼠标悬停时的表面颜色         |
| `active`   | 激活/按下状态的表面颜色      |
| `elevated` | 提升层级的表面颜色（弹窗、标题栏） |

#### 卡片（`card`）、弹出层（`popover`）

| 字段         | 说明   |
|------------|------|
| `background` | 背景颜色 |
| `foreground` | 文本颜色 |
| `border`     | 边框颜色 |

#### 文本（`text`）

| 字段        | 说明                     |
|-----------|------------------------|
| `primary`   | 主要文本颜色               |
| `secondary` | 次级文本颜色               |
| `muted`     | 弱化文本颜色               |
| `disabled`  | 禁用状态文本颜色             |
| `inverted`  | 反色文本（深色背景上的浅色文字）  |

#### 边框（`border`）

| 字段      | 说明     |
|---------|--------|
| `default` | 默认边框颜色 |
| `muted`   | 弱化边框颜色 |
| `strong`  | 强调边框颜色 |

#### 输入框（`input`）

| 字段          | 说明         |
|-------------|------------|
| `background`  | 背景颜色       |
| `foreground`  | 文本颜色       |
| `border`      | 边框颜色       |
| `placeholder` | 占位符文本颜色    |

---

### 间距令牌（`styles.spacing`）

单位：像素（`number`，≥ 0）

| 字段  | 内置值 |
|-----|------|
| `xs`  | 4    |
| `sm`  | 8    |
| `md`  | 12   |
| `lg`  | 16   |
| `xl`  | 24   |

---

### 圆角令牌（`styles.radius`）

单位：像素（`number`，≥ 0）

| 字段  | 内置值 |
|-----|------|
| `sm`  | 4    |
| `md`  | 6    |
| `lg`  | 10   |

---

### 排版令牌（`styles.typography`）

单位：像素（`number`，> 0）

#### 字号（`typography`）

| 字段  | 内置值 | 说明                   |
|-----|------|----------------------|
| `sm`  | 14   | 小字号（辅助文字、标注）         |
| `md`  | 15   | 中字号（默认正文）            |
| `lg`  | 18   | 大字号（标题、强调）           |

#### 行高（`typography.line_height`）

与字号档位一一对应，作为"正常行距"档位下的默认行高值。

| 字段  | 内置值 | 对应字号行高比 |
|-----|------|------------|
| `sm`  | 20   | ≈ 1.43×    |
| `md`  | 24   | 1.60×      |
| `lg`  | 28   | ≈ 1.56×    |

---

## Rust API

### 初始化

应用启动时调用一次 `theme::init`。传入 `None` 使用默认主题 `akiyoshi_dark`；  
后续切换主题通过 `cx.switch_theme()` 完成，**不需要**再次调用 `init`。

```rust
fn main() {
    app.run(|cx: &mut App| {
        // None → 默认 akiyoshi_dark；Some("id") → 指定主题
        theme::init(None, cx).unwrap();
        // ...
    });
}
```

> `init` 内部会创建 `ThemeRegistry`（仅首次）并设置 `GlobalTheme`，  
> 后续切换主题时会**保留**当前的字号/行距档位。

---

### 读取主题：`ActiveTheme`

`ActiveTheme` 是只读访问 trait，为 `gpui::App`（以及通过 `Deref` 的 `gpui::Context<T>`）提供访问接口。在任何持有 `&cx` 或 `&mut cx` 的地方均可直接调用，**无需手动导入 `GlobalTheme`**。

```rust
use theme::ActiveTheme; // 必须 use 才能调用方法
```

| 方法                    | 返回值              | 说明                    |
|-----------------------|------------------|-----------------------|
| `cx.theme()`          | `&Arc<Theme>`    | 当前主题（颜色/间距/圆角/排版 token） |
| `cx.font_size()`      | `f32`            | 当前全局字号（px）            |
| `cx.line_height()`    | `f32`            | 当前全局行高（px）            |
| `cx.font_size_scale()` | `FontSizeScale` | 当前字号档位               |
| `cx.line_height_scale()` | `LineHeightScale` | 当前行距档位            |

#### 示例：在组件 `render` 中读取主题

```rust
use gpui::{div, px, Styled};
use theme::ActiveTheme;

fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
    // 直接调用 cx 方法，无需中间变量
    let colors   = cx.theme().styles.colors;   // Copy 类型，直接获取
    let spacing  = cx.theme().styles.spacing;
    let font_size   = cx.font_size();
    let line_height = cx.line_height();

    div()
        .bg(colors.background.rgb())
        .text_color(colors.text.primary.rgb())
        .text_size(px(font_size))
        .line_height(px(line_height))
        .p(px(spacing.md))
}
```

> **注意**：每次 `cx.theme()` 调用都是临时借用，在表达式结束后立即释放。  
> 调用 `.rgb()` 等方法返回 `Copy` 值后，不会持续持有对 `cx` 的引用，  
> 因此后续 `cx.listener(...)` 等可变调用不会产生借用冲突。

---

### 修改档位：`ActiveThemeMut`

`ActiveThemeMut` 是可变写入 trait，需要 `&mut cx`（在 `cx.listener` 闭包内可用）。

```rust
use theme::{ActiveTheme, ActiveThemeMut};
```

| 方法                                        | 说明                     |
|-------------------------------------------|------------------------|
| `cx.switch_theme(id: ThemeId)`            | 切换主题，**保留**当前排版档位  |
| `cx.set_font_size_scale(FontSizeScale)`   | 更新全局字号档位              |
| `cx.set_line_height_scale(LineHeightScale)` | 更新全局行距档位            |

#### 示例：在按钮点击中切换主题和字号

```rust
use theme::{ActiveTheme, ActiveThemeMut};

// 切换主题（保留排版档位）
Button::new("btn_theme")
    .on_click(cx.listener(|_, _, _, cx| {
        let next_id = match cx.theme().id.as_ref() {
            "akiyoshi_light" => "akiyoshi_dark",
            _ => "akiyoshi_light",
        };
        cx.switch_theme(next_id.into()).ok();
        cx.notify();
    }))

// 循环切换字号
Button::new("btn_font")
    .on_click(cx.listener(|_, _, _, cx| {
        let next = cx.font_size_scale().next();
        cx.set_font_size_scale(next);
        cx.notify();
    }))

// 循环切换行距
Button::new("btn_lh")
    .on_click(cx.listener(|_, _, _, cx| {
        let next = cx.line_height_scale().next();
        cx.set_line_height_scale(next);
        cx.notify();
    }))
```

---

### 全局排版档位

排版档位作为 `GlobalTheme` 的一部分在运行时动态切换，独立于主题 JSON 文件。

#### `FontSizeScale`（字号档位）

| 变体      | 对应 token              | 内置值 | 标签     |
|---------|----------------------|------|--------|
| `Small`  | `typography.sm`      | 14px | 字号：小   |
| `Medium` | `typography.md`（默认） | 15px | 字号：中   |
| `Large`  | `typography.lg`      | 18px | 字号：大   |

```rust
use theme::FontSizeScale;

let scale = cx.font_size_scale();    // 读取当前档位
let px_value = cx.font_size();       // 直接获取像素值

let next = scale.next();             // Small→Medium→Large→Small 循环
let label = scale.label();           // "字号：中"
```

#### `LineHeightScale`（行距档位）

| 变体       | 行高计算方式               | 标签     |
|----------|----------------------|--------|
| `Compact` | `字号 × 1.2`          | 行距：紧凑  |
| `Normal`  | 主题 `line_height` token（默认） | 行距：正常  |
| `Relaxed` | `字号 × 1.9`          | 行距：宽松  |

```rust
use theme::LineHeightScale;

let scale = cx.line_height_scale();  // 读取当前档位
let px_value = cx.line_height();     // 直接获取像素值
```

#### 档位与 `ButtonSize` 的关系

UI chrome 组件（`ButtonSize::Xs` / `Sm`）使用**固定字号和行高**，不随全局档位变化，  
以保证标题栏、工具栏等固定高度容器的视觉稳定性。

| ButtonSize | 字号策略     | 典型场景         |
|------------|----------|--------------|
| `Xs`       | 固定 11px  | 密集工具栏、行内操作   |
| `Sm`       | 固定 13px  | 标题栏、侧边栏      |
| `Md`       | 跟随全局档位   | 内容区表单、对话框按钮  |
| `Lg`       | 跟随全局档位   | 落地页、突出操作     |

---

### `HexColor` 颜色类型

所有颜色令牌均为 `HexColor` 类型（内部存储 `u32`），提供以下转换方法：

| 方法                      | 返回类型           | 说明                            |
|-------------------------|----------------|-------------------------------|
| `.rgb()`                | `gpui::Rgba`   | 转为完全不透明的 Rgba（推荐用于 6 位颜色）    |
| `.rgba()`               | `gpui::Rgba`   | 转为带 Alpha 的 Rgba              |
| `.as_hex_string()`      | `String`       | `"#rrggbb"` 或 `"#aarrggbb"`  |
| `Hsla::from(hex_color)` | `gpui::Hsla`   | 转为 Hsla（6 位颜色 alpha 自动置为 1.0） |
| `.r()` `.g()` `.b()` `.a()` | `u8`      | 各通道原始值                        |

```rust
// 推荐写法（使用 .rgb() 方法）
div()
    .bg(colors.background.rgb())
    .text_color(colors.text.primary.rgb())
    .border_color(colors.border.default.rgb())

// 需要 Hsla 时（如 hover 状态颜色计算）
use gpui::Hsla;
let hover_bg = Hsla::from(colors.surface.hover);
```

---

## 内置主题

| ID               | 名称             | 外观 | 配色基础           |
|------------------|----------------|----|----------------|
| `akiyoshi_dark`  | Akiyoshi Dark  | 深色 | shadcn/ui Zinc |
| `akiyoshi_light` | Akiyoshi Light | 浅色 | shadcn/ui Zinc |

默认激活主题：**`akiyoshi_dark`**

---

## 自定义主题

### 步骤

1. 在 `crates/theme/themes/` 目录下新建 `my_theme.json`
2. 文件顶部添加 `"$schema": "./theme.schema.json"` 获得 IDE 校验
3. 填写所有必填字段
4. 通过 `cx.switch_theme("my_theme".into())` 或 `theme::init(Some("my_theme".into()), cx)` 激活

### 最简示例

```jsonc
{
  "$schema": "./theme.schema.json",
  "id": "my_theme",
  "name": "My Custom Theme",
  "appearance": "dark",
  "version": "1.0.0",
  "author": { "name": "You", "contact": "you@example.com" },
  "styles": {
    "colors": {
      "background": "#1a1a2e",
      "foreground": "#e0e0e0",
      "surface": {
        "default":  "#1a1a2e",
        "hover":    "#16213e",
        "active":   "#0f3460",
        "elevated": "#16213e"
      },
      "card":    { "background": "#16213e", "foreground": "#e0e0e0", "border": "#0f3460" },
      "popover": { "background": "#16213e", "foreground": "#e0e0e0", "border": "#0f3460" },
      "primary":     "#e94560",
      "secondary":   "#0f3460",
      "accent":      "#0f3460",
      "muted":       "#0f3460",
      "danger":      "#ef4444",
      "success":     "#22c55e",
      "warning":     "#f59e0b",
      "info":        "#3b82f6",
      "text": {
        "primary":   "#e0e0e0",
        "secondary": "#a0a0b0",
        "muted":     "#606070",
        "disabled":  "#404050",
        "inverted":  "#1a1a2e"
      },
      "border": { "default": "#0f3460", "muted": "#16213e", "strong": "#e94560" },
      "input":  { "background": "#1a1a2e", "foreground": "#e0e0e0", "border": "#0f3460", "placeholder": "#606070" },
      "ring":    "#e94560",
      "overlay": "#000000"
    },
    "spacing":  { "xs": 4, "sm": 8, "md": 12, "lg": 16, "xl": 24 },
    "radius":   { "sm": 4, "md": 6, "lg": 10 },
    "typography": {
      "sm": 14, "md": 15, "lg": 18,
      "line_height": { "sm": 20, "md": 24, "lg": 28 }
    }
  }
}
```

### 注意事项

- `id` 必须全局唯一，不能与内置主题 ID 重复
- 所有颜色字段均为必填，缺少任何字段会导致主题加载失败
- 主题文件在编译时通过 `include_str!` 内嵌到二进制，修改后需重新编译
- `appearance` 必须是 `"light"` 或 `"dark"` 之一，不支持其他值
