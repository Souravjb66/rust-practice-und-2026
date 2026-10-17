
#[derive(Debug)]
enum Car{
    price(i32),
    miledge(i32),
    brand(String),
    drive {i:i32,v:i32},
    
}


fn main() {
    println!("Hello, world!");
    let a = 22;
    let b = "hhh";
    let c: i32=21;
    println!("{} and {} , {}",a,b,c);
    let bb:bool=true;
    println!("dog is a animal ,{}",bb);
    let cc:char='c';
    println!("{}  char is ",cc);
    let nm:&str="hello ji";
    println!("title is {}",nm);
    add(5,5);
    let vv:String=String::from("pro");
    call(vv.clone());
    println!("cc {}",vv);
    let mut vc=vec![1,2,3];
    let vc2=&vc;
    println!("{:?}",vc2);
     println!("{:?}",vc);
     let first=&vc[0];
     println!("first index {}",first);
     vc.push(7);
     
    println!("new first ind {:?}",vc);
    longest_stirng(&mut String::from("fff hdhc hhdch hhd h"));
    let nevv=vec![1,4,5];
    let nn1=&nevv;
    let nn2=&nevv;
    println!("{:?} {:?}",nn1,nn2);
    let cname=Car::brand("bmw".to_string());
    println!("car name is {:?}",match cname {
        Car::brand(name)=>name,
        _ => "none".to_string(),
    });
    println!("drive {:?}",Car::drive { i: 34, v: 33 });

    let all= |x|x*2;
    println!("{}",all(6));
    
   
    
}

fn add(a:i8,b:i8){
    println!("{}",a+b);
}
fn call(s:String){
    println!("value is {}",s)
}
fn longest_stirng(words:&mut str){
    println!("{:?}",words.split_whitespace());
    
}
