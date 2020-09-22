## Bitcoin address generator in Rust

Pretty fast bitcoin address generator in Rust, built as a PoC, **DO NOT USE FOR PRODUCTION**   


### Run me it:    

```shell
git clone http://github.com/flipchan/rust-bitcoin.git   
cd rust-bitcoin/   
cargo build --release  
./target/release/bitcoin   
```


```Rust
root@computer:/tmp/rustcoins/bitcoin# time ./target/release/bitcoin                          
Private key is: ee4f8e52a29db2df1ec0499abfad15b0be27df769d89763a7255c72920a8742e                     
buf is: 03fd70c6dd1890d0e1a3085f513f17112f008fabcd9c4148d608d672b4f7f3c693                           
first 4 bytes is: 030ddc08                                                                           
step8: 00e9701364ca822b3f9ca571fb7c3f9991c9d4ba16030ddc08                                            
step 9: 1NHJhrgYJEEq9BU8dyrvDr9eUkR7AbhHUj                                                           
                                                                                                     
real    0m0.005s                                                                                     
user    0m0.002s                                                                                     
sys     0m0.000s                                                                                     
```




Generates a bitcoin address based on the steps found here:    

https://en.bitcoin.it/wiki/Technical_background_of_version_1_Bitcoin_addresses


