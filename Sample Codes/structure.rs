struct character {
		name : String,
		life : u32
}
fn main(){
	impl character{
		fn new(name : String) -> Character {
			return Character{ name: name , life:100};
		}
		fn get_life(&self) -> u8 {
			return self.life;
		}
	}
	//let c : Character = character
}

// Resolve this code issue with declaration of structure


