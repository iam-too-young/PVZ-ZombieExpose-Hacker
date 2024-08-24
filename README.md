###依赖导入
```
[dependencies]
winapi = { version = "0.3", features = ["winuser", "memoryapi", "processthreadsapi", "handleapi", "errhandlingapi", "psapi", "winbase", "windef"] }
```

###实现思路
建立一个透明且可以穿透点击的窗口，将该窗口始终置顶，并持续移动到游戏窗口，获取DC，使用内存上下文DC_mem，创建一个“绘图区域”，在其中先清除所有框，再对每个僵尸的坐标绘制，最后统一拷贝到DC

###注意事项
本程序没有对高DPI做处理，如果你的电脑有高DPI设置，
请将该程序的高DPI设置设置为“应用程序”
否则绘图会偏移
