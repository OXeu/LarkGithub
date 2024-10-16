# 飞书多维表格同步至 Github Issue
本项目可拉取飞书多维表格的数据并同步创建 Github Issue

## 特性
- [x] 支持创建/更新 Github Issue
- [x] 支持自定义标题内容模板
- [x] 支持自定义字段同步至标签
- [x] 支持自定义筛选条件
- [x] 支持字段为空时占位内容
- [x] 支持时间戳转可读的时间日期
- [x] 支持同步图片/文件

## 使用方法

### 1
创建一个飞书应用，添加以下权限：
- bitable:app 查看、评论、编辑和管理多维表格
- contact:user.base:readonly 获取用户基本信息

发布并审核

### 2
为需要同步的多维表格增加一个超链接字段，名称随意（如 Github Issue），但需要将名称填写到 `LARK_GITHUB_BIND_FIELD` 中。

### 3

复制 [lark-sync.yaml](./.github/workflows/lark-sync.yaml) 至需要同步的仓库

### 4

在仓库 > Settings > Secrets and variables > Action 中点击 New Repository Secret 添加以下 Secrets (逐条添加)
|Name|示例值|说明|
|---|---|---|
|LARK_APP_ID|cli_xxxxxxxxxx|飞书应用的 APP ID|
|LARK_APP_SECRET|R4ndOmsEcRetR4ndOmsEcRet|飞书应用的 APP Secret|

### 5
飞书多维表格的链接通常为以下形式：
```
https://bingyan.feishu.cn/base/VPe0b11IqALpzZsa8OjcC90qnth?table=tblXgSAuBu3tKMjD&view=vewMnpHsUz
```
其中的 `VPe0b11IqALpzZsa8OjcC90qnth` 为多维表格的 token, 保存到下列 `LARK_BITABLE_TOKEN` 中
`tblXgSAuBu3tKMjD` 为多维表格的某一个表的 id,保存到下列 `LARK_BITABLE_TABLE_ID` 中


在仓库 > Settings > Secrets and variables > Action > Variables 中点击 New Repository Variable 添加以下 Variable (逐条添加)
|Name|示例值|说明|
|---|---|---|
|LARK_GITHUB_BIND_FIELD|Github issue|多维表格中用于绑定 Github Issue链接的字段名|
|LARK_BITABLE_TOKEN|VPe0b11IqALpzZsa8OjcC90qnth|飞书多维表格的 token|
|LARK_BITABLE_TABLE_ID|tblXgSAuBu3tKMjD|飞书多维表格中所需同步表的 id|
|LARK_BITABLE_CONDITION_CONJUNCTION|or|多个筛选条件之间的条件关系可填 and 或 or 或不填|
|LARK_BITABLE_CONDITIONS|需求描述,isNotEmpty;需求详细描述（可附文档）,isNotEmpty|多个筛选条件用 ; 分隔，筛选条件内的多个内容用 , 分割，详见下文|
|ISSUE_UPDATE|true|是否在多维表格内容更新后同步更新 issue 内容|
|ISSUE_TITLE_FORMAT|{需求描述:未命名}|Issue 标题格式化模板，详见下文|
|ISSUE_LABEL_FIELDS|优先级,需求分类,需求状态|Issue Label来源字段，多个用 `,` 分隔|
|ISSUE_CONTENT_FORMAT|同标题格式，详细示例见下文|Issue 内容格式化模板，详见下文|

## 格式化模板
标题和内容使用模板来提高可自定义程度，以下是其详细规则：

### 插入指定字段内容
```
{字段名称}
```

### 插入时间&日期
```
{@日期的字段名}
```

### 当字段为空时的默认值
```
{字段名称:默认值}

时间也可以使用默认值
{@创建时间:未知时间}
```

### 特殊插值
```
记录创建时间
{created_at}

记录创建人
{created_by}

记录更新时间
{updated_at}

记录更新人
{updated_by}
```

### 示例
Issue 内容
```
## 需求描述
{需求描述:无}

## 详细描述
 {需求详细描述（可附文档）:无}

 {现象:}

--- 
于 **{@需求提出日期}** 创建
```
效果：

```markdown
## 需求描述
菜品信息补充

## 详细描述
 加入口让用户上传商家/菜品信息，经核实后后台上传

--- 
于 **2024/10/11 16:00** 创建
```
## 筛选字段
详见 https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-record/record-filter-guide
