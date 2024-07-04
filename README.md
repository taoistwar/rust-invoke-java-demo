# README

本样例主要是为了在 Rust 中调用 JAVA，缘起是有些旧的 JAVA 系统没有 RUST SDK（Hive）。

## 编译 java 类

```bash
cd java
javac com\taoistwar\jni\PrintLibraryPath.java
jar cfm PrintLibraryPath.jar com\taoistwar\jni\PrintLibraryPath.class
```
