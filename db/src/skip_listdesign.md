## 线程安全的跳表

### 支持的操作
add（包括overwrite），get，delete
所有操作都是无锁的
delete 只是进行标记不进行回收，达到一定数量后由gc进行回收
gc操作需要block所有其他的操作
add操作通过对node的next_node进行cas操作实现无锁

## 实现

### 结构

每一层是一个线程安全的list 
对于level>0,保存指向下层list的节点的引用
术语
index node:level >0 的node 用于加速查找
base node:level =0 的node 

## remove

如何删除 只是对 base list，进行删除mark，上层节点不需要处理，后续由gc完成回收, 因为list删除后增加的node会放在最后一个delete
node，所以结构是这样的 某个key第一次增加到list

```
               ┌──────────┐
               │          │
               │  index   │
               │          │
               │          │
               └─────┬────┘
                     │
                     │
                     │
                     │
               ┌─────▼────┐
               │          │
               │          │
       base    │   alived │
               │          │
               └──────────┘
```

多次删除增加同一个key

                  ┌───────┐
                  │ index │
                  │       │
                  └───┬───┘
                      │
                  ┌───▼───┐  ┌─────────┐   ┌──────────┐
                  │deleted│  │ deleted │   │          │
       base_level │       ├──►         ├───┤►alived   │
                  └───────┘  └─────────┘   └──────────┘

如何gc 和list一样用一个全局锁锁住整个skip list 从最高层开始，发现某个节点删除后，把该节点从该层中删除,第n层遍历结束对n-1层重复上述过程

如何增加level add增加了节点后，list返回新增节点的指针，从低层到高层逐个增加新节点

何时gc remove计数达到阈值新启动
