# Keybinds Configuration

YM2151 Tone Editorでは、カレントディレクトリに `ym2151-tone-editor.toml` ファイルを配置することで、キーバインドをカスタマイズできます。

## 設定ファイルの作成

1. `ym2151-tone-editor.toml.example` をコピーして `ym2151-tone-editor.toml` を作成します：

```bash
cp ym2151-tone-editor.toml.example ym2151-tone-editor.toml
```

2. エディタで `ym2151-tone-editor.toml` を編集してキーバインドをカスタマイズします。

## 設定形式

設定ファイルはTOML形式で、以下の形式で記述します：

```toml
[keybinds]
"キー" = "アクション"
```

### 使用可能なキー

- **文字キー**: `"a"`, `"b"`, ..., `"z"` (小文字)
- **文字キー + Shift**: `"A"`, `"B"`, ..., `"Z"` (大文字)
- **数字キー**: `"0"`, `"1"`, ..., `"9"`
- **特殊文字 + Shift**: `"!"`, `"@"`, `"#"`, `"$"`, `"%"`, `"^"`, `"&"`, `"*"`, `"("`, `")"`
- **その他の特殊文字**: `"+"`, `"-"`, `"_"`, `"="`, `"."`, `","`, `"<"`, `">"`, `"/"`, `"?"`
- **矢印キー**: `"Left"`, `"Right"`, `"Up"`, `"Down"`
- **特殊キー**: `"Home"`, `"End"`, `"PageUp"`, `"PageDown"`, `"Esc"`, `"Space"`

### 使用可能なアクション

#### 値の変更
- `decrease_value` - 値を1減らす
- `increase_value` - 値を1増やす
- `set_value_to_max` - 最大値に設定
- `set_value_to_min` - 最小値（0）に設定
- `set_value_to_random` - ランダムな値に設定

#### 値の増減（特定の量）
- `increase_value_by1` ～ `increase_value_by10` - 値を1～10増やす
- `decrease_value_by1` ～ `decrease_value_by10` - 値を1～10減らす

#### 音声再生
- `play_current_tone` - 現在の音色を再生

#### フィードバック調整
- `increase_fb` - フィードバック（FB）を増やす
- `decrease_fb` - フィードバック（FB）を減らす

#### カーソル移動
- `move_cursor_left` - カーソルを左に移動
- `move_cursor_right` - カーソルを右に移動
- `move_cursor_up` - カーソルを上に移動
- `move_cursor_down` - カーソルを下に移動

#### その他
- `exit` - エディタを終了

## 設定例

### デフォルト設定のカスタマイズ

```toml
[keybinds]
# qキーを無効化してuキーで値を減らす
"u" = "decrease_value"

# iキーで値を増やす（eキーの代わり）
"i" = "increase_value"

# xキーでランダム値に設定（rキーの代わり）
"x" = "set_value_to_random"
```

### シンプルな設定（最小限のキーバインド）

```toml
[keybinds]
"-" = "decrease_value"
"+" = "increase_value"
"Left" = "move_cursor_left"
"Right" = "move_cursor_right"
"Up" = "move_cursor_up"
"Down" = "move_cursor_down"
"Space" = "play_current_tone"
"Esc" = "exit"
```

## 注意事項

- 設定ファイルが存在しない場合は、デフォルトのキーバインドが使用されます
- キーの重複設定がある場合、最後の設定が有効になります
- 無効なキーやアクション名を指定した場合、その行は無視されます
- 変更を反映するには、エディタを再起動してください

## デフォルトのキーバインド

デフォルトのキーバインドは `ym2151-tone-editor.toml.example` ファイルに記載されています。このファイルを参考にしてカスタマイズしてください。
