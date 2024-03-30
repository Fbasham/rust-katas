DoublyLinkedList.prototype.remove = function(i){
    if (i>=this.length || i<0) return false
    let c = this.head
    for (let k=0;k<i;k++) c = c.next
    if (c.prev) c.prev.next = c.next
    else this.head = c.next
    if (c.next) c.next.prev = c.prev
    else this.tail = c.prev
    this.length--
    return true
  }