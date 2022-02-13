## 线程安全的跳表

### 支持的操作
add（包括overwrite），get，delete
所有操作都是无锁的
delete 只是进行标记不进行回收, 这里参考了leveldb skiplist的实现，在memtable转为sstable时，删除的数据不会写入sstable，所以不需要额外进行gc
gc操作需要block所有其他的操作
add操作通过对node的next_node进行cas操作实现无锁

## 实现

### 结构

每一层是一个线程安全的list 
对于level>0,保存指向下层list的节点的引用
术语
index node:level >0 的node 用于加速查找
base node:level =0 的node 
