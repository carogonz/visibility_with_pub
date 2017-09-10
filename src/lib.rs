
mod victor {
    
    pub fn in_public() {}

    pub fn is_stoic() {
    	baby_yap::his_favorite();
    	
    }


    pub mod baby_yap{ 

   		pub fn his_favorite() {}
   	
   		fn likes_raccoons () {}
   }
}

fn test() {
	victor::in_public();
	victor::baby_yap::his_favorite(); 
	//victor::baby_yap::likes_raccoons(); can't access something private 


}