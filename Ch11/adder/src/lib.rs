#[cfg(test)]
mod tests {
	#[derive(Debug)]
	struct Rectangle {
	    width: u32,
	    height: u32,
	}

	impl Rectangle {
	    fn can_hold(&self, other: &Rectangle) -> bool {
	        self.width > other.width && self.height > other.height
	    }
	}

    #[test]
    fn it_woks() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn rect(){
    	let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };
    	assert!(!smaller.can_hold(&larger));
    }
}
