---
title: "Dictionary"
weight: 1
# bookFlatSection: false
# bookToc: true
# bookHidden: false
# bookCollapseSection: false
# bookComments: false
# bookSearchExclude: false
---

# 数据字典

首先，我们为什么需要数据字典？

因为在数据库的日常使用中，可能会有大量的重复查询，比如一些固定的流程逻辑，或者配置表。
比如，中国的少数名族的名称、中国的省份名称。
对于这些较为固定的“配置”信息，我们可以使用数据字典的方式，进行存储。
这样可以避免不必要的查表操作，极大的加速数据获取的速度。

## 数据字典的创建模式

当前有两种创建方式：

1. 通过DDL进行创建
2. 通过加载配置文件（XML）创建


## 内置数据字典

在Clickhouse中，内置了一个Geo相关的一个字典格式，由于关联性不强。
我们此处忽略，感兴趣的朋友可以访问：
[Embedded Dictionaries](https://clickhouse.com/docs/en/sql-reference/dictionaries#embedded-dictionaries)
获取更多咨询。

## 外部扩展数据字典的内存布局（Layout）

官方推荐使用：`flat`, `hashed` 以及 `complex_key_hashed`来进行数据的存储。
这些布局都是全量加载到内存（RAM）中的，因此会避免可能低效的缓存操作。

其他的布局包括：

- `flat`: 内存存储，有长度限制的数组，以`UInt64`为Key。支持所有数据源，性能最佳。
- `hashed`: 内存存储，没有长度限制，以`UInt64`为key。支持所有数据源。
- `sparse_hashed`: 类似`hashed`，但是性能稍弱，会占用更多CPU。
- `complex_key_hashed`: 类似`hashed`，但是可使用复合Key。
- `complex_key_sparse_hashed`: 类似`shparse_hashed`，但是可使用复合Key。
- `hashed_arry`: 内存存储，优化过内存，类似`hashed`，速度比`sparse_hashed`要快，可媲美`hashed`，支持所有数据源。
- `complex_key_hashed_array`: 类似`hashed_array`，但是可使用复合Key。
- `range_hashed`: 带有起始和结束时间的`hashed`布局。
- `complex_key_range_hashed`: 类似`range_hashed`，但是可使用复合Key。
- `cache`: 提供缓存机制的布局类型，支持所有数据类型。同时支持过期时间等设置。
- `complex_key_cache`: 类似`cache`，但是可使用复合Key。
- `ssd_cache`: 类似`cache`，但是数据存储在SSD，索引存储在RAM（内存）。
- `complex_key_ssd_cache`: 类似`ssd_cache`，但是可使用复合Key。
- `direct`: 数据不存储在内存，而是在需要的时候从源数据加载（除了本地文件数据源）。
- `complex_key_direct`: 类似`direct`，但是可使用复合Key。
- `ip_trie`: 提供了对IP地址的数据支持。

## 数据源

Clickhouse提供了本地数据文件、执行程序、远程数据文件，以及数据库等多种支持。
其中：

- `file`: 本地文件。
- `executable`: 本地可执行文件结果。
- `executable_pool`: 本地可执行的Pool，提供持续的数据输入。
- `http`: 通过访问网络上的资源，进行数据获取。
- `odbc`: 接入ODBC驱动连接的数据库。
- `mysql`: 接入MySQL类型的数据库。
- `clickhouse`: 接入Clickhouse类型的数据库。
- `mongodb`: 接入MongoDB类型的数据库。
- `redis`: 接入Redis类型的数据库。
- `cassandra`: 接入Cassadra类型的数据库。
- `postgresql`: 接入PostgreSQL类型的数据库。

除上面几种外，Clickhouse还提供了一种特数格式的数据源：`Null`，此数据源主要用于构建一个空的数据字典，用于测试等目的。

