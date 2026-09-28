#[derive(Clone,Debug,PartialEq,Eq)] pub struct Event{pub id:String,pub activity:String,pub objects:Vec<String>} impl Event{pub fn bound(&self)->bool{!self.id.is_empty()&&!self.objects.is_empty()}}
