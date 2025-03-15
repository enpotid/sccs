mod sctype;

fn main() {
    let a = sctype::scuint::ScUInt::from("57923847019283740192837401928374019283740192837401928")
        .unwrap();
    let b =
        sctype::scuint::ScUInt::from("8472093847209384702938470293847209384720938472093847209384")
            .unwrap();
    let c = sctype::scuint::ScUInt::from(
        "129384710928374019283740192837401928374019283740192837401928374019",
    )
    .unwrap();
    let d = sctype::scuint::ScUInt::from(
        "98723984720398472093847209384720938472093847209384720938472093847209384",
    )
    .unwrap();
    let e = sctype::scuint::ScUInt::from(
        "748201938470129384701923847019283740192837401928374019283740192837401928374",
    )
    .unwrap();
    println!("{}", a + b + c + d + e);
}
