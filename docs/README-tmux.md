# 🏦 VAULT CLI - Pure Icon Tmux

## 🎯 Pure Icon Status Line

```
🏦 session │ 1🔍 2 3* │ ⚡💻🕐📅
```

## 🚀 Quick Setup

```bash
cp .tmux.conf ~/.tmux.conf
tmux source ~/.tmux.conf
```

## 🎨 Icons Used

- **🏦** Bank (session)
- **🔍** Zoom (zoomed windows)
- **⚡** Power
- **💻** Host
- **🕐** Time
- **📅** Date

## 🎯 Features

- **Minimal**: Only icons, no text clutter
- **Clean**: Pure visual status
- **Fast**: Minimal processing
- **Universal**: Works in all terminals

## 🔧 Customization

Edit `.tmux.conf`:
```bash
# Change icons
set -g status-left "🏦 #S "
set -g status-right "⚡💻🕐📅"

# Change colors
set -g status-bg colour236
set -g status-fg colour254
```
