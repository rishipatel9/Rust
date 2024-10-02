use std::thread;
use std::sync::mpsc;
fn main() { 
    let handle=thread::spawn(||{
        for i in 0..5{
            println!("Spwan Thread no: {}",i);
        }
    });
    handle.join().unwrap();
    for i in 0..4{
        println!("Main Thread no: {}",i);
    }

    let v=vec![1,2,344];
    {
        let handle2 =thread::spawn(move ||{
            println!("hello from spawned thread with value : {v:?}");
        });
        handle2.join().unwrap();
    }
    let (tx,rx)=mpsc::channel();

    // thread::spawn(move||{
    //     tx.send(String::from("Hello from another thread")).unwrap();
    // });

    // let recived=rx.recv();
    // println!("{recived:?}")

    for i in 1..5{
        let producer=tx.clone();
        thread::spawn(move ||{
            let mut sum:u64=0;
            for j in  i * 10000000..(i+1 *10000000) -1 {
                sum=sum+j; 

            }
            producer.send(sum).unwrap();
        });
    }
    drop(tx);
    let mut ans:u64=0;
    for val in rx{
        println!("{val:?}");
        ans=ans+val;
    }
    println!("Ans : {ans:?}");

}
