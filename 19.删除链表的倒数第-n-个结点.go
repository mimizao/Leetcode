/*
 * @lc app=leetcode.cn id=19 lang=golang
 *
 * [19] 删除链表的倒数第 N 个结点
 */
package main

import "fmt"

type ListNode struct {
	Val  int
	Next *ListNode
}

// @lc code=start
/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
func removeNthFromEnd(head *ListNode, n int) *ListNode {
	listNodeLen := 1
	for currentListNode := head; currentListNode.Next != nil; currentListNode = currentListNode.Next {
		listNodeLen++
	}
	fmt.Println("listNodeLen: ", listNodeLen)
	if listNodeLen < n {
		return head
	}
	if listNodeLen == n {
		return head.Next
	}
	currentListNode := head
	for index := 0; index < listNodeLen-n; index++ {
		currentListNode = currentListNode.Next
		fmt.Println("i:,val:", index, currentListNode.Val)
	}
	tempListNode := currentListNode.Next
	currentListNode.Next = tempListNode.Next
	return head
}

// @lc code=end
