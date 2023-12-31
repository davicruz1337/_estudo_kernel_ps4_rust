use crate::fs::{DirentType, Vnode};
use gmtx::{Gutex, GutexGroup, GutexReadGuard, GutexWriteGuard};
use std::ops::Deref;
use std::sync::{Arc, Weak};
use std::time::SystemTime;

/// An implementation of `devfs_dirent` structure.
pub struct Dirent {
    inode: i32,                        // de_inode
    uid: Gutex<i32>,                   // de_uid
    gid: Gutex<i32>,                   // de_gid
    mode: Gutex<u16>,                  // de_mode
    dir: Option<Weak<Self>>,           // de_dir
    children: Gutex<Vec<Arc<Self>>>,   // de_dlist
    ctime: SystemTime,                 // de_ctime
    atime: Gutex<SystemTime>,          // de_atime
    mtime: Gutex<SystemTime>,          // de_mtime
    vnode: Gutex<Option<Weak<Vnode>>>, // de_vnode
    dirent: crate::fs::Dirent,         // de_dirent
}

impl Dirent {
    pub fn new<N>(
        ty: DirentType,
        inode: i32,
        uid: i32,
        gid: i32,
        mode: u16,
        dir: Option<Weak<Self>>,
        name: N,
    ) -> Self
    where
        N: Into<String>,
    {
        let gg = GutexGroup::new();
        let now = SystemTime::now();

        Self {
            inode,
            uid: gg.spawn(uid),
            gid: gg.spawn(gid),
            mode: gg.spawn(mode),
            dir,
            children: gg.spawn(Vec::new()),
            ctime: now,
            atime: gg.spawn(now),
            mtime: gg.spawn(now),
            vnode: gg.spawn(None),
            dirent: crate::fs::Dirent::new(ty, name),
        }
    }

    pub fn inode(&self) -> i32 {
        self.inode
    }

    pub fn uid(&self) -> GutexReadGuard<i32> {
        self.uid.read()
    }

    pub fn gid(&self) -> GutexReadGuard<i32> {
        self.gid.read()
    }

    pub fn mode(&self) -> GutexReadGuard<u16> {
        self.mode.read()
    }

    /// [`None`] represents self as a value.
    pub fn dir(&self) -> Option<&Weak<Self>> {
        self.dir.as_ref()
    }

    pub fn children_mut(&self) -> GutexWriteGuard<Vec<Arc<Self>>> {
        self.children.write()
    }

    pub fn vnode_mut(&self) -> GutexWriteGuard<Option<Weak<Vnode>>> {
        self.vnode.write()
    }

    /// See `devfs_find` on the PS4 for a reference.
    pub fn find<N: AsRef<str>>(&self, name: N, ty: Option<DirentType>) -> Option<Arc<Self>> {
        let name = name.as_ref();

        for child in self.children.read().deref() {
            // Check name.
            if child.dirent.name() != name {
                continue;
            }

            // Check type.
            if let Some(ty) = ty {
                if child.dirent.ty() != ty {
                    continue;
                }
            }

            return Some(child.clone());
        }

        None
    }

    /// See `devfs_parent_dirent` on the PS4 for a reference.
    pub fn parent(&self) -> Option<Arc<Self>> {
        let parent = if !self.is_directory() {
            self.dir.as_ref().unwrap().clone()
        } else if matches!(self.name(), "." | "..") {
            return None;
        } else {
            // Get de_dir from "..".
            let children = self.children.read();
            let dotdot = &children[1];

            dotdot.dir.as_ref().unwrap().clone()
        };

        parent.upgrade()
    }
}

impl Deref for Dirent {
    type Target = crate::fs::Dirent;

    fn deref(&self) -> &Self::Target {
        &self.dirent
    }
}
