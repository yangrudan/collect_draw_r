# 收集堆栈合并与绘制火焰图
[![Rust](https://github.com/yangrudan/collect_draw_r/actions/workflows/rust.yml/badge.svg)](https://github.com/yangrudan/collect_draw_r/actions/workflows/rust.yml)

## 0x01 介绍

>堆栈合并（Stack Merging）是一种技术，用于将多个调用堆栈中的信息汇总到一起，以便于分析和优化。
>
>火焰图则是一种图形化工具，通过可视化的方式展示函数调用的层次和时间分布，帮助开发者快速定位性能瓶颈。

### 1. 收集数据
**前置条件**: 确保已经启用了性能分析工具Probing网页服务，并且已经生成了性能分析数据.

### 2. 堆栈合并
收集多个rank堆栈信息并进行合并.

### 3. 火焰图绘制
定制化火焰图生成.

## 0x02 使用方式

准备工作(提供堆栈)
```bash
❯ cd /home/yang/worksapce/demangle_l                                                                                                                      
❯ source /home/yang/worksapce/probing/venv/bin/activate
❯ PROBING=1 PROBING_PORT=9922 python main.py     
```

**修改urls.json**

```
❯ cd /home/yang/worksapce/collect_draw_r                                                                                                                      
❯ cargo run
```

## 0x03 性能优化

本项目已针对 10000+ URL 请求进行了性能优化:

### 优化特性

1. **高并发处理**
   - 批处理大小: 100 URLs/批次 (原 10)
   - 最大并发数: 200 个并发请求
   - 连接池: 50 个空闲连接/主机 (原 10)

2. **内存优化**
   - 使用 `BufWriter` 进行缓冲写入
   - 预分配向量容量避免重复内存分配
   - 流式处理减少内存占用

3. **用户体验**
   - 实时进度显示 (每批次更新)
   - 显示处理速度和总耗时
   - 清晰的成功/失败反馈

### 性能提升

- ✓ **10倍吞吐量**: 批处理大小从 10 增加到 100
- ✓ **5倍连接复用**: 连接池从 10 增加到 50
- ✓ **20倍并行度**: 从 10 并发增加到 200 并发
- ✓ **减少内存分配**: 缓冲写入和预分配向量

## 0x04 运行结果

![mmm.svg](./output/flamegraph.svg)

![mmm222.svg](./output/flamegraph222.svg)

## 0x05 Output文件说明

- `urls.json` 为模拟的请求;
- `response.json` 为单进程收集数据;
- `output.json` 为模拟4进程收集的数据;
- `processed_stacks.txt` 为json转换成一维txt;
- `merged_stacks.txt` 为合并后的堆栈信息;
- `mmm.svg` 测试堆栈合并火焰图;
- `flamegraph.svg` 为生成的火焰图;
