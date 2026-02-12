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
   - 最大并发数: 500 个并发请求 (原 10)
   - 连接池: 500 个空闲连接/主机 (原 10，针对单主机优化)
   - 真正的并发处理: 所有请求同时启动，由信号量控制并发数
   - TCP Keep-Alive: 保持连接活跃，减少握手开销
   - HTTP/2 Keep-Alive: 优化长连接性能

2. **内存优化**
   - 使用 `BufWriter` 进行缓冲写入
   - 预分配向量容量避免重复内存分配
   - 流式处理减少内存占用

3. **用户体验**
   - 实时进度报告 (每2秒更新)
   - 显示处理速度、总耗时和平均每URL耗时
   - 清晰的成功/失败反馈

### 性能提升

- ✓ **50倍并行度**: 从 10 并发增加到 500 并发
- ✓ **50倍连接复用**: 连接池从 10 增加到 500 (单主机优化)
- ✓ **真正的并发**: 移除批处理等待，实现连续并发处理
- ✓ **连接优化**: TCP和HTTP/2 Keep-Alive保持连接活跃
- ✓ **减少内存分配**: 缓冲写入和预分配向量

### 性能说明

**注意**: 实际性能受服务器处理能力限制。如果所有请求指向同一服务器，实际吞吐量取决于：
- 服务器的处理能力和并发限制
- 网络带宽和延迟
- 服务器端的速率限制策略

客户端优化可确保不会成为瓶颈，但无法突破服务器端的物理限制。

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
