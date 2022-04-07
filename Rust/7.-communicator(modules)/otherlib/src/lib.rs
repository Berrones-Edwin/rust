#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

// use outermost;

mod outermost {

    pub fn middle_function(){}

    fn middle_secret_function(){}

    mod inside {
        pub fn inner_function(){
            // ::outermost::middle_secret_function();
        }
        fn secret_function(){}
    }
}

fn try_me(){

//     outermost::middle_function(); //✔
//     outermost::middle_secret_function();
//     outermost::inside::inner_function();
//     outermost::inside::secret_function();
 }