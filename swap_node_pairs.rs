use preloaded::Node;

fn swap_node_pairs(head: Option<Box<Node>>) -> Option<Box<Node>> {
    let mut dummy = Node { next: head };
    let mut current = &mut dummy;
    while current.next.is_some() && current.next.as_ref().unwrap().next.is_some() {
        let mut first = current.next.take().unwrap();
        let mut second = first.next.take().unwrap();
        let next_pair = second.next.take();
        first.next = next_pair;
        second.next = Some(first);
        current.next = Some(second);
        current = current.next.as_mut().unwrap().next.as_mut().unwrap();
    }
    dummy.next
}