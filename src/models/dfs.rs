#[derive(PartialEq, Debug)]
pub struct DFS {
    pub from: usize,
    pub to: usize,
    pub from_label: isize,
    pub e_label: usize,
    pub to_label: isize,
}

impl DFS {
    pub fn new() -> DFS {
        DFS {
            from: 0,
            to: 0,
            from_label: 0,
            e_label: 0,
            to_label: 0,
        }
    }

    pub fn from(from: usize, to: usize, from_label: isize, e_label: usize, to_label: isize) -> DFS {
        DFS {
            from,
            to,
            from_label,
            e_label,
            to_label,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equal_dfs() {
        let dfs1 = DFS::from(1, 2, 3, 4, 5);
        let dfs2 = DFS::from(1, 2, 3, 4, 5);
        let dfs3 = DFS::from(2, 2, 3, 4, 5);

        assert_eq!(dfs1, dfs2);
        assert_ne!(dfs1, dfs3);
        assert_ne!(dfs2, dfs3);
    }
}
