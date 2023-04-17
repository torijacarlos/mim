use roxmltree::Node;

pub struct NodeManipulation;

impl NodeManipulation {
    #[inline]
    pub fn get_text_from_node(node: Option<Node>) -> String {
        if let Some(n) = node {
            if let Some(t) = n.text() {
                return t.to_string();
            }
        }
        String::new()
    }

    #[inline]
    pub fn get_attr_from_node(node: Option<Node>, attr: String) -> String {
        if let Some(n) = node {
            if let Some(a) = n.attribute(&attr[..]) {
                return a.to_string();
            }
        }
        String::new()
    }
}
