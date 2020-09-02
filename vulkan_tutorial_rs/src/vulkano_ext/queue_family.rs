use std::cmp::Ordering;
use vulkano::instance::QueueFamily;

pub struct QueueFamilyExt<'a>(QueueFamily<'a>);

impl<'a> QueueFamilyExt<'a> {
    pub fn new(inner: QueueFamily) -> QueueFamilyExt {
        QueueFamilyExt(inner)
    }

    pub fn inner(self) -> QueueFamily<'a> {
        self.0
    }
}

impl PartialEq for QueueFamilyExt<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl Eq for QueueFamilyExt<'_> {}

impl PartialOrd for QueueFamilyExt<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.id().partial_cmp(&other.0.id())
    }
}

impl Ord for QueueFamilyExt<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.id().cmp(&other.0.id())
    }
}
