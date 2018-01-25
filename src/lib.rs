extern crate git2;
extern crate tempdir;
#[macro_use]
extern crate log;

mod filelock;
mod scratch;
mod shell;
mod base_repo;
mod treeops;
mod view_subdir;
pub mod virtual_repo;

pub use filelock::FileLock;
pub use scratch::Scratch;
pub use scratch::view_ref;
pub use scratch::view_ref_root;
pub use shell::Shell;
pub use shell::thread_local_temp_dir;
pub use treeops::*;
pub use view_subdir::SubdirView;
pub use base_repo::BaseRepo;

#[derive(Clone)]
pub enum UnapplyView
{
    Done(git2::Oid),
    RejectNoFF,
    RejectMerge,
    NoChanges,
}

pub trait View
{
    fn apply(&self, tree: &git2::Tree) -> Option<git2::Oid>;
    fn unapply(&self,
               repo: &git2::Repository,
               tree: &git2::Tree,
               parent_tree: &git2::Tree)
        -> git2::Oid;
}
