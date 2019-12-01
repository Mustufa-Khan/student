// fn main(){

//     let numbers=vec![77,88,98,4,15,6,77,8];

//     let mut largest=numbers[0];
    

//     for &i in numbers.iter(){

//         if i>largest{

//             largest=i;
//         }
//     }

//  println!("{}",largest);

// }







// fn largest(list: &[i32]) -> i32{

//     let mut largest=list[0];
    
//     for &i in list.iter(){
//         if i>largest
//         {largest=i;}
//     }

//     largest
// }


// fn main(){
//      let numbers = vec![12,45,89,97,1055,33];

//      let result = largest(&numbers);

//      println!("{}",result);
// }













// #[derive(Debug)]
// struct Point <T,U>{

//     x:T,
//     y:U,
// }


// fn main(){


//    let p1=Point{ 

//         x:10,
//         y:20.00,
//     };

//     println!("{:?}",p1);
// }












//#[derive(Debug)]
// struct Point <T,U>{

//     x:T,
//     y:U,
// }


// fn main(){


//    let p1=Point{ x:10, y:'m'};
//    let p2=Point{x:20.00, y:'s'};
//    let p3=p2.mixup(p1);



//     println!("{:?}",p3);
// }


// impl <T,U> Point <T,U>{

//     fn mixup<V,W>(&self, other: Point<V,W>) -> Point<&T,W>{

//         Point{
        
//         x: &self.x,
//         y: other.y,
//         }
//     }
// }





// #[derive(Debug)]
//  enum optioni32{

//     some(i32),
//     none(),
//  }

//  #[derive(Debug)]
//  enum optionf64{

//     some(f64),
//     none(),
//  }


//  fn main(){

//     let p1=optioni32::some(10);
//     let p2=optionf64::some(20.0);

//     println!("{:?}, {:?}",p1,p2);
//  }






// pub trait Summary{

//     fn summarize(&self) -> String;
// }

// #[derive(Debug)]
// struct Newsarticle{ 
//     headline:String,
//     location:String,
//     author:String,
//     content:String,
// }

// impl Summary for Newsarticle{

//     fn summarize(&self) -> String{

//         format!("{} {} {}",self.headline,self.location,self.author)
//     }
// }


// #[derive(Debug)]
// struct Tweet {
//     username:String,
//     content:String,
//     reply:bool,
// }

// impl Summary for Tweet{

//     fn summarize(&self) -> String{

//         format!("{} {}",self.username,self.content)
//     }   
// }
// fn main(){

// let tweet1=Tweet{
//     username:String::from("Mustufa Khan"),
//     content:String::from("Ofcourse You Know It!"),
//     reply:false,
// };

// print!("{:?}",tweet1.summarize());

// }










// #[derive(Debug)]
// struct NewsArticle{

//     author:String,
//     content:String,
// }

// #[derive(Debug)]
// struct Tweet{

//     username:String,
//     content:String,
// }


// pub trait Summary{

// fn summarize(&self) -> String;
    
// }


// impl Summary for NewsArticle{

//     fn summarize(&self) -> String{

//         format!("{}, {} ",self.author,self.content)
//     }
// }


// impl Summary for Tweet{

//     fn summarize(&self) -> String {

//         format!("{}, {}",self.username,self.content)
//     }
// }



// fn main(){

//     let NewsArticle1=NewsArticle{

//         author:String::from("SUMMERA"),
//         content:String::from("IOT REVOLUTION"),
//     };

//     let Tweeting1=Tweet{

//         username:String::from("SUMMERA31"),
//         content:String::from("4IR INNOVATION"),
//     };


//     println!("{:?}",NewsArticle1.summarize());

//     println!("{:?}",Tweeting1.summarize());


// }


// 









// #[derive(Debug)]
// struct Human{

//     name:String,
//     id:i32,

// }

// pub trait information{
 
//     fn info(&self) ->String;

// }

// impl information for Human{

//     fn info(&self) ->String{

//         let n=format! ("{}",self.name);
//         n
//     }
// }

// impl Human{

//     fn new(name:String, id:i32) -> Human{
//         Human{
    
//             name,
//             id,
//         }
//     }
    
//     // fn output(&self){
//     //     println!("{:?} {:?}",self.name,self.id);
//     // }

   
// }    




// fn main(){

   

//     // let human1=Human::new("Muastufa".to_string(),10);

//   let _human1=Human::new( "Mustufa Khan".to_string(), 10 );


    

//     println!("{:?}",_human1.info());
// }











#[derive(Debug)]
struct Human {
    name:String,
    id:i32,
}


impl Human{
    fn getdata(name:String, id:i32) -> Human{
   
      Human{
       name,
       id, 
     }
    }

 fn show(&self) {
     
     println!("Name is: {}\nId is: {}\n",self.name,self.id);
     
 }

}

fn main(){

    let human1 = Human::getdata("Mustufa Khan".to_string(),10); 
    let human2 =Human::getdata("Muneeba Younus".to_string(),20);
    human1.show();
   human2.show();
}



















