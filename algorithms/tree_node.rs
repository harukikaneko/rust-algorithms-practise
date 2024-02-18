#[derive(Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

pub fn binary_tree(p: Option<Box<TreeNode>>, q: Option<Box<TreeNode>>) -> bool {
    match (p, q) {
        (None, None) => true,
        (Some(p), Some(q)) => {
            p.val == q.val && binary_tree(p.left, q.left) && binary_tree(p.right, q.right)
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_tree_case_equal_tree() {
        let p = TreeNode {
            val: 1,
            left: Some(Box::new(TreeNode {
                val: 2,
                left: Some(Box::new(TreeNode { val: 3, left: None, right: None })),
                right: Some(Box::new(TreeNode { val: 4, left: None, right: None })),
            })),
            right: Some(Box::new(TreeNode {
                val: 5,
                left: Some(Box::new(TreeNode { val: 6, left: None, right: None })),
                right: Some(Box::new(TreeNode { val: 7, left: None, right: None })),
            })),
        };
        let q = TreeNode {
            val: 1,
            left: Some(Box::new(TreeNode {
                val: 2,
                left: Some(Box::new(TreeNode { val: 3, left: None, right: None })),
                right: Some(Box::new(TreeNode { val: 4, left: None, right: None })),
            })),
            right: Some(Box::new(TreeNode {
                val: 5,
                left: Some(Box::new(TreeNode { val: 6, left: None, right: None })),
                right: Some(Box::new(TreeNode { val: 7, left: None, right: None })),
            })),
        };
        assert_eq!(binary_tree(Some(Box::new(p)), Some(Box::new(q))), true);
    }

    #[test]
    fn test_binary_tree_case_not_equal_tree() {
        let p = TreeNode {
            val: 1,
            left: Some(Box::new(TreeNode {
                val: 2,
                left: Some(Box::new(TreeNode { val: 3, left: None, right: None })),
                right: Some(Box::new(TreeNode { val: 4, left: None, right: None })),
            })),
            right: Some(Box::new(TreeNode {
                val: 5,
                left: Some(Box::new(TreeNode { val: 6, left: None, right: None })),
                right: Some(Box::new(TreeNode { val: 7, left: None, right: None })),
            })),
        };
        let q = TreeNode {
            val: 1,
            left: Some(Box::new(TreeNode {
                val: 2,
                left: Some(Box::new(TreeNode { val: 2, left: None, right: None })),
                right: Some(Box::new(TreeNode { val: 4, left: None, right: None })),
            })),
            right: Some(Box::new(TreeNode {
                val: 5,
                left: Some(Box::new(TreeNode { val: 6, left: None, right: None })),
                right: Some(Box::new(TreeNode { val: 7, left: None, right: None })),
            })),
        };
        assert_eq!(binary_tree(Some(Box::new(p)), Some(Box::new(q))), false);
    }
}
