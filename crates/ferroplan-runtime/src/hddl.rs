#[derive(Clone, Debug)]
pub struct Task {
    pub id: String,
    pub children: Vec<Task>,
}
impl Task {
    pub fn leaves<'a>(&'a self, out: &mut Vec<&'a str>) {
        if self.children.is_empty() {
            out.push(&self.id)
        } else {
            for c in &self.children {
                c.leaves(out)
            }
        }
    }
}
