use subprocess::Popen;
use bodo_connect::net::NetworkMap;

fn main() {
    let nm = NetworkMap::try_from("/home/topongo/.config/bodoConnect/networkmap.json".to_owned()).unwrap();
    println!("{:?}", nm);
    let dell = nm.get_host("dell").unwrap();
    println!("{:?}", dell);
    let local_subnet = nm.find_current_subnet();
    let hops = nm.hops_gen(dell, local_subnet).await;
    println!("{:?}", hops);
    // let proc = Popen::create(&[], config);
}
