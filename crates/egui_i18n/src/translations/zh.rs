/// Simplified Chinese translations for the UI.
///
/// Contains translations for:
/// - Demo names
/// - Common UI elements
/// - Actions and settings
pub fn get_translations() -> std::collections::HashMap<&'static str, &'static str> {
    let mut trans = std::collections::HashMap::new();

    // 菜单栏
    trans.insert("egui demo app", "egui 演示应用");
    trans.insert("🌏 Language", "🌏 语言");
    trans.insert("💻 Backend", "💻 后台");
    trans.insert("✨ Demos", "✨ 演示");
    trans.insert("🖹 EasyMark editor", "🖹 EasyMark 编辑器");
    trans.insert("🕑 Fractal Clock", "🕑 分形时钟");
    trans.insert("🔺 3D painting", "🔺 3D 绘画");
    trans.insert("🎨 Rendering test", "🎨 渲染测试");

    // Demo 名称
    trans.insert("About egui", "关于 egui");

    // 通用 UI 文本
    trans.insert("File", "文件");
    trans.insert("Edit", "编辑");
    trans.insert("View", "视图");
    trans.insert("Help", "帮助");
    trans.insert("Settings", "设置");
    trans.insert("Theme", "主题");
    trans.insert("Open", "打开");
    trans.insert("Save", "保存");
    trans.insert("Cancel", "取消");
    trans.insert("OK", "确定");
    trans.insert("Reset", "重置");

    trans
}
