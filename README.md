# Leetcode
如果可以的话，就坚持下去吧

以下内容仅为自己的一些随笔感想，并非题解
## 1.两数之和
这题没有什么要说的，其实就是用空间换时间

## 2.两数相加
关于这题，我最开始想的是一种很笨的方法，就是我把链表先倒过来转成数组，然后再把两个数组相加，然后再把这个数组转成链表，后来呢尝试了一下，发现太复杂了， 不过这个思路应该还是对的，不过确实太麻烦了。
后来沉下心想了一下，发现其实每次都是只要考虑当前位置的就行了，后面的对与当前的节点只是Next，自然也就可以用递归了。
这是自己第一次用递归解出来链表类的问题，所以还是挺开心的。继续坚持

## 3.无重复字符的最长子串
关于这题，其实最开始看到就是有思路的，但是后面真写的时候发现还是有些地方没有考虑到的
1. 关于`right`应该在满足什么条件下右移，这个卡了一下。最开始我还想用一个切片还存储当前满足条件的子串，后来发现是多么的愚蠢，因为这样的话我每次去判断在当前子串是否有`string[right]`以及对于后续的处理都会非常的麻烦
2. 关于`left`应该什么情况下右移，这里卡了比较长的时间。我最开始并没有考虑到需要判断当前`map`中`string[right]`的index和`left`的情况，只是想着如果有的map中有`string[right]`就直接把`left`挪到`index`就行了，这样实际会有可能把`left`往左移的，所以还是需要判断`index`和`left`的情况的；另外就是`left`和`index`判断的时候，最开始是`left=index`的，也是没有考虑完全，其实是需要`left=index+1`的。

## 4.寻找两个正序数组的中位数
这题在看到时间复杂度要求是O(log(m+n))的时候，是想过用二分法的，但是后续就没有思路了。
关于题解中给出的方法，这里说一下自己的理解过程，所谓中位数就是找到合并后的数组的第K个数，但是又不能真的合并了之后再去找，所以题目中的方法是从AB两个数组中每次删除K/2个数，同时需要比较AB数组中哪个更小，因为小的那个肯定是不满足条件的，但是大的那个数组就可能删过了。