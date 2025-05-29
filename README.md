# curld

一个简单的命令行工具，用于将本地目录中的文件上传到指定的 HTTP 服务器。

## 安装

```bash
cargo install --path .
```

## 使用方法

```bash
curld -d <本地目录路径> <目标URL>
```

例如：

```bash
curld -d ./my-files http://192.168.3.250:7799/new-path/path-to-file
```

## 功能

- 递归遍历指定目录中的所有文件
- 保持目录结构上传文件
- 显示上传进度和结果
- 支持异步操作，提高上传效率
