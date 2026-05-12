/// 可禁用的组件特征
pub trait Disableable {
    /// 设置组件是否禁用
    fn disabled(self, disabled: bool) -> Self;
}
