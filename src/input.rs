pub struct Input {
    action:String,
    todo:String,
}

impl Input {
    pub fn build(mut args:impl Iterator<Item = String>)->Result<Input, &'static str>{
        args.next();

        let action = match args.next() {
            Some(a) => a,
            None => return Err("Action is not provided")
        };

        let todo = match args.next() {
            Some(a) => a,
            None => {
                if(action == "list"){
                     String::from("")
                }
                else{
                    return Err("Todo Not provided");
                }
            }
        };

        Ok(Input {
            action,
            todo
        })
    }


    pub fn get_action(&self)->&str{
        &self.action
    }

    pub fn get_todo(&self)->&str{
        &self.todo
    }
}
