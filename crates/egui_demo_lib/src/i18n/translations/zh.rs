pub fn get_chinese_translations() -> std::collections::HashMap<&'static str, &'static str> {
    let mut trans = std::collections::HashMap::new();

    // Demo 名称
    trans.insert("About egui", "关于 egui");
    trans.insert("☰ Context Menus", "☰ 上下文菜单");
    trans.insert("♫ Dancing Strings", "♫ 跳动字符串");
    trans.insert("🗖 Extra Viewport", "🗖 额外视图");
    trans.insert("🔤 Font Book", "🔤 字体册");
    trans.insert("✨ Highlighting", "✨ 高亮显示");
    trans.insert("🖮 Code Editor", "🖮 代码编辑器");
    trans.insert("✋ Drag and Drop", "✋ 拖放演示");
    trans.insert("🔍 Pan Zoom", "🔍 平移缩放");

    // 通用 UI 文本
    trans.insert("File", "文件");
    trans.insert("Edit", "编辑");
    trans.insert("View", "视图");
    trans.insert("Help", "帮助");
    trans.insert("Settings", "设置");
    trans.insert("Language", "语言");
    trans.insert("Theme", "主题");
    trans.insert("Open", "打开");
    trans.insert("Save", "保存");
    trans.insert("Cancel", "取消");
    trans.insert("OK", "确定");
    trans.insert("Reset", "重置");

    trans
}
