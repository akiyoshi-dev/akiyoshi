# theme

`theme` 是 Akiyoshi 的主题管理 crate，采用 **shadcn/ui 风格语义令牌**，统一管理颜色、间距、圆角与排版。

---

## 文件结构

```
themes/
├── dark.json           # 内置深色主题（默认）
├── light.json          # 内置浅色主题
└── theme.schema.json   # JSON Schema（字段约束 + IDE 自动补全）
```

在每个主题文件顶部声明 schema 引用，可获得编辑器实时校验与提示：

```json
{
  "$schema": "./theme.schema.json"
}
```

---

## 主题文件结构

```jsonc
{
  "$schema": "./theme.schema.json",
  "id": "my_dark",           // 必填，全局唯一标识符
  "name": "My Dark Theme",   // 必填，显示名称
  "appearance": "dark",      // 必填，"light" | "dark"
  "describe": "...",         // 可选，主题说明
  "version": "1.0.0",        // 可选，语义化版本（x.y.z）
  "author": {                // 可选
    "name": "作者名",
    "contact": "email 或社交链接"
  },
  "styles": { ... }          // 必填，见下方设计令牌说明
}
```

---

## 设计令牌（`styles`）

### 颜色令牌（`styles.colors`）

#### 基础色

| 字段           | 类型     | 说明                              |
|--------------|--------|---------------------------------|
| `background` | hex 颜色 | 应用整体背景颜色                        |
| `foreground` | hex 颜色 | 默认前景色（通常为默认文本颜色）                |
| `primary`    | hex 颜色 | 主色调，用于主要按钮、强调操作等                |
| `secondary`  | hex 颜色 | 次要颜色，用于辅助操作                     |
| `accent`     | hex 颜色 | 强调颜色，用于高亮、特殊状态                  |
| `muted`      | hex 颜色 | 弱化颜色，用于低强调信息                    |
| `danger`     | hex 颜色 | 危险颜色，用于删除、错误操作                  |
| `success`    | hex 颜色 | 成功状态颜色                          |
| `warning`    | hex 颜色 | 警告状态颜色                          |
| `info`       | hex 颜色 | 信息提示颜色                          |
| `ring`       | hex 颜色 | 聚焦状态外圈颜色（focus ring）            |
| `overlay`    | hex 颜色 | 遮罩层颜色（如 Modal 背景遮罩）             |

#### 表面层（`surface`）

| 字段         | 说明              |
|------------|-----------------|
| `default`  | 默认表面背景颜色        |
| `hover`    | 鼠标悬停时的表面颜色      |
| `active`   | 激活状态下的表面颜色      |
| `elevated` | 提升层级后的表面颜色（弹窗） |

#### 卡片（`card`）

| 字段           | 说明     |
|--------------|--------|
| `background` | 卡片背景颜色 |
| `foreground` | 卡片文本颜色 |
| `border`     | 卡片边框颜色 |

#### 弹出层（`popover`）

| 字段           | 说明       |
|--------------|----------|
| `background` | 弹出层背景颜色  |
| `foreground` | 弹出层文本颜色  |
| `border`     | 弹出层边框颜色  |

#### 文本（`text`）

| 字段          | 说明                    |
|-------------|-----------------------|
| `primary`   | 主要文本颜色                |
| `secondary` | 次级文本颜色                |
| `muted`     | 弱化文本颜色                |
| `disabled`  | 禁用状态文本颜色              |
| `inverted`  | 反色文本颜色（通常用于深色背景上的白字） |

#### 边框（`border`）

| 字段        | 说明     |
|-----------|--------|
| `default` | 默认边框颜色 |
| `muted`   | 弱化边框颜色 |
| `strong`  | 强调边框颜色 |

#### 输入框（`input`）

| 字段            | 说明         |
|---------------|------------|
| `background`  | 输入框背景颜色    |
| `foreground`  | 输入框文本颜色    |
| `border`      | 输入框边框颜色    |
| `placeholder` | 输入框占位符文本颜色 |

---

### 间距令牌（`styles.spacing`）

单位：像素（`number`，≥ 0）

| 字段   | 参考值（dark.json） |
|------|----------------|
| `xs` | 4              |
| `sm` | 8              |
| `md` | 12             |
| `lg` | 16             |
| `xl` | 24             |

---

### 圆角令牌（`styles.radius`）

单位：像素（`number`，≥ 0）

| 字段   | 参考值（dark.json） |
|------|----------------|
| `sm` | 4              |
| `md` | 8              |
| `lg` | 12             |

---

### 排版令牌（`styles.typography`）

单位：像素（`number`，> 0）

| 字段   | 参考值（dark.json） |
|------|----------------|
| `sm` | 12             |
| `md` | 14             |
| `lg` | 16             |

---

## 在 Rust 中使用

### 初始化

在应用启动时调用 `theme::init`，传入想要激活的主题 ID（`None` 使用默认主题 `akiyoshi_dark`）：

```rust
use theme::GlobalTheme;

// 使用默认深色主题
theme::init(None, cx).unwrap();

// 使用指定主题
theme::init(Some("akiyoshi_light".into()), cx).unwrap();
```

### 在组件中读取当前主题

```rust
use theme::GlobalTheme;
use gpui::{rgb, div, IntoElement};

fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
    let theme = GlobalTheme::theme(cx);

    div()
        .bg(rgb(theme.styles.colors.background.into()))
        .text_color(rgb(theme.styles.colors.text.primary.into()))
}
```

### 常用字段访问路径

```rust
let theme = GlobalTheme::theme(cx);

// 颜色
theme.styles.colors.background          // 背景色
theme.styles.colors.primary             // 主色调
theme.styles.colors.text.primary        // 主文本色
theme.styles.colors.text.inverted       // 反色文本（用于深色按钮上的白字）
theme.styles.colors.border.default      // 默认边框
theme.styles.colors.surface.hover       // 悬停表面色
theme.styles.colors.danger              // 危险色
theme.styles.colors.card.background     // 卡片背景
theme.styles.colors.input.placeholder   // 输入框占位符色

// 间距（f32，单位 px）
theme.styles.spacing.md                 // 12px
theme.styles.spacing.xl                 // 24px

// 圆角（f32，单位 px）
theme.styles.radius.md                  // 8px

// 字体大小（f32，单位 px）
theme.styles.typography.md              // 14px
```

### 动态切换主题

```rust
theme::init(Some("akiyoshi_light".into()), cx).unwrap();
cx.notify();
```

---

## 内置主题

| ID               | 名称             | 外观 |
|------------------|--------------|----|
| `akiyoshi_dark`  | Akiyoshi Dark  | 深色 |
| `akiyoshi_light` | Akiyoshi Light | 浅色 |

默认激活主题：`akiyoshi_dark`

---

## 自定义主题

1. 在 `crates/theme/themes/` 目录下新建 `my_theme.json`
2. 文件顶部添加 `"$schema": "./theme.schema.json"` 获得 IDE 校验
3. 填写所有必填字段（`id`、`name`、`appearance`、`styles`）
4. 通过 `theme::init(Some("my_theme".into()), cx)` 激活

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
        "default": "#1a1a2e",
        "hover":   "#16213e",
        "active":  "#0f3460",
        "elevated":"#1a1a2e"
      },
      "primary": "#e94560",
      "border": {
        "default": "#0f3460",
        "muted":   "#16213e",
        "strong":  "#e94560"
      },
      "text": {
        "primary":   "#e0e0e0",
        "secondary": "#a0a0b0",
        "muted":     "#606070",
        "disabled":  "#404050",
        "inverted":  "#1a1a2e"
      }
      // ... 其余必填颜色字段
    },
    "spacing":    { "xs": 4, "sm": 8, "md": 12, "lg": 16, "xl": 24 },
    "radius":     { "sm": 4, "md": 8, "lg": 12 },
    "typography": { "sm": 12, "md": 14, "lg": 16 }
  }
}
```
