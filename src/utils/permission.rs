/// 权限裁决
///
/// 函数中使用`map_or`函数，account_permission为默认值,不为`None`则执行函数`|permission| {}`。
pub(crate) fn determine_permission(account_permission: i32, data_permission: Option<i32>) -> i32 {
    data_permission.map_or(account_permission, |permission| {
        if account_permission > permission {
            account_permission
        } else {
            permission
        }
    })
}
