names = ["神里绫人", "枫原万叶", "迪卢克", "嘉明", "白术", "重云", "雷泽", "魈", "菲米尼", "凯亚", "五郎", "班尼特", "莱欧斯利", "温迪", "阿贝多", "艾尔海森", "卡维", "流浪者", "提纳里", "托马", "荒泷一斗", "林尼", "鹿野院平藏", "赛诺", "行秋", "那维莱特", "米卡", "达达利亚", "钟离"]
n = len(names)
order = [[False for i in range(n)] for j in range(n)]
def cmp(a, b):
    if  order[a][b]:
        return 1
    elif order[b][a]:
        return 0
    else:
        ans = (input(f"choose: 0. {names[a]} 1. {names[b]}"))
        if ans == 0:
            order[a][b] = True
        else:
            order[b][a] = True
        for k in range(n):
            for i in range(n):
                for j in range(n):
                    order[i][j] = order[i][j] or (order[i][k] and order[k][j])
        return cmp(a, b)

def merge(arr, l, m, r): 
    n1 = m - l + 1
    n2 = r- m 
  
    # 创建临时数组
    L = [0] * (n1)
    R = [0] * (n2)
  
    # 拷贝数据到临时数组 arrays L[] 和 R[] 
    for i in range(0 , n1): 
        L[i] = arr[l + i] 
  
    for j in range(0 , n2): 
        R[j] = arr[m + 1 + j] 
  
    # 归并临时数组到 arr[l..r] 
    i = 0     # 初始化第一个子数组的索引
    j = 0     # 初始化第二个子数组的索引
    k = l     # 初始归并子数组的索引
  
    while i < n1 and j < n2 : 
        if cmp(L[i], R[j]) == 0: 
            arr[k] = L[i] 
            i += 1
        else: 
            arr[k] = R[j] 
            j += 1
        k += 1
  
    # 拷贝 L[] 的保留元素
    while i < n1: 
        arr[k] = L[i] 
        i += 1
        k += 1
  
    # 拷贝 R[] 的保留元素
    while j < n2: 
        arr[k] = R[j] 
        j += 1
        k += 1
  
def mergeSort(arr,l,r): 
    if l < r: 
        m = int((l+(r-1))/2)
        mergeSort(arr, l, m) 
        mergeSort(arr, m+1, r) 
        merge(arr, l, m, r)



arr = [i for i in range(n)]
mergeSort(arr,0,n-1)
ans='\n'.join([str(i+1)+'.'+names[i] for i in range(n)])
with open('zzh.txt', 'w') as f:
    f.write(ans)
print(ans)
